import { type BoardState, calculateMovesForBishop } from '$lib';
import {
    calculateMovesForKing,
    calculateMovesForKnight,
    calculateMovesForPawn,
    calculateMovesForQueen,
    calculateMovesForRook
} from '$lib/math/moveCalculator';
import { isCoordinateInsideMatrix } from '$lib/util/matrix';
import { cloneState, updateCastlingState } from '$lib/game/state';
import { parseSquareName } from '$lib/util/notation';
import type { Move } from '$lib/game/move';

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

export type Square = {
    x: number,
    y: number
}

export type Piece = {
    type: PieceType
    team: Team
}

export enum Side {
    KINGSIDE,
    QUEENSIDE
}

export default class BoardLogic {
    board: (Piece | null)[][];
    state: BoardState

    constructor(board: (Piece | null)[][], state: BoardState) {
        this.board = board;
        this.state = state;
    }

    getPieceAt(x: number, y: number): Piece | null {
        return this.board[y-1][x-1];
    }
    
    isPositionValid(x: number, y: number): boolean {
        return x > 0 && x < 9 && y > 0 && y < 9;
    }

    getAllMovesForTeam(team: Team): number[][] {
        let moves: number[][] = []
        for (let y = 1; y <= 8; y++) {
            for (let x = 1; x <= 8; x++) {
                const piece = this.getPieceAt(x, y);
                if (!piece || piece.team !== team) continue;
                const pieceMoves = this.hypotheticallyCalculateMovesFor(x,y,piece.type,piece.team);
                moves = moves.concat(pieceMoves);
            }
        }
        return moves;
    }

    getThreatenedSpaces(team: Team): number[][] {
        let spaces: number[][] = [];
        for (let y = 1; y <= 8; y++) {
            for (let x = 1; x <= 8; x++) {
                const piece = this.getPieceAt(x, y);
                if (!piece || piece.team === team) continue;
                const threats = this.optimisticallyCalculatePieceMoves(x, y, piece.type, piece.team, true);
                if (isCoordinateInsideMatrix(threats, 8, 7) || isCoordinateInsideMatrix(threats, 8, 5)) {
                    console.log(piece.type, x, y, "viado", (isCoordinateInsideMatrix(threats, 8, 7) ? '8,7' : '8,5'));
                }
                spaces = spaces.concat(threats);
            }
        }
        return spaces;
    }

    isCastlingAvailable(team: Team): boolean {
        switch (team) {
            case Team.White:
                return this.state.castling.whiteKingSide || this.state.castling.whiteQueenSide;
            case Team.Black:
                return this.state.castling.blackKingSide || this.state.castling.blackQueenSide;
            default:
                return false;
        }
    }

    getSide(x: number): Side {
        return x < 5 ? Side.QUEENSIDE : Side.KINGSIDE
    }

    getYOrientation(team: Team): number {
        return team === Team.White ? 1 : -1;
    }

    getBoardYStartForTeam(team: Team): number {
        return team === Team.White? 1 : 8;
    }

    getBoardYEndForTeam(team: Team): number {
        return team === Team.White? 8 : 1;
    }

    isTeamInCheck(team: Team): boolean {
        const threats = this.getThreatenedSpaces(team);
        return threats.some((coordinates) => {
            const [x, y] = coordinates;
            const piece = this.getPieceAt(x, y);
            return piece != null && piece.type === PieceType.KING && piece.team == team;
        });
    }

    calculateMovesFor(x: number, y: number): number[][] {
        const piece = this.getPieceAt(x,y);
        if (!piece) return [];
        return this.hypotheticallyCalculateMovesFor(x,y,piece.type,piece.team);
    }

    isMovePossible(currentX: number, currentY: number, futureX: number, futureY: number) {
        const pieceMoves = this.calculateMovesFor(currentX, currentY);
        return isCoordinateInsideMatrix(pieceMoves, futureX, futureY);
    }

    playMove(currentX: number, currentY: number, futureX: number, futureY: number, legally: boolean = true): Move | null {
        if (legally && !this.isMovePossible(currentX, currentY, futureX, futureY)) return null;
        const piece = this.getPieceAt(currentX, currentY);
        if (!piece) return null;
        let move: Move = {
            piece,
            check: false,
            capture: this.getPieceAt(futureX, futureY) != null,
            castle: false,
            checkmate: false,
            enPassant: false,
            promotion: false
        };

        if (piece.type == PieceType.PAWN && futureY == this.getBoardYEndForTeam(piece.team)) {
            piece.type = PieceType.QUEEN;
            move = { ...move, promotion: true }
        }
        if (piece.type == PieceType.ROOK && currentY == this.getBoardYStartForTeam(piece.team)) {
            if (currentX == 1 || currentX == 8) {
                const side = this.getSide(currentX);
                updateCastlingState(this.state, piece.team, side, false);
            }
        }
        if (piece.type == PieceType.KING) {
            if (Math.abs(futureX - currentX) === 2) {
                const rookX = futureX - currentX === 2 ? 8 : 1;
                const rookFutureX = futureX - currentX === 2 ? 6 : 4;
                this.movePiece(rookX, currentY, rookFutureX, currentY);
                move = { ...move, castle: true }
            }
            if (piece.team == Team.White) {
                this.state.castling.whiteQueenSide = false;
                this.state.castling.whiteKingSide = false;
            } else {
                this.state.castling.blackQueenSide = false;
                this.state.castling.blackKingSide = false;
            }
        }
        if (piece.type == PieceType.PAWN && Math.abs(futureY - currentY) === 2) {
            this.state.enPassantTargetSquare = { x: currentX, y: futureY - this.getYOrientation(piece.team) };
            move = { ...move, enPassant: true }
        } else {
            this.state.enPassantTargetSquare = undefined;
        }
        if (piece.type == PieceType.PAWN && futureX != currentX) {
            const targetPiece = this.getPieceAt(futureX, futureY);
            if (!targetPiece) {
                const enPassantTarget = this.getPieceAt(futureX, currentY);
                if (enPassantTarget && enPassantTarget.type == PieceType.PAWN && enPassantTarget.team !== piece.team) {
                    this.board[currentY - 1][futureX - 1] = null;
                    move = { ...move, capture: true, enPassant: true }
                }
            }
        }
        this.state.teamToPlay = piece.team === Team.White ? Team.Black : Team.White;
        this.board[currentY - 1][currentX - 1] = null;
        this.board[futureY - 1][futureX - 1] = piece;
        return move;
    }

    movePiece(currentX: number, currentY: number, futureX: number, futureY: number) {
        const piece = this.getPieceAt(currentX, currentY);
        this.board[currentY-1][currentX-1] = null;
        this.board[futureY-1][futureX-1] = piece;
    }

    hypotheticallyCalculateMovesFor(x: number, y: number, type: PieceType, team: Team, threatsOnly: boolean = false): number[][] {
        const optimisticMoves = this.optimisticallyCalculatePieceMoves(x,y,type,team, threatsOnly);
        const moves = []
        for (const move of optimisticMoves) {
            const hypotheticalBoard = this.clone();
            hypotheticalBoard.playMove(x, y, move[0], move[1], false);
            if (!hypotheticalBoard.isTeamInCheck(team)) {
                moves.push(move);
            }
        }
        return moves;
    }

    optimisticallyCalculatePieceMoves(x: number, y: number, type: PieceType, team: Team, threatsOnly: boolean = false): number[][] {
        switch (type) {
            case PieceType.PAWN:
                return calculateMovesForPawn(this, team, x, y, threatsOnly);
            case PieceType.KNIGHT:
                return calculateMovesForKnight(this, team, x, y);
            case PieceType.BISHOP:
                return calculateMovesForBishop(this, team, x, y);
            case PieceType.ROOK:
                return calculateMovesForRook(this, team, x, y);
            case PieceType.QUEEN:
                return calculateMovesForQueen(this, team, x, y);
            case PieceType.KING:
                return calculateMovesForKing(this, team, x, y, threatsOnly);
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

    clone(): BoardLogic {
        const newBoard = this.board.map(row => row.slice());
        const newState = cloneState(this.state);
        return new BoardLogic(newBoard, newState);
    }

    static fromFEN(fen: string): BoardLogic {
        const sectors = fen.split(' ');
        const rows = sectors[0].split('/');
        const board = [];

        const pieceFactory: { [key: string]: () => Piece } = {
            'P': () => ({ type: PieceType.PAWN, team: Team.White }),
            'N': () => ({ type: PieceType.KNIGHT, team: Team.White }),
            'B': () => ({ type: PieceType.BISHOP, team: Team.White }),
            'R': () => ({ type: PieceType.ROOK, team: Team.White }),
            'Q': () => ({ type: PieceType.QUEEN, team: Team.White }),
            'K': () => ({ type: PieceType.KING, team: Team.White }),
            'p': () => ({ type: PieceType.PAWN, team: Team.Black }),
            'n': () => ({ type: PieceType.KNIGHT, team: Team.Black }),
            'b': () => ({ type: PieceType.BISHOP, team: Team.Black }),
            'r': () => ({ type: PieceType.ROOK, team: Team.Black }),
            'q': () => ({ type: PieceType.QUEEN, team: Team.Black }),
            'k': () => ({ type: PieceType.KING, team: Team.Black })
        };

        for (const row of rows) {
            const boardRow: (Piece | null)[] = [];
            for (const char of row) {
                if (char >= '1' && char <= '8') {
                    for (let i = 0; i < parseInt(char, 10); i++) {
                        boardRow.push(null);
                    }
                } else {
                    boardRow.push(pieceFactory[char]());
                }
            }
            board.push(boardRow);
        }
        const turn = sectors[1] == 'b' ? Team.Black : Team.White;
        const castlingAvailability = sectors[2]
        const enPassantTargetSquareNotation = sectors[3];
        const enPassantTargetSquare = enPassantTargetSquareNotation === '-' ? undefined : parseSquareName(enPassantTargetSquareNotation);

        return new BoardLogic(board.reverse(), {
            teamToPlay: turn,
            enPassantTargetSquare,
            castling: {
                whiteKingSide: castlingAvailability.includes('K'),
                whiteQueenSide: castlingAvailability.includes('Q'),
                blackKingSide: castlingAvailability.includes('k'),
                blackQueenSide: castlingAvailability.includes('q')
            }
        });
    }
}