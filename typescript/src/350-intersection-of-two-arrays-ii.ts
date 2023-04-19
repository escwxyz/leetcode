/**
 * @param nums1 - array one
 * @param nums2 - array two
 * @returns intersection
 * Problem:https://leetcode.com/problems/intersection-of-two-arrays-ii/
 */
export const intersect = (nums1: number[], nums2: number[]): number[] => {
	if (nums1.length === 1) return nums2.includes(nums1[0]) ? nums1 : []
	if (nums2.length === 1) return nums1.includes(nums2[0]) ? nums2 : []

	let a = nums1.sort((a, b) => a - b)
	let b = nums2.sort((a, b) => a - b)

	let p1 = 0
	let p2 = 0

	const result: Array<number> = []

	while (p1 <= a.length - 1 && p2 <= b.length - 1) {
		if (a[p1] < b[p2]) {
			p1++
		} else if (a[p1] > b[p2]) {
			p2++
		} else {
			result.push(a[p1])
			p1++
			p2++
		}
	}

	return result
}

if (import.meta.vitest) {
	const { test, expect } = import.meta.vitest

	test('one', () => {
		expect(intersect([1, 2, 2, 1], [2, 2])).toStrictEqual([2, 2])
	})
	test('one', () => {
		expect(intersect([4, 9, 5], [9, 4, 9, 8, 4])).toStrictEqual([4, 9])
	})
}
