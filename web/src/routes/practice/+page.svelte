<script lang="ts">
    // Generate the chessboard cells data
    import { BoardLogic, Team } from '$lib';
    import Chessboard from '$lib/Chessboard.svelte';
    import { GameEnding } from '$lib/game/logic';

    const fen = 'rnbqkbnr/pppppppp/8/8/8/8/PPPPPPPP/RNBQKBNR w KQkq - 0 1';
    let board = BoardLogic.fromFEN(fen);
    $: ending = board.state.ending;
    $: teamToPlay = board.state.teamToPlay;

    function getGameEnding(): string {
        if (!ending) throw Error("Game hasn't ended");
        switch (ending) {
            case GameEnding.CHECKMATE: return `Checkmate! ${teamToPlay}`
            case GameEnding.STALEMATE: return `Stalemate! Draw`
        }
    }
</script>

<main>
    <div id="container">
        <Chessboard bind:board={board} opponent={null}/>
        <div id="sidebar">
            <div class="sidebar-overheader">
                <div class="turn-color-palette" style={`--turn-color: ${teamToPlay === Team.Black ? '#191a19' : '#dbdbdb'}`}></div>
                <span class="team-to-play">{ending ? getGameEnding() : `${teamToPlay} to play!`}</span>
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
        font-family: "Archivo BLACK", sans-serif;
        font-weight: 400;
        font-style: normal;
    }
</style>
