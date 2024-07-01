import {Team} from "$lib";
import type {Square} from "$lib/game/logic";

const API = "https://cdbf-2804-7f0-20-2002-7468-4962-c308-664b.ngrok-free.app";

export async function fetchPieceMoves(fen: string, square: Square): Promise<any> {
    return fetch(`${API}/api/playground/moves/piece`, {
        method: 'POST',
        body: JSON.stringify({fen, coordinates: square}),
        headers: {
            'Content-Type': 'application/json',
        },
    }).then(res => res.json());
}

export async function fetchAllTeamMoves(fen: string, team: Team): Promise<any> {
    return fetch(`${API}/api/playground/moves/team`, {
        method: 'POST',
        body: JSON.stringify({fen, team: team === Team.White ? 0 : 1}),
        headers: {
            'Content-Type': 'application/json',
        },
    }).then(res => res.json());
}

export async function fetchBestMove(fen: string): Promise<any> {
    return fetch(`${API}/api/playground/moves/best`, {
        method: 'POST',
        body: JSON.stringify({fen}),
        headers: {
            'Content-Type': 'application/json',
        },
    }).then(res => res.json());
}

export async function fetchThreats(fen: string): Promise<any> {
    return fetch(`${API}/api/playground/moves/threats`, {
        method: 'POST',
        body: JSON.stringify({fen}),
        headers: {
            'Content-Type': 'application/json',
        },
    }).then(res => res.json());
}