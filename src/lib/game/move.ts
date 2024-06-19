import { type Piece } from '$lib';

export type Move = {
	piece: Piece,
	check: boolean,
	checkmate:  boolean,
	castle: boolean,
	enPassant: boolean,
	promotion: boolean,
	capture: boolean,
}