import BoardLogic from "../logic/board.js";

export function calculateMovesForPawn(board: BoardLogic, x: number, y: number): number[][] {
    
}

export function calculateMovesForBishop(board: BoardLogic, x: number, y: number): number[][] {
    const moves: number[][] = [];
    const directions = [
        [1, 1],
        [1, -1],
        [-1, 1],
        [-1, -1]
    ];

    for (const direction of directions) {
        const [dx, dy] = direction;
        let i = 1;
        while (true) {
            const newX = x + i * dx;
            const newY = y + i * dy;
            if (newX <= 0 || newX >= 8 || newY <= 0 || newY >= 8) {
                break;
            }
            console.log("Target: ", newX, newY)
            const targetingPiece = board.getPieceAt(newX, newY);
            if (targetingPiece !== null) {
                moves.push([newX, newY]);
                break;
            }
            moves.push([newX, newY]);
            i++;
        }
    }

    return moves;
}