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
    White = "White",
    Black = "Black"
}

export type Player = {
    team: Team,
    onMove: (move: Move) => Promise<void>
}

export function getOppositeTeam(team: Team): Team {
    return team === Team.White? Team.Black : Team.White;
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

export enum GameEnding {
    CHECKMATE = "Checkmate",
    STALEMATE = "Stalemate"
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

    getAllMovesForTeam(team: Team): Move[] {
        let moves: Move[] = []
        for (let y = 1; y <= 8; y++) {
            for (let x = 1; x <= 8; x++) {
                const piece = this.getPieceAt(x, y);
                if (!piece || piece.team !== team) continue;
                const pieceMoves = this.hypotheticallyCalculateMovesFor(x, y, piece);
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
                const threats = this.optimisticallyCalculatePieceMoves(x, y, piece, true);
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

    // TODO: improve this mess
    getGameEnding(check: boolean | undefined = undefined): GameEnding | null {
        const opponent = getOppositeTeam(this.state.teamToPlay);
        const possibleMoves = this.getAllMovesForTeam(opponent).length;
        if (possibleMoves !== 0) return null;
        if (check === true || check === false && this.isTeamInCheck(opponent)) return GameEnding.CHECKMATE;
        else return GameEnding.STALEMATE
    }

    checkEnding() {
        this.state.ending = this.getGameEnding();
    }

    calculateMovesFor(x: number, y: number): Move[] {
        const piece = this.getPieceAt(x,y);
        if (!piece) return [];
        return this.hypotheticallyCalculateMovesFor(x, y, piece);
    }

    isMovePossible(currentX: number, currentY: number, futureX: number, futureY: number) {
        const pieceMoves = this.calculateMovesFor(currentX, currentY).map(move => move.to);
        return isCoordinateInsideMatrix(pieceMoves, futureX, futureY);
    }

    playMove(currentX: number, currentY: number, futureX: number, futureY: number, legally: boolean = true, allowEnd: boolean = true): Move | null {
        if (legally && !this.isMovePossible(currentX, currentY, futureX, futureY)) return null;
        const piece = this.getPieceAt(currentX, currentY);
        if (!piece) return null;
        let move: Move = {
            piece,
            from: [currentX, currentY],
            to: [futureX, futureY],
            check: false,
            capture: this.getPieceAt(futureX, futureY) != null,
            castle: false,
            ending: null,
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
        this.board[currentY - 1][currentX - 1] = null;
        this.board[futureY - 1][futureX - 1] = piece;
        const check = this.isTeamInCheck(getOppositeTeam(piece.team));
        move = { ...move, check };
        if (allowEnd) {
            const ending = this.getGameEnding(check);
            if (ending) {
                move = { ...move, ending }
                this.state.ending = ending;
            } else {
                this.state.teamToPlay = getOppositeTeam(piece.team);
            }
        } else {
            this.state.teamToPlay = getOppositeTeam(piece.team);
        }
        return move;
    }

    movePiece(currentX: number, currentY: number, futureX: number, futureY: number) {
        const piece = this.getPieceAt(currentX, currentY);
        this.board[currentY-1][currentX-1] = null;
        this.board[futureY-1][futureX-1] = piece;
    }

    hypotheticallyCalculateMovesFor(x: number, y: number, piece: Piece, threatsOnly: boolean = false): Move[] {
        const optimisticMoves = this.optimisticallyCalculatePieceMoves(x,y, piece, threatsOnly);
        const moves = []
        for (const moveCoords of optimisticMoves) {
            const hypotheticalBoard = this.clone();
            const move = hypotheticalBoard.playMove(x, y, moveCoords[0], moveCoords[1], false, false);
            if (move && !hypotheticalBoard.isTeamInCheck(piece.team)) {
                moves.push(move);
            }
        }
        return moves;
    }

    optimisticallyCalculatePieceMoves(x: number, y: number, piece: Piece, threatsOnly: boolean = false): number[][] {
        switch (piece.type) {
            case PieceType.PAWN:
                return calculateMovesForPawn(this, piece.team, x, y, threatsOnly);
            case PieceType.KNIGHT:
                return calculateMovesForKnight(this, piece.team, x, y);
            case PieceType.BISHOP:
                return calculateMovesForBishop(this, piece.team, x, y);
            case PieceType.ROOK:
                return calculateMovesForRook(this, piece.team, x, y);
            case PieceType.QUEEN:
                return calculateMovesForQueen(this, piece.team, x, y);
            case PieceType.KING:
                return calculateMovesForKing(this, piece.team, x, y, threatsOnly);
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
        // This would suffice for a shallow clone, but we also want to clone the pieces themselves
        const newBoard: (Piece | null)[][] = this.board.map(row => row.slice());
        // So it has to be done like this:
        for (let y = 0; y < 8; y++) {
            for (let x = 0; x < 8; x++) {
                const piece = this.board[y][x];
                if (piece) {
                    newBoard[y][x] = { type: piece.type, team: piece.team }
                }
            }
        }
        const newState = cloneState(this.state);
        return new BoardLogic(newBoard, newState);
    }

    toFen(): string {
        const pieceToFen: { [key: number]: string } = {
            [PieceType.PAWN]: "P",
            [PieceType.KNIGHT]: "N",
            [PieceType.BISHOP]: "B",
            [PieceType.ROOK]: "R",
            [PieceType.QUEEN]: "Q",
            [PieceType.KING]: "K"
        };

        const rows = this.board.map(row => {
            let emptyCount = 0;
            let fenRow = '';
            for (const piece of row) {
                if (piece) {
                    if (emptyCount > 0) {
                        fenRow += emptyCount;
                        emptyCount = 0;
                    }
                    const pieceName = pieceToFen[piece.type];
                    fenRow += piece.team === Team.White ? pieceName : pieceName.toLowerCase();
                } else {
                    emptyCount++;
                }
            }
            if (emptyCount > 0) {
                fenRow += emptyCount;
            }
            return fenRow;
        }).reverse().join('/');

        const turn = this.state.teamToPlay === Team.White ? 'w' : 'b';

        const castlingRights = [];
        if (this.state.castling.whiteKingSide) castlingRights.push('K');
        if (this.state.castling.whiteQueenSide) castlingRights.push('Q');
        if (this.state.castling.blackKingSide) castlingRights.push('k');
        if (this.state.castling.blackQueenSide) castlingRights.push('q');
        const castling = castlingRights.length ? castlingRights.join('') : '-';

        const enPassant = this.state.enPassantTargetSquare ? `${String.fromCharCode(96 + this.state.enPassantTargetSquare.x)}${this.state.enPassantTargetSquare.y}` : '-';

        const fen = `${rows} ${turn} ${castling} ${enPassant} 0 1`;
        return fen;
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

        const logic = new BoardLogic(board.reverse(), {
            teamToPlay: turn,
            enPassantTargetSquare,
            ending: null,
            castling: {
                whiteKingSide: castlingAvailability.includes('K'),
                whiteQueenSide: castlingAvailability.includes('Q'),
                blackKingSide: castlingAvailability.includes('k'),
                blackQueenSide: castlingAvailability.includes('q')
            }
        });
        logic.checkEnding();
        return logic;
    }
}