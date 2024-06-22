
<main>
	<div class="background"/>
	<div class="page">
		<a href="/bot">Bot</a>
		<a href="/practice">Practice</a>
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
        background: #bed2d0;
    }
		.background {
				position: fixed;
				top: 0;
				left: 0;
				width: 100vw;
				height: 100vh;
				background: #bed2d0 url('/brand/background_board.png') no-repeat fixed 50% 50%;
				background-size: 100%;
				filter: blur(24px);
				-webkit-filter: blur(24px);
				z-index: 1;
		}
		.page {
				display: flex;
				flex-direction: column;
				z-index: 2;
		}
		a {
				color: #3f3e3e;
				font-size: 48px;
				margin: 24px;
		}
</style>