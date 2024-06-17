// place files you want to import through the `$lib` alias in this folder.
import BoardLogic, {PieceType, type Piece, Team} from '$lib/game/logic';
import type { BoardState } from '$lib/game/state';
import { calculateMovesForBishop } from '$lib/math/moveCalculator';
import {Highlighting} from '$lib/graphics/highlighting';

export {BoardLogic, PieceType, type Piece, Team};
export { calculateMovesForBishop };
export { Highlighting };
export { type BoardState };