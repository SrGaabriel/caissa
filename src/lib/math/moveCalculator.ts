import BoardLogic, { Team } from '../logic/board.js';
import { isCoordinateInsideMatrix } from '$lib/util/matrix';

const KNIGHT_DIRECTIONS = [
    [-1, 2],
    [1, 2],
    [-2, 1],
    [-2, -1],
    [2, 1],
    [2, -1],
    [-1, -2],
    [1, -2]
];
const ROOK_DIRECTIONS = [
    [1, 0],
    [0, 1],
    [-1, 0],
    [0, -1]
];

const BISHOP_DIRECTIONS = [
    [1, 1],
    [1, -1],
    [-1, 1],
    [-1, -1]
];

const KING_QUEEN_DIRECTIONS = [
    [1, 1],
    [1, -1],
    [-1, 1],
    [-1, -1],
    [1, 0],
    [0, 1],
    [-1, 0],
    [0, -1]
];

export function getPawnYDirection(team: Team) {
    return team === Team.White ? 1 : -1;
}

export function calculateMovesForPawn(board: BoardLogic, team: Team, x: number, y: number): number[][] {
    const moves = [];
    const yDirection = getPawnYDirection(team);
    if (canPawnMarch(board, x, y, yDirection)) {
        moves.push([x, y + yDirection]);
        if ((team === Team.White && y === 2) || (team === Team.Black && y === 7)) {
            if (canPawnMarch(board, x, y, 2 * yDirection)) {
                moves.push([x, y + yDirection * 2]);
            }
        }
    }
    const pawnTakingDirections = [
        [1, yDirection],
        [-1, yDirection]
    ]
    for (const takingDirection of pawnTakingDirections) {
        const [dx, dy] = takingDirection;
        const newX = x + dx;
        const newY = y + dy;
        const targetingPiece = board.getPieceAt(newX, newY);
        if (targetingPiece && targetingPiece.team !== team) {
            moves.push([newX, newY]);
        }
    }
    return moves;
}

function canPawnMarch(board: BoardLogic, x: number, y: number, steps: number): boolean {
    const newY = y + steps;
    console.log(x, newY, "march!");
    const impedingPiece = board.getPieceAt(x, newY);
    return !impedingPiece;
}

export function calculateMovesForKnight(board: BoardLogic, team: Team, x: number, y: number): number[][] {
    const moves = [];
    for (const direction of KNIGHT_DIRECTIONS) {
        const [dx, dy] = direction;
        const newX = x + dx;
        const newY = y + dy;
        if (!board.isPositionValid(newX, newY)) {
            continue;
        }

        const targetingPiece = board.getPieceAt(newX, newY);
        if (targetingPiece?.team === team)
            continue;
        moves.push([newX, newY]);
    }
    return moves;
}

export function calculateMovesForKing(board: BoardLogic, team: Team, x: number, y: number, smart: boolean): number[][] {
    const moves = []
    const threatenedSpaces = smart ? board.getThreatenedSpaces(team) : [];
    for (const direction of KING_QUEEN_DIRECTIONS) {
        const [dx, dy] = direction;
        const newX = x + dx;
        const newY = y + dy;
        if (!board.isPositionValid(newX, newY)) {
            continue;
        }
        if (smart && isCoordinateInsideMatrix(threatenedSpaces, newX, newY)) { // Values have to be inverted
            console.log(newX, newY, "is inside!")
            continue;
        }

        const targetingPiece = board.getPieceAt(newX, newY);
        if (targetingPiece?.team === team)
            continue;
        moves.push([newX, newY]);
    }
    return moves;
}

function calculateProgressiveMoves(board: BoardLogic, team: Team, x: number, y: number, directions: number[][]): number[][] {
    const moves = [];
    for (const direction of directions) {
        const [dx, dy] = direction;
        let i = 1;
        while (true) {
            const newX = x + i * dx;
            const newY = y + i * dy;
            if (!board.isPositionValid(newX, newY)) {
                break;
            }
            const targetingPiece = board.getPieceAt(newX, newY);
            if (targetingPiece) {
                if (targetingPiece.team !== team) {
                    moves.push([newX, newY]);
                }
                break;
            }
            moves.push([newX, newY]);
            i++;
        }
    }
    return moves;
}

export function calculateMovesForRook(board: BoardLogic, team: Team, x: number, y: number): number[][] {
    return calculateProgressiveMoves(board, team, x, y, ROOK_DIRECTIONS);
}

export function calculateMovesForBishop(board: BoardLogic, team: Team, x: number, y: number): number[][] {
    return calculateProgressiveMoves(board, team, x, y, BISHOP_DIRECTIONS);
}

export function calculateMovesForQueen(board: BoardLogic, team: Team, x: number, y: number): number[][] {
    return calculateProgressiveMoves(board, team, x, y, KING_QUEEN_DIRECTIONS);
}