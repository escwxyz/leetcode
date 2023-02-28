/**
 * @param nums - Number array
 * @returns Length of array after removing duplicates
 *
 * Problem 26
 * https://leetcode.cn/problems/remove-duplicates-from-sorted-array/
 */
export const removeDuplicates = (nums: number[]): number => {
	if (nums.length < 2) {
		return nums.length
	}
	let a = 0
	let b = 0
	while (b < nums.length) {
		if (nums[a] !== nums[b]) {
			a++
			nums[a] = nums[b]
		}
		b++
	}

	return a + 1
}

if (import.meta.vitest) {
	const { it, expect } = import.meta.vitest

	it('test one', () => {
		expect(removeDuplicates([1, 1, 2])).toBe(2)
	})

	it('test two', () => {
		expect(removeDuplicates([0, 0, 1, 1, 1, 2, 2, 3, 3, 4])).toBe(5)
	})
}
