import { Team } from '$lib';

export type BoardState = {
	teamToPlay: Team,
	castling: CastlingState
}

export type CastlingState = {
	whiteKingSide: boolean,
  whiteQueenSide: boolean,
  blackKingSide: boolean,
  blackQueenSide: boolean,
}

export function defaultState(): BoardState {
	return {
    teamToPlay: Team.White,
    castling: {
      whiteKingSide: true,
      whiteQueenSide: true,
      blackKingSide: true,
      blackQueenSide: true,
    }
  }
}

export function cloneState(state: BoardState): BoardState {
	return {
    teamToPlay: state.teamToPlay,
    castling: {
      whiteKingSide: state.castling.whiteKingSide,
      whiteQueenSide: state.castling.whiteQueenSide,
      blackKingSide: state.castling.blackKingSide,
      blackQueenSide: state.castling.blackQueenSide,
    }
  }
}