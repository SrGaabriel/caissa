import {Team} from "$lib";
import type {Square} from "$lib/game/logic";

export async function fetchPieceMoves(fen: string, square: Square): Promise<any> {
    return fetch(`https://183a-2804-7f0-20-117d-a026-8f00-f6d5-a215.ngrok-free.app/api/playground/moves/piece`, {
        method: 'POST',
        body: JSON.stringify({fen, coordinates: square}),
        headers: {
            'Content-Type': 'application/json',
        },
    }).then(res => res.json());
}

export async function fetchAllTeamMoves(fen: string, team: Team): Promise<any> {
    return fetch(`https://183a-2804-7f0-20-117d-a026-8f00-f6d5-a215.ngrok-free.app/api/playground/moves/team`, {
        method: 'POST',
        body: JSON.stringify({fen, team: team === Team.White ? 0 : 1}),
        headers: {
            'Content-Type': 'application/json',
        },
    }).then(res => res.json());
}

export async function fetchBestMove(fen: string): Promise<any> {
    return fetch(`https://183a-2804-7f0-20-117d-a026-8f00-f6d5-a215.ngrok-free.app/api/playground/moves/best`, {
        method: 'POST',
        body: JSON.stringify({fen}),
        headers: {
            'Content-Type': 'application/json',
        },
    }).then(res => res.json());
}