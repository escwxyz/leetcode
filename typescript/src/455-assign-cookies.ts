/**
 * Problem: https://leetcode.com/problems/assign-cookies/
 * @param g - greedy array
 * @param s - cookie array
 * @returns count
 *
 */
export const findContentChildren = (g: number[], s: number[]): number => {
	if (s.length === 0) return 0

	let count = 0

	const a = g.sort((a, b) => a - b)
	const b = s.sort((a, b) => a - b)

	let p = 0
	let q = 0

	while (p < a.length && q < b.length) {
		if (b[q] >= a[p]) {
			count++
			p++
			q++
		} else {
			q++
		}
	}

	return count
}

if (import.meta.vitest) {
	const { test, expect } = import.meta.vitest
	test('test one', () => {
		expect(findContentChildren([1, 2, 3], [1, 1])).toBe(1)
	})
	test('test two', () => {
		expect(findContentChildren([1, 2], [1, 2, 3])).toBe(2)
	})
}
