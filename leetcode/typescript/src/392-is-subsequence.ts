/**
 * @param t - Target string
 * @param s - Search string
 * @returns If target string includes search string
		 *
 * Problem:
 * https://leetcode.cn/problems/is-subsequence
 */

export const isSubSequence = (t: string, s: string): boolean => {
	let index = -1

	for (const c of s) {
		index = t.indexOf(c, index + 1)

		if (index < 0) return false
	}

	return true
}

if (import.meta.vitest) {
	const { it, expect } = import.meta.vitest

	it('true', () => {
		expect(isSubSequence('ahbgdc', 'abc')).toBe(true)
	})

	it('false', () => {
		expect(isSubSequence('ahbgdc', 'axc')).toBe(false)
	})
}
