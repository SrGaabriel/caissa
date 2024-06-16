<script lang="ts">
  // Generate the chessboard cells data
  import {onMount} from "svelte";
  import { BoardLogic, type Piece, Team } from '$lib';
  import highlighting from '$lib/graphics/highlighting';

  let board = BoardLogic.DEFAULT;
  let cells = [];
  for (let row = 8; row >= 1; row--) {
    for (let col = 1; col <= 8; col++) {
      cells.push({
        x: col,
        y: row,
        highlighting: null,
        color: (row + col) % 2 === 0 ? 'black' : 'white'
      });
    }
  }

  function highlightCell(x: number, y: number) {
    const cell = document.getElementById(`cell-${x}-${y}`);
    if (cell) {
      cell.style.backgroundColor = 'red';
    }
  }

  function drawBoard() {
    for (let row = 1; row <= 8; row++) {
      for (let col = 1; col <= 8; col++) {
        drawPiece(col, row, board.getPieceAt(col, row));
      }
    }
  }

  function drawPiece(x: number, y: number, piece: Piece | null) {
    const cell = document.getElementById(`cell-${x}-${y}`);
    if (!cell) {
      console.log(`Cell ${x-1}-${y-1} not found`);
    }
    if (cell && piece) {
      const image = document.createElement("img");
      const teamName = piece.team == Team.White ? "white" : "black";
      image.src = `/pieces/neo_${teamName}_${board.getPieceName(piece.type).toLowerCase()}.png`
      image.height = 100
      image.dataset.xpos = x.toString();
      image.dataset.ypos = y.toString();
      cell.appendChild(image);
    }
  }

  function handleSquareClick(event: Event) {
    const cell = event.target as HTMLElement;
    if (!cell.dataset.xpos || !cell.dataset.ypos) {
      console.error("Element inside cell not identified");
      return;
    }
    
    const x: number = +cell.dataset.xpos;
    const y: number = +cell.dataset.ypos;
    cell.style.backgroundColor = 'blue';
    console.log(x, y);
    highlightPossibleMoves(x,y);
  }

  function highlightPossibleMoves(x: number, y: number) {
    const moves = board.calculateMovesFor(x,y);
    moves.forEach(move => {
      highlightCell(move[0], move[1]);
    });
  }

  onMount(() => {
    drawBoard();
  });
</script>

<main>
  <div id="chessboard">
    {#each cells as cell}
      <div id={`cell-${cell.x}-${cell.y}`} data-xpos={cell.x} data-ypos={cell.y} on:click={handleSquareClick} class="cell {cell.color}"></div>
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
    display: flex;
    justify-content: center;
    align-items: center;
  }
  
  .cell.black {
    background-color: #769656;
  }

  .cell.white {
    background-color: #eeeed2;
  }
</style>
