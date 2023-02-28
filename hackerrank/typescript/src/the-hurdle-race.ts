/**
 * @param k - Initial maximum height one can jump
 * @param height - Array of heights of hurdles
 * @returns Number of dose
 *
 * Challenge
 * https://www.hackerrank.com/challenges/the-hurdle-race/problem
 */
export const hurdleRace = (k: number, height: number[]): number => {
	const max = Math.max.apply(null, height)

	return k >= max ? 0 : max - k
}

if (import.meta.vitest) {
	const { it, expect } = import.meta.vitest

	it('Hurdle Race', () => {
		expect(hurdleRace(1, [1, 2, 3, 3, 4])).toBe(2)
	})
}
