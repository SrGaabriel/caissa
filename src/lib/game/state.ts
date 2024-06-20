import { Team } from '$lib';
import { GameEnding, Side, type Square } from '$lib/game/logic';

export type BoardState = {
	teamToPlay: Team,
	castling: CastlingState,
	ending: GameEnding | null,
	enPassantTargetSquare: Square | undefined
}

export type CastlingState = {
	whiteKingSide: boolean,
  whiteQueenSide: boolean,
  blackKingSide: boolean,
  blackQueenSide: boolean,
}

export function updateCastlingState(state: BoardState, team: Team, side: Side, value: boolean) {
	const { castling } = state;

	switch (team) {
		case Team.White:
			state.castling = { ...castling, [side === Side.KINGSIDE ? 'whiteKingSide' : 'whiteQueenSide']: value };
			break;
		case Team.Black:
			state.castling = { ...castling, [side === Side.KINGSIDE ? 'blackKingSide' : 'blackQueenSide']: value };
			break;
		default:
			throw new Error('Invalid team specified');
	}
}

export function cloneState(state: BoardState): BoardState {
	return {
    teamToPlay: state.teamToPlay,
		ending: state.ending,
		enPassantTargetSquare: state.enPassantTargetSquare,
    castling: {
      whiteKingSide: state.castling.whiteKingSide,
      whiteQueenSide: state.castling.whiteQueenSide,
      blackKingSide: state.castling.blackKingSide,
      blackQueenSide: state.castling.blackQueenSide,
    }
  }
}