<script lang="ts">
    // Generate the chessboard cells data
    import { BoardLogic, Highlighting, type Piece, Team } from '$lib';

    type Cell = {
        x: number,
        y: number,
        piece: Piece | null,
        highlighting: Highlighting | null,
    }

    const fen = 'rnb1k1n1/ppp2pp1/3p1qr1/2b1p1Bp/3P2P1/1P1Q1NR1/P1P1PP1P/RN2KB2 b Qq - 1 8';
    const board = BoardLogic.fromFEN(fen);
    let cells: Cell[] = [];
    drawBoard(Team.Black);

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

    function handleSquareClick(event: Event) {
        console.log(cells[0], cells[1], cells[8]);
        const element = event.target as HTMLElement;
        if (!element.dataset.xpos || !element.dataset.ypos) {
            console.error('Element inside cell not identified');
            return;
        }
        const x: number = +element.dataset.xpos;
        const y: number = +element.dataset.ypos;

        highlightPossibleMoves(x, y);
    }

    function highlightPossibleMoves(x: number, y: number) {
        const moves = board.calculateMovesFor(x, y);
        moves.forEach(move => {
            highlightCell(move[0], move[1], Highlighting.POSSIBLE_MOVE);
        });
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
                    <span class="possibleCapture"></span>
                {:else if cell.highlighting === Highlighting.POSSIBLE_MOVE}
                    <span class="possibleSpace"></span>
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
        background-color: #769656;
    }

    .cell.white {
        background-color: #eeeed2;
    }
</style>
