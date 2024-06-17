export function isCoordinateInsideMatrix(matrix: number[][], first: number, second: number): boolean {
	return matrix.some(coordinate =>
		coordinate[0] === first && coordinate[1] === second
	);
}