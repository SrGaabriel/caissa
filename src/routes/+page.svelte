<script lang="ts">
    // Generate the chessboard cells data
    import { BoardLogic, Highlighting, type Piece, Team } from '$lib';

    type Cell = {
        x: number,
        y: number,
        piece: Piece | null,
        highlighting: Highlighting | null,
    }

    const fen = '5r1k/p6p/8/3Bqp1Q/P1p5/7P/5b2/R2R3K w - - 4 37';
    const board = BoardLogic.fromFEN(fen);
    let cells: Cell[] = [];
    let selectedCell: Cell | null = null;
    let teamToPlay: Team = Team.White;
    drawBoard(Team.White);

    function drawBoard(team: Team) {
        function pushCell(col: number, row: number) {
            cells.push({
                x: col,
                y: row,
                piece: board.getPieceAt(col, row),
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

    function animateMove(currentCell: Cell, futureCell: Cell) {
        futureCell.piece = currentCell.piece;
        currentCell.piece = null;
        teamToPlay = teamToPlay == Team.White ? Team.Black : Team.White;
    }

    function handleSquareClick(event: Event) {
        console.log(cells[0], cells[1], cells[8]);
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
        if (selectedCell && selectedCell.piece) { // If the player is trying to click on another cell while he already has one selected...
            if (cell.piece && cell.piece.team == selectedCell.piece.team) { // If the player is trying to switch attacking pieces...
                selectPiece(cell);
                if (cell.piece.team === teamToPlay) {
                    highlightPossibleMoves(x, y);
                }
                return;
            }
            if (teamToPlay !== selectedCell.piece.team) { // If the player is trying to move outside his turn...
                clearHighlights(false);
                cell.highlighting = Highlighting.SELECTED; // We just switch his apparently selected pieces
                cells = cells;
                return;
            } // Otherwise, we move
            const move = board.tryToMovePiece(selectedCell.x, selectedCell.y, x, y);
            if (move) {
                animateMove(selectedCell, cell);
                selectedCell = null;
                clearHighlights();
            }
            return;
        }
        // If the player is trying to select a piece...
        if (!cell.piece) return;
        if (cell.piece.team === teamToPlay) {
            selectPiece(cell);
            highlightPossibleMoves(x, y);
        } else {
            clearHighlights(false);
            markCell(cell, Highlighting.SELECTED);
        }
    }

    function selectPiece(cell: Cell) {
        if (!cell.piece) return;
        clearHighlights(false);
        cell.highlighting = Highlighting.SELECTED;
        selectedCell = cell;
        cells = cells;
    }

    function markCell(cell: Cell, marker: Highlighting, update: boolean = true) {
        cell.highlighting = Highlighting.SELECTED;
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
</script>

<main>
    <div id="chessboard">
        {#each cells as cell}
            <div
                id={`cell-${cell.x}-${cell.y}`}
                data-xpos={cell.x}
                data-ypos={cell.y}
                data-haspiece={!(!cell.piece)}
                data-highlighting={cell.highlighting}
                on:click={handleSquareClick}
                class={`cell ${(cell.y + cell.x) % 2 === 0 ? 'black' : 'white'}`}
            >
                {#if cell.piece}
                    <img src={`${getPieceAsset(cell.piece)}`} data-xpos={cell.x} data-ypos={cell.y} class="pieceAsset"
                         height=96/>
                {/if}
                {#if cell.highlighting === Highlighting.POSSIBLE_MOVE && cell.piece}
                    <span data-xpos={cell.x} data-ypos={cell.y} class="possibleCapture"></span>
                {:else if cell.highlighting === Highlighting.POSSIBLE_MOVE}
                    <span data-xpos={cell.x} data-ypos={cell.y} class="possibleSpace"></span>
                {/if}
            </div>
        {/each}
    </div>
</main>

<style>
    main {
        display: flex;
        justify-content: center;
        align-items: center;
        margin: 0;
        height: 98vh;
        width: 99vw;
    }

    #chessboard {
        display: grid;
        grid-template-columns: repeat(8, 100px);
        grid-template-rows: repeat(8, 100px);
        gap: 0;
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

    .cell[data-highlighting="selected"] {
        background-color: #d9d984 !important;;
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

    .cell.black {
        background-color: #565c96;
    }

    .cell.white {
        background-color: #c4c3c3;
    }
</style>
