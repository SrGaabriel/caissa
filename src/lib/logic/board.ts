import { calculateMovesForBishop } from '$lib';

export enum PieceType {
    KING,
    QUEEN,
    ROOK,
    BISHOP,
    KNIGHT,
    PAWN
}

export enum Team {
    White,
    Black
}

export type Piece = {
    type: PieceType
    team: Team
}

export default class BoardLogic {
    board: (Piece | null)[][];

    constructor(board: (Piece | null)[][]) {
        this.board = board;
    }

    getPieceAt(x: number, y: number): Piece | null {
        return this.board[y-1][x-1];
    }
    
    calculateMovesFor(x: number, y: number): number[][] {
        console.log(x-1, y-1);
        const piece = this.getPieceAt(x,y);
        if (!piece) return [];
        switch (piece.type) {
            case PieceType.BISHOP:
                return calculateMovesForBishop(this, x, y);
            default:
                return []
        }
    }

    getPieceName(piece: PieceType): string {
        switch(piece) {
            case PieceType.KING:
                return "King";
            case PieceType.QUEEN:
                return "Queen";
            case PieceType.ROOK:
                return "Rook";
            case PieceType.BISHOP:
                return "Bishop";
            case PieceType.KNIGHT:
                return "Knight";
            case PieceType.PAWN:
                return "Pawn";
            default:
                return "Empty";
        }
    }

    static DEFAULT : BoardLogic = new BoardLogic([
            [
                { type: PieceType.ROOK, team: Team.White },
                { type: PieceType.KNIGHT, team: Team.White },
                { type: PieceType.BISHOP, team: Team.White },
                { type: PieceType.QUEEN, team: Team.White },
                { type: PieceType.KING, team: Team.White },
                { type: PieceType.BISHOP, team: Team.White },
                { type: PieceType.KNIGHT, team: Team.White },
                { type: PieceType.ROOK, team: Team.White }
            ],
            [
                { type: PieceType.PAWN, team: Team.White },
                { type: PieceType.PAWN, team: Team.White },
                { type: PieceType.PAWN, team: Team.White },
                { type: PieceType.PAWN, team: Team.White },
                { type: PieceType.PAWN, team: Team.White },
                { type: PieceType.PAWN, team: Team.White },
                { type: PieceType.PAWN, team: Team.White },
                { type: PieceType.PAWN, team: Team.White }
            ],
            [null, null, null, null, null, null, null, null],
            [null, null, null, null, null, null, null, null],
            [null, null, null, null, null, null, null, null],
            [null, null, null, null, null, null, null, null],
            [
                { type: PieceType.PAWN, team: Team.Black },
                { type: PieceType.PAWN, team: Team.Black },
                { type: PieceType.PAWN, team: Team.Black },
                { type: PieceType.PAWN, team: Team.Black },
                { type: PieceType.PAWN, team: Team.Black },
                { type: PieceType.PAWN, team: Team.Black },
                { type: PieceType.PAWN, team: Team.Black },
                { type: PieceType.PAWN, team: Team.Black }
            ],
            [
                { type: PieceType.ROOK, team: Team.Black },
                { type: PieceType.KNIGHT, team: Team.Black },
                { type: PieceType.BISHOP, team: Team.Black },
                { type: PieceType.QUEEN, team: Team.Black },
                { type: PieceType.KING, team: Team.Black },
                { type: PieceType.BISHOP, team: Team.Black },
                { type: PieceType.KNIGHT, team: Team.Black },
                { type: PieceType.ROOK, team: Team.Black }
            ]
    ]);
}