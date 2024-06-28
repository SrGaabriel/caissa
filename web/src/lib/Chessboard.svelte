<script lang="ts">
	// Generate the chessboard cells data
	import { BoardLogic, Highlighting, type Piece, Team } from '$lib';
	import { crossfade } from 'svelte/transition';
	import { getMoveSound } from '$lib/sound/sounds';
	import { linear } from 'svelte/easing';
	import type { Player } from '$lib/game/logic';
	import {fetchPieceMoves} from "$lib/api/moves";

	type Cell = {
		x: number,
		y: number,
		highlighting: Highlighting | null,
	}

	type CursorPosition = {
		x: number,
		y: number,
	}

	const [send, receive] = crossfade({
		duration: 250,
		easing: linear,
	});

	export let board: BoardLogic;
	export let opponent: Player | null;
	export let locked: boolean = false;
	let cells: Cell[] = [];
	let selectedCell: Cell | null = null;
	let draggingCell: Cell | null = null;
	let dragTimeout: number | null = null;
	const isScreenMirrored: boolean = false;
	$: teamToPlay = board.state.teamToPlay;
	$: ending = board.state.ending;
	drawBoard(Team.White);

	function drawBoard(team: Team) {
		function pushCell(col: number, row: number) {
			cells.push({
				x: col,
				y: row,
				highlighting: null
			});
		}

		if (team === Team.Black) {
			for (let row = 1; row <= 8; row++) { // Y
				for (let col = 8; col >= 1; col--) { // X
					pushCell(col, row);
				}
			}
		} else {
			for (let row = 8; row >= 1; row--) { // Y
				for (let col = 1; col <= 8; col++) { // X
					pushCell(col, row);
				}
			}
		}
	}

	function findCell(x: number, y: number): Cell | undefined {
		return cells.find(cell => cell.x === x && cell.y === y);
	}

	function highlightCell(x: number, y: number, marker: Highlighting) {
		const cell = findCell(x, y);
		if (cell) {
			cell.highlighting = marker;
			cells = cells;
		}
	}

	function updatePage() {
		board = board;
		clearHighlights(true);
	}

	function handleSquareClick(event: Event) {
		if (locked || ending) return;
		const element = event.target as HTMLElement;
		if (!element.dataset.xpos || !element.dataset.ypos) {
			console.error(`Element inside cell not identified: ${element.nodeName}`);
			return;
		}
		const x: number = +element.dataset.xpos;
		const y: number = +element.dataset.ypos;
		if (selectedCell && x == selectedCell.x && y == selectedCell.y) {
			clearHighlights();
			selectedCell = null;
			return;
		}

		const cell = findCell(x, y);
		if (!cell) return;
		const piece = board.getPieceAt(x,y);
		if (selectedCell && board.getPieceAt(selectedCell.x, selectedCell.y)) { // If the player is trying to click on another cell while he already has one selected...
			const selectedPiece = board.getPieceAt(selectedCell.x, selectedCell.y)!;
			if (piece && piece.team == selectedPiece.team) { // If the player is trying to switch attacking pieces...
				selectPiece(cell);
				if (piece.team === teamToPlay) {
					highlightPossibleMoves(x, y);
				}
				return;
			}
			if (!isTeamAllowedToMove(selectedPiece.team)) { // If the player is trying to move outside his turn...
				clearHighlights(false);
				cell.highlighting = Highlighting.SELECTED; // We just switch his apparently selected pieces
				cells = cells;
				return;
			} // Otherwise, we move
			playMove(selectedCell.x, selectedCell.y, x, y).then((move) => {
				if (!move) {
					clearHighlights(false);
					selectPiece(cell);
					highlightPossibleMoves(x, y);
					return;
				}
				updatePage();
			});
			updatePage();
			selectedCell = null;
			clearHighlights();
			return;
		}
		// If the player is trying to select a piece...
		if (!piece) {
			clearHighlights(true);
		} else if (isTeamAllowedToMove(piece.team)) {
			selectPiece(cell);
			highlightPossibleMoves(x, y);
		} else {
			clearHighlights(false);
			markCell(cell, Highlighting.SELECTED);
		}
	}

	function calculateCursorOffset(event: MouseEvent, chessboard: HTMLElement): CursorPosition {
		const chessboardRect = chessboard.getBoundingClientRect();
		const deltaX = event.clientX - chessboardRect.left
		const deltaY = event.clientY - chessboardRect.top;
		return {
			x: deltaX,
			y: deltaY
		}
	}

	function followCursor(chessboard: HTMLElement, asset: HTMLElement, cursor: CursorPosition) {
		asset.style.position = 'absolute';
		asset.style.left = (cursor.x - asset.offsetWidth / 2) + 'px';
		asset.style.top = (cursor.y - asset.offsetHeight / 2) + 'px';
	}

	function getCellUnderCursor(cursor: CursorPosition, team: Team): Cell | undefined {
		const x = Math.max(1, Math.min(Math.floor(cursor.x / 100) + 1, 8));
		const blackPerspectiveY = Math.max(1, Math.min(Math.floor(cursor.y / 100) + 1, 8));
		const whitePerspectiveY = Math.abs(8 - blackPerspectiveY) + 1;
		const y = isScreenMirrored ? (team === Team.Black ? blackPerspectiveY : whitePerspectiveY) : whitePerspectiveY;
		return findCell(x,y);
	}

	function setConsideringCell(cell: Cell) {
		cells.forEach((cell) => {
			if (cell.highlighting === Highlighting.CONSIDERING) {
				cell.highlighting = null;
			}
		});
		if (draggingCell)
			highlightPossibleMoves(draggingCell.x, draggingCell.y);
		markCell(cell, Highlighting.CONSIDERING, true);
	}

	function handleMouseDown(event: MouseEvent) {
		if (locked || ending) return;
		if (event.button === 2) return;
		const element = event.target as HTMLElement;
		if (!element.dataset.xpos || !element.dataset.ypos) {
			console.error(`Element inside cell not identified: ${element.nodeName}`);
			return;
		}
		const x: number = +element.dataset.xpos;
		const y: number = +element.dataset.ypos;
		const piece = board.getPieceAt(x,y);
		if (!piece || !isTeamAllowedToMove(piece.team)) return;
		if (!piece) return;

		const cellElement = document.getElementById(`cell-${x}-${y}`);
		const asset = document.getElementById(`asset-${x}-${y}`)!;
		const chessboard = document.getElementById('chessboard')!;
		const chessCell = findCell(x,y)!;
		clearHighlights(false);
		markCell(chessCell, Highlighting.SELECTED, false);
		highlightPossibleMoves(x,y);
		if (!cellElement || !element) return;
		dragTimeout = setTimeout(() => {
			cellElement.removeChild(asset);
			chessboard.appendChild(asset);
			draggingCell = findCell(x, y)!;
			const cursor = calculateCursorOffset(event, chessboard);
			followCursor(chessboard, asset, cursor);
		}, 100);
	}

	function handleMouseMove(event: MouseEvent) {
		if (locked || !draggingCell) return;
		const chessboard = document.getElementById('chessboard')!;
		const asset = document.getElementById(`asset-${draggingCell.x}-${draggingCell.y}`)!;
		const cursor = calculateCursorOffset(event, chessboard);
		const piece = board.getPieceAt(draggingCell.x, draggingCell.y);
		if (!piece) return;
		followCursor(chessboard, asset, cursor);
		const cellUnderCursor = getCellUnderCursor(cursor, piece.team);
		if (!cellUnderCursor || (cellUnderCursor.x === draggingCell.x && cellUnderCursor.y === draggingCell.y)) return;
		setConsideringCell(cellUnderCursor);
	}

	function handleMouseUp(event: MouseEvent) {
		if (dragTimeout) clearTimeout(dragTimeout);
		if (locked) return;
		if (!draggingCell) return;
		const piece = board.getPieceAt(draggingCell.x, draggingCell.y);
		if (!piece || !isTeamAllowedToMove(piece.team)) return;
		const chessboard = document.getElementById('chessboard')!;
		const cursor = calculateCursorOffset(event, chessboard);
		const cellUnderCursor = getCellUnderCursor(cursor, piece.team);
		if (!cellUnderCursor) {
			clearHighlights(false);
			selectPiece(draggingCell);
			highlightPossibleMoves(draggingCell.x, draggingCell.y);
			resetPieceMovement(chessboard, false);
			return;
		}
		const cell = draggingCell;
		if (!board.isMovePossible(draggingCell.x, draggingCell.y, cellUnderCursor.x, cellUnderCursor.y)) {
			clearHighlights(false);
			selectPiece(cell);
			highlightPossibleMoves(cell.x, cell.y);
			resetPieceMovement(chessboard, false);
			return;
		}
		playMove(draggingCell.x, draggingCell.y, cellUnderCursor.x, cellUnderCursor.y).then((move) => {
			if (!move && draggingCell) {
				clearHighlights(false);
				selectPiece(cell);
				highlightPossibleMoves(cell.x, cell.y);
				resetPieceMovement(chessboard, false);
				return;
			}
			updatePage();
			resetPieceMovement(chessboard, true);
		})
		updatePage();
		resetPieceMovement(chessboard, true);
	}

	async function playMove(currentX: number, currentY: number, futureX: number, futureY: number): Promise<boolean> {
		const move = board.playMove(currentX, currentY, futureX, futureY);
		if (!move) return false;
		const sound = getMoveSound(move);
		console.log(sound);
		const audio = new Audio(`sounds/${getMoveSound(move)}.mp3`);
		audio.play();
		await opponent?.onMove(move);
		return true;
	}

	function handleContextMenu(event: Event) {
		event.preventDefault();
		if (locked) return;
		const element = event.target as HTMLElement;
		if (!element.dataset.xpos || !element.dataset.ypos) {
			console.error(`Element inside cell not identified: ${element.nodeName}`);
			return;
		}
		const x: number = +element.dataset.xpos;
		const y: number = +element.dataset.ypos;
		const cell = findCell(x,y);
		if (!cell) return;
		if (cell.highlighting === Highlighting.THREATENED) {
			cell.highlighting = null;
			cells = cells;
		} else {
			markCell(cell, Highlighting.THREATENED, true);
		}
	}

	function resetPieceMovement(chessboard: HTMLElement, clearAllHighlights: boolean) {
		if (!draggingCell) return;
		if (clearAllHighlights) {
			clearHighlights(true);
		}
		const cell = document.getElementById(`cell-${draggingCell.x}-${draggingCell.y}`)!;
		const asset = document.getElementById(`asset-${draggingCell.x}-${draggingCell.y}`)!;
		asset.style.position = '';
		asset.style.left = '';
		asset.style.top = '';
		chessboard.removeChild(asset);
		cell.appendChild(asset);
		draggingCell = null;
	}

	function selectPiece(cell: Cell) {
		clearHighlights(false);
		cell.highlighting = Highlighting.SELECTED;
		selectedCell = cell;
		cells = cells;
	}

	function markCell(cell: Cell, marker: Highlighting, update: boolean = true) {
		cell.highlighting = marker;
		if (update) {
			cells = cells;
		}
	}

	function highlightPossibleMoves(x: number, y: number) {
		const moves = board.calculateMovesFor(x,y);
		moves.forEach((move) => {
			highlightCell(move.to[0], move.to[1], Highlighting.POSSIBLE_MOVE);
		});
	}

	function clearHighlights(update: boolean = true) {
		cells.forEach(cell => {
			cell.highlighting = null;
		});
		if (update) cells = cells;
	}

	function getPieceAsset(piece: Piece): string {
		const teamName = piece.team == Team.White ? 'white' : 'black';
		return `/pieces/neo_${teamName}_${board.getPieceName(piece.type).toLowerCase()}.png`;
	}

	function isTeamAllowedToMove(team: Team): boolean {
		console.log(opponent, team, opponent.team, teamToPlay);
		const legal = !(opponent && team === opponent.team) && ((opponent && teamToPlay !== opponent.team) || (!opponent && teamToPlay === team));
		console.log(legal);
		return legal;
		// if (player && team !== player) return false;
		// if (player && teamToPlay !== player) return false;
		// if (!player && teamToPlay !== player) return true;
	}
</script>

		<div id="chessboard" on:mousemove={handleMouseMove} on:mouseup={handleMouseUp} draggable="false">
			{#each cells as cell (cell.x + '-' + cell.y)}
				{@const piece = board.getPieceAt(cell.x,cell.y)}
				<div
					id={`cell-${cell.x}-${cell.y}`}
					data-xpos={cell.x}
					data-ypos={cell.y}
					data-haspiece={piece != null}
					data-highlighting={cell.highlighting}
					draggable="false"
					on:click={handleSquareClick}
					on:mousedown={handleMouseDown}
					on:contextmenu={handleContextMenu}
					class={`cell ${(cell.y + cell.x) % 2 === 0 ? 'black' : 'white'}`}
				>
					{#if piece != null}
						<img
							id={`asset-${cell.x}-${cell.y}`}
							src={`${getPieceAsset(piece)}`}
							data-xpos={cell.x}
							data-ypos={cell.y}
							class="pieceAsset"
							in:send={{ key: piece }}
							out:receive={{ key: piece }}
							draggable="false"
							unselectable="on"
							height=96
						/>
					{/if}
					{#if cell.highlighting === Highlighting.POSSIBLE_MOVE && board.getPieceAt(cell.x,cell.y)}
						<span data-xpos={cell.x} data-ypos={cell.y} class="possibleCapture" draggable="false"></span>
					{:else if cell.highlighting === Highlighting.POSSIBLE_MOVE}
						<span data-xpos={cell.x} data-ypos={cell.y} class="possibleSpace" draggable="false"></span>
					{/if}
				</div>
			{/each}
		</div>

<style>
    #chessboard {
        position: relative;
        display: grid;
        grid-template-columns: repeat(8, 100px);
        grid-template-rows: repeat(8, 100px);
        gap: 0;
    }

    .cell:first-child {
        border-top-left-radius: 16px;
    }

    .cell:nth-child(57) {
        border-bottom-left-radius: 16px;
    }

    .cell {
        position: relative;
        display: flex;
        justify-content: center;
        align-items: center;
    }

    .cell[data-haspiece="true"] {
        cursor: pointer;
    }

    .cell[data-highlighting="threatened"].white {
        background-color: #de8181 !important;;
    }

    .cell[data-highlighting="threatened"].black {
        background-color: #df4f4f !important;;
    }


    .cell[data-highlighting="selected"] {
        background-color: #d9d984 !important;;
    }

    .cell[data-highlighting="considering"] {
        filter: brightness(95%);
        border: 4px solid white;
    }

    .possibleCapture {
        position: absolute;
        aspect-ratio: 1/1;
        width: 80%;
        background-color: transparent;
        border-radius: 100%;
        border: solid 8px black;
        opacity: 0.15;
        z-index: 0;
    }

    .possibleSpace {
        position: absolute;
        aspect-ratio: 1/1;
        width: 30%;
        background-color: black;
        border-radius: 100%;
        opacity: 0.15;
    }

    .pieceAsset {
        -moz-user-select: none;
        -webkit-user-select: none;
        user-select: none;
    }

    .cell.black {
        background-color: #aa6e30;
    }

    .cell.white {
        background-color: #ebd9af;
    }
</style>
