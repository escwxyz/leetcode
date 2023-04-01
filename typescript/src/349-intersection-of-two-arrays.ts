/**
 * @param nums1 - Integer array one
 * @param nums2 - Integer array two
 * @returns Intersection array of two arrays
 *
 * Problem 349:
 * https://leetcode.com/problems/intersection-of-two-arrays
 */
export const intersection = (nums1: number[], nums2: number[]): number[] => {
	const set1 = new Set(nums1)
	const set2 = new Set(nums2)
	const r: number[] = []

	set1.forEach((v) => {
		if (set2.has(v)) {
			r.push(v)
		}
	})

	return r
}

if (import.meta.vitest) {
	const { it, expect } = import.meta.vitest

	it('[2]', () => {
		expect(intersection([1, 2, 2, 1], [2, 2])).toStrictEqual([2])
	})
}
