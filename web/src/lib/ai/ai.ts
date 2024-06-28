import { BoardLogic, PieceType, Team } from '$lib';
import type { Move } from '$lib/game/move';
import {fetchBestMove} from "$lib/api/moves";

type TranspositionTable = {
	[key: string]: number
};

export default class ArtificialIntelligence {
		board: BoardLogic;
		team: Team;
		transpositionTable: TranspositionTable;

	constructor(board: BoardLogic, team: Team) {
			this.board = board;
			this.team = team;
			this.transpositionTable = {};
		}

		async respond(move: Move) {
			const bestMove: any = await fetchBestMove(this.board.toFen());
			console.log(bestMove);
			this.board.playMove(bestMove.origin.x+1, bestMove.origin.y+1, bestMove.target.x+1, bestMove.target.y+1);
		}

		chooseMove(board: BoardLogic, depth: number, alpha: number, beta: number): Move | null {
			const moves = board.getAllMovesForTeam(this.team);
			if (moves.length === 0) return null;
			let bestMove = null;
			let bestScore = -Infinity;
			for (const move of moves) {
				const hypotheticalBoard = board.clone();
				hypotheticalBoard.playMove(move.from[0], move.from[1], move.to[0], move.to[1]);
				const score = -this.negamax(hypotheticalBoard, depth - 1, -beta, -alpha);
				if (score > bestScore) {
					bestScore = score;
					bestMove = move;
				}
				alpha = Math.max(alpha, score);
				if (alpha >= beta) break; // Beta cut-off
			}
			return bestMove;
		}

		negamax(board: BoardLogic, depth: number, alpha: number, beta: number): number {
			const boardHash = board.toFen();
			if (this.transpositionTable[boardHash] !== undefined) {
				return this.transpositionTable[boardHash];
			}

			if (depth === 0) {
				return this.evaluatePosition(board);
			}

			const moves = board.getAllMovesForTeam(this.team);
			if (moves.length === 0) return -Infinity;
			let bestScore = -Infinity;
			for (const move of moves) {
				const hypotheticalBoard = board.clone();
				hypotheticalBoard.playMove(move.from[0], move.from[1], move.to[0], move.to[1]);
				const score = -this.negamax(hypotheticalBoard, depth - 1, -beta, -alpha);
				bestScore = Math.max(bestScore, score);
				alpha = Math.max(alpha, score);
				if (alpha >= beta) break; // Beta cut-off
			}

			this.transpositionTable[boardHash] = bestScore;
			return bestScore;
		}

		evaluatePosition(board: BoardLogic): number {
			let score = 0;
			for (let x = 1; x <= 8; x++) {
				for (let y = 1; y <= 8; y++) {
					const piece = board.getPieceAt(x, y);
					if (!piece) continue;
					const value = this.calculatePieceValue(piece.type);
					score += piece.team === this.team ? value : -value;
				}
			}
			return score;
		}

		calculatePieceValue(pieceType: PieceType) {
			switch (pieceType) {
				case PieceType.PAWN:
					return 1;
				case PieceType.KNIGHT:
					return 3;
				case PieceType.BISHOP:
					return 3;
				case PieceType.ROOK:
					return 5;
				case PieceType.QUEEN:
					return 9;
				case PieceType.KING:
					return 1000;
			}
		}

	randomMove(): Move | null {
		const moves = this.board.getAllMovesForTeam(this.team);
		if (moves.length === 0) return null;
		const moveIndex = Math.floor(Math.random() * moves.length);
		return moves[moveIndex]
	}
}