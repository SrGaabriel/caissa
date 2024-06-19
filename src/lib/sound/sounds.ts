import type { Move } from '$lib/game/move';

export enum ChessSound {
	Capture = "Capture",
	Check = "Check",
	Castle = "Castle",
	Move = "Move"
}

export function getMoveSound(move: Move): ChessSound {
		if (move.check) return ChessSound.Check;
		if (move.castle) return ChessSound.Castle;
		if (move.capture) return ChessSound.Capture;
		return ChessSound.Move;
}