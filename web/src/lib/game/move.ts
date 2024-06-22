import { type Piece } from '$lib';
import type { GameEnding } from '$lib/game/logic';

export type Move = {
	piece: Piece,
	from: number[],
	to: number[],
	check: boolean,
	ending: GameEnding | null,
	castle: boolean,
	enPassant: boolean,
	promotion: boolean,
	capture: boolean,
}