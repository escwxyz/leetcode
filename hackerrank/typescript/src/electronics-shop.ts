/**
 * @param keyboards - Price array of keyboard
 * @param drives - Price array of drive
 * @param b - Total budget
 * @returns Money spent or -1 if impossible
 */

// Challenges:
// https://www.hackerrank.com/challenges/electronics-shop/problem

export const getMoneySpent = (
	keyboards: number[],
	drives: number[],
	b: number
): number => {
	const r = keyboards
		.filter((v) => v < b)
		.map((v) => {
			const d = drives.filter((l) => l + v <= b)
			return d.length == 0 ? 0 : v + Math.max.apply(null, d)
		})

	return r.every((v) => v == 0) ? -1 : Math.max.apply(null, r)
}

if (import.meta.vitest) {
	const { it, expect } = import.meta.vitest
	it('Non -1', () => {
		expect(getMoneySpent([5, 2, 8], [3, 1], 10)).toBe(9)
	})

	it('-1', () => {
		expect(getMoneySpent([5], [4], 5)).toBe(-1)
	})
}