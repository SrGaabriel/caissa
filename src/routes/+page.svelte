<script lang="ts">
    // Generate the chessboard cells data
    import { BoardLogic, Highlighting, type Piece, Team } from '$lib';
    import { flip } from 'svelte/animate'
    import { getMoveSound } from '$lib/sound/sounds';
    import { getOppositeTeam } from '$lib/game/logic';

    type Cell = {
        x: number,
        y: number,
        highlighting: Highlighting | null,
    }

    type CursorPosition = {
        x: number,
        y: number,
    }

    // const fen = '5r1k/p6p/8/3Bqp1Q/P1p5/7P/5b2/R2R3K w - - 4 37';
    // const fen = `1nq2b1r/rb6/1ppp1npp/p7/4P1PP/2NPk2B/PPP2p1N/RQ2K1R1 w Q - 0 25`;
    const fen = `r1b2rk1/pp1n1ppp/2p2n2/q2pp1B1/1bPP4/2N1P3/PPQNBPPP/R3K2R w KQ - 0 10` // MAGNUS VS KASPAROV
    // const fen = `r3k2r/pbppqpb1/1pn3p1/7p/1N2pPn1/1PP4N/PB1P2PP/2QRKR2 b kq f3 0 1` // EN PASSANT
    let board = BoardLogic.fromFEN(fen);
    let cells: Cell[] = [];
    let selectedCell: Cell | null = null;
    let draggingCell: Cell | null = null;
    let dragTimeout: number | null = null;
    const isScreenMirrored: boolean = false;
    $: teamToPlay = board.state.teamToPlay;
    $: checkmate = false;
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
            if (teamToPlay !== selectedPiece.team) { // If the player is trying to move outside his turn...
                clearHighlights(false);
                cell.highlighting = Highlighting.SELECTED; // We just switch his apparently selected pieces
                cells = cells;
                return;
            } // Otherwise, we move
            const move = playMove(selectedCell.x, selectedCell.y, x, y);
            if (move) {
                updatePage();
                selectedCell = null;
                clearHighlights();
            }
            return;
        }
        // If the player is trying to select a piece...
        if (!piece) {
            clearHighlights(true);
        } else if (piece.team === teamToPlay) {
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
        if (event.button === 2) return;
        const element = event.target as HTMLElement;
        if (!element.dataset.xpos || !element.dataset.ypos) {
            console.error(`Element inside cell not identified: ${element.nodeName}`);
            return;
        }
        const x: number = +element.dataset.xpos;
        const y: number = +element.dataset.ypos;
        const piece = board.getPieceAt(x,y);
        if (!piece || piece.team !== board.state.teamToPlay) return;
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
        if (!draggingCell) return;
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
        if (dragTimeout)
            clearTimeout(dragTimeout);
        if (!draggingCell) return;
        const piece = board.getPieceAt(draggingCell.x, draggingCell.y);
        if (!piece) return;
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
        const move = playMove(draggingCell.x, draggingCell.y, cellUnderCursor.x, cellUnderCursor.y);
        if (!move) {
            clearHighlights(false);
            selectPiece(draggingCell);
            highlightPossibleMoves(draggingCell.x, draggingCell.y);
            resetPieceMovement(chessboard, false);
            return;
        }
        updatePage();
        resetPieceMovement(chessboard, true);
    }

    function playMove(currentX: number, currentY: number, futureX: number, futureY: number): boolean {
        const move = board.playMove(currentX, currentY, futureX, futureY);
        if (!move) return false;
        checkmate = move.checkmate;
        const sound = getMoveSound(move);
        console.log(sound);
        const audio = new Audio(`sounds/${getMoveSound(move)}.mp3`);
        audio.play();
        return true;
    }

    function handleContextMenu(event: Event) {
        event.preventDefault();
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
        const moves = board.calculateMovesFor(x, y);
        moves.forEach(move => {
            highlightCell(move[0], move[1], Highlighting.POSSIBLE_MOVE);
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

    function reset(fen: string) {
        board = BoardLogic.fromFEN(fen);
        updatePage();
        clearHighlights();
        selectedCell = null;
    }

    function starting() {
        reset('rnbqkbnr/pppppppp/8/8/8/8/PPPPPPPP/RNBQKBNR w KQkq - 0 1');
    }
</script>

<main>
    <div id="container">
        <div id="chessboard" on:mousemove={handleMouseMove} on:mouseup={handleMouseUp} draggable="false">
            {#each cells as cell (cell.x + '-' + cell.y)}
                <div
                    id={`cell-${cell.x}-${cell.y}`}
                    data-xpos={cell.x}
                    data-ypos={cell.y}
                    data-haspiece={!(!board.getPieceAt(cell.x,cell.y))}
                    data-highlighting={cell.highlighting}
                    draggable="false"
                    animate:flip
                    on:click={handleSquareClick}
                    on:mousedown={handleMouseDown}
                    on:contextmenu={handleContextMenu}
                    class={`cell ${(cell.y + cell.x) % 2 === 0 ? 'black' : 'white'}`}
                >
                    {#if board.getPieceAt(cell.x,cell.y) != null}
                        <img
                          id={`asset-${cell.x}-${cell.y}`}
                          src={`${getPieceAsset(board.getPieceAt(cell.x,cell.y))}`}
                          data-xpos={cell.x}
                          data-ypos={cell.y}
                          class="pieceAsset"
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
        <div id="sidebar">
            <div class="sidebar-overheader">
                <div class="turn-color-palette" style={`--turn-color: ${(checkmate ? getOppositeTeam(teamToPlay) : teamToPlay) === Team.Black ? '#191a19' : '#dbdbdb'}`}></div>
                <span class="team-to-play">{checkmate ? `Checkmate! ${getOppositeTeam(teamToPlay)}!` : `${teamToPlay} to play!`}</span>
            </div>
        </div>
    </div>
</main>

<style>
    main {
        display: flex;
        justify-content: center;
        align-items: center;
        margin: 0;
        padding: 0;
        overflow: hidden;
        height: 100vh;
        width: 100vw;
        background-color: #bed2d0;
        user-select: none;
    }

    #container {
        display: flex;
        justify-content: center;
        align-items: center;
        border-radius: 16px;
        box-shadow: 3px 8px 30px black;
    }

    #sidebar {
        background-color: #3f3e3e;
        width: 400px;
        height: 800px;
        color: azure;
        font-weight: 600;
        font-family: sans-serif;
        border-top-right-radius: 16px;
        border-bottom-right-radius: 16px;
    }

    .sidebar-overheader {
        display: flex;
        align-items: center;
        border-top-right-radius: 16px;
        width: 100%;
        height: 100px;
        background-color: #2a2929;
        border-bottom: 1px solid dimgray;
    }

    .turn-color-palette {
        width: 48px;
        aspect-ratio: 1/1;
        background-color: var(--turn-color);
        margin-left: 24px;
        border: 3px solid gray;
        border-radius: 100%;
    }

    .team-to-play {
        user-select: none;
        font-size: 26px;
        margin-left: 16px;
        margin-top: 2px;
        font-family: "Archivo Black", sans-serif;
        font-weight: 400;
        font-style: normal;
    }

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
