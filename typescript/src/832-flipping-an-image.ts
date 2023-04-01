/**
 * @param image - 2D array representing image
 * @returns 2D array of modified image
 *
 * Problem 832:
 * https://leetcode.com/problems/flipping-an-image/
 */
export const flipAndInvertImage = (image: number[][]): number[][] => {
	for (let i = 0; i < image.length; i++) {
		image[i].reverse()
		for (let j = 0; j < image[i].length; j++) {
			image[i][j] = image[i][j] === 0 ? 1 : 0
		}
	}

	return image
}

if (import.meta.vitest) {
	const { it, expect } = import.meta.vitest

	it('test', () => {
		expect(
			flipAndInvertImage([
				[1, 1, 0],
				[1, 0, 1],
				[0, 0, 0],
			])
		).toStrictEqual([
			[1, 0, 0],
			[0, 1, 0],
			[1, 1, 1],
		])
	})
}
