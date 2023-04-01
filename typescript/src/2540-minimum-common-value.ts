/**
 * @param nums1 - Array one
 * @param nums2 - Array two
 * @returns Minimum common value
 *
 * Problem 2540:
 * https://leetcode.com/problems/minimum-common-value/
 */
export const getCommon = (nums1: number[], nums2: number[]): number => {
	let p1: number = 0
	let p2: number = 0
	let r: number = -1
	while (p1 < nums1.length && p2 < nums2.length) {
		if (nums1[p1] < nums2[p2]) {
			p1++
		} else if (nums1[p1] > nums2[p2]) {
			p2++
		} else {
			// we already find the minimum since it's sorted
			r = nums1[p1]
			break
		}
	}

	return r
}

if (import.meta.vitest) {
	const { it, expect } = import.meta.vitest

	it('true', () => {
		expect(getCommon([1, 2, 3], [2, 4])).toBe(2)
	})

	it('false', () => {
		expect(getCommon([22, 4], [10, 99, 21])).toBe(-1)
	})
}
