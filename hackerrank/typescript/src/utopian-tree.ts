/**
 * @param n - Number of growth cycles
 * @returns Height after growth
 *
 * Challenge:
 * https://www.hackerrank.com/challenges/utopian-tree/problem
 */
export const utopianTree = (n: number): number => {
	let h = 1
	for (let index = 1; index <= n; index++) {
		if (index % 2 == 0) {
			h++
		} else {
			h *= 2
		}
	}
	return h
}

if (import.meta.vitest) {
	const { it, expect } = import.meta.vitest

	it('Utopian Tree', () => {
		expect(utopianTree(5)).toBe(14)
	})
}
