import { calculateMovesForBishop } from '$lib';
import {
    calculateMovesForKing,
    calculateMovesForKnight,
    calculateMovesForPawn,
    calculateMovesForQueen,
    calculateMovesForRook
} from '$lib/math/moveCalculator';
import { isCoordinateInsideMatrix } from '$lib/util/matrix';

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
    
    isPositionValid(x: number, y: number): boolean {
        return x > 0 && x < 9 && y > 0 && y < 9;
    }

    getThreatenedSpaces(team: Team): number[][] {
        let spaces: number[][] = [];
        for (let y = 1; y <= 8; y++) {
            for (let x = 1; x <= 8; x++) {
                const piece = this.getPieceAt(x, y);
                if (!piece || piece.team === team) continue;
                const threats = this.hypotheticallyCalculateMovesFor(x, y, piece.type, piece.team, false);
                spaces = spaces.concat(threats);
            }
        }
        return spaces;
    }

    calculateMovesFor(x: number, y: number): number[][] {
        const piece = this.getPieceAt(x,y);
        if (!piece) return [];
        return this.hypotheticallyCalculateMovesFor(x,y,piece.type,piece.team, true);
    }

    isMovePossible(currentX: number, currentY: number, futureX: number, futureY: number) {
        const pieceMoves = this.calculateMovesFor(currentX, currentY);
        return isCoordinateInsideMatrix(pieceMoves, futureX, futureY);
    }

    tryToMovePiece(currentX: number, currentY: number, futureX: number, futureY: number): boolean {
        console.log("Move!")
        if (!this.isMovePossible(currentX, currentY, futureX, futureY)) return false;
        const currentPiece = this.getPieceAt(currentX, currentY);
        if (!currentPiece) return false;
        // const futurePiece = this.getPieceAt(futureX, futureY);
        this.board[currentY-1][currentX-1] = null;
        this.board[futureY-1][futureX-1] = currentPiece;
        console.log("Move successful!")
        return true;
    }

    hypotheticallyCalculateMovesFor(x: number, y: number, type: PieceType, team: Team, smart: boolean): number[][] {
        switch (type) {
            case PieceType.PAWN:
                return calculateMovesForPawn(this, team, x, y);
            case PieceType.KNIGHT:
                return calculateMovesForKnight(this, team, x, y);
            case PieceType.BISHOP:
                return calculateMovesForBishop(this, team, x, y);
            case PieceType.ROOK:
                return calculateMovesForRook(this, team, x, y);
            case PieceType.QUEEN:
                return calculateMovesForQueen(this, team, x, y);
            case PieceType.KING:
                return calculateMovesForKing(this, team, x, y, smart);
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

    static fromFEN(fen: string): BoardLogic {
        const rows = fen.split(' ')[0].split('/');
        const board = [];

        const pieceMap: { [key: string]: Piece } = {
            'P': { type: PieceType.PAWN, team: Team.White },
            'N': { type: PieceType.KNIGHT, team: Team.White },
            'B': { type: PieceType.BISHOP, team: Team.White },
            'R': { type: PieceType.ROOK, team: Team.White },
            'Q': { type: PieceType.QUEEN, team: Team.White },
            'K': { type: PieceType.KING, team: Team.White },
            'p': { type: PieceType.PAWN, team: Team.Black },
            'n': { type: PieceType.KNIGHT, team: Team.Black },
            'b': { type: PieceType.BISHOP, team: Team.Black },
            'r': { type: PieceType.ROOK, team: Team.Black },
            'q': { type: PieceType.QUEEN, team: Team.Black },
            'k': { type: PieceType.KING, team: Team.Black }
        };

        for (const row of rows) {
            const boardRow: (Piece | null)[] = [];
            for (const char of row) {
                if (char >= '1' && char <= '8') {
                    for (let i = 0; i < parseInt(char, 10); i++) {
                        boardRow.push(null);
                    }
                } else {
                    boardRow.push(pieceMap[char]);
                }
            }
            board.push(boardRow);
        }

        return new BoardLogic(board.reverse());
    }
}