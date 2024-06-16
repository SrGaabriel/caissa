// place files you want to import through the `$lib` alias in this folder.
import BoardLogic, {PieceType, type Piece, Team} from '$lib/logic/board';
import { calculateMovesForBishop } from '$lib/math/moveCalculator';
import Highlighting from '$lib/graphics/highlighting';

export {BoardLogic, PieceType, type Piece, Team};
export { calculateMovesForBishop };
export { Highlighting };