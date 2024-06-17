import type { Square } from '$lib/game/logic';

export function parseSquareName(squareName: string): Square {
	const x = squareName[0].charCodeAt(0) - 'a'.charCodeAt(0) + 1;
  const y = parseInt(squareName[1]);
  return {x,y};
}

export function translateCoordinates(x: number, y: number): string {
	return String.fromCharCode('a'.charCodeAt(0) + x - 1) + y;
}

export function translateSquare(square: Square): string {
	return translateCoordinates(square.x, square.y)
}