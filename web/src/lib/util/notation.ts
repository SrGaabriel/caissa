import {PieceType, type Square} from '$lib/game/logic';

export function parseSquareName(squareName: string): Square {
	const x = squareName[0].charCodeAt(0) - 'a'.charCodeAt(0) + 1;
  const y = parseInt(squareName[1]);
  return {x,y};
}

// Here we'll check things like mov.check, mov.piece, etc...
// Ideally, we'll have something like Nxf4+ for a knight capturing a piece at f4 with a check
// or O-O for a king side castle
// or e4 for a pawn moving to e4
// or exd5 for a pawn capturing a piece at d5
// or e8=Q for a pawn promoting to a queen
// or e8=Q+ for a pawn promoting to a queen with a check
// or e8=Q# for a pawn promoting to a queen with a checkmate
// export function parseMove(origin: Square, team: Team, move: string): Move {
// 	const capture = move.includes('x');
// 	const check = move.includes('+');
// 	const checkmate = move.includes('#');
// 	const promotion = move.includes('=');
// 	const castle = move === 'O-O' || move === 'O-O-O';
// 	const enPassant = move.includes('e.p.'); // TODO: fix
// 	const type = translatePiece(move[0]);
// 	const piece: Piece = {
// 		type,
// 		team
// 	};
// 	const ending = checkmate ? GameEnding.CHECKMATE : null; // TODO: fix
// 	const square = move.slice(1, move.length - (promotion ? 3 : 0));
// 	return {
// 		piece,
// 		capture,
// 		check,
// 		ending,
// 		promotion,
// 		castle,
// 		enPassant,
// 		from: [origin.x, origin.y],
// 		to: square
// 	};
// }

export function translateCoordinates(x: number, y: number): string {
	return String.fromCharCode('a'.charCodeAt(0) + x - 1) + y;
}

export function translateSquare(square: Square): string {
	return translateCoordinates(square.x, square.y)
}

export function translatePiece(piece: string): PieceType {
	switch (piece) {
		case 'K':
			return PieceType.KING;
		case 'Q':
			return PieceType.QUEEN;
		case 'R':
			return PieceType.ROOK;
		case 'B':
			return PieceType.BISHOP;
		case 'N':
			return PieceType.KNIGHT;
		default:
			return PieceType.PAWN;
	}
}