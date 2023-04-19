/**
 * @param nums - Array
 * @returns Array after operation
 *
 * Problem 283:
 * https://leetcode.com/problems/move-zeroes/
 */
export const moveZeroes = (nums: number[]): number[] => {
	let fast = 0
	let slow = 0

	while (fast < nums.length) {
		if (nums[fast] !== 0) {
			nums[slow++] = nums[fast]
		}
		fast++
	}
	for (let index = slow; index < nums.length; index++) {
		nums[index] = 0
	}

	return nums
}

if (import.meta.vitest) {
	const { it, expect } = import.meta.vitest

	it('test one', () => {
		expect(moveZeroes([0, 1, 0, 3, 12])).toStrictEqual([1, 3, 12, 0, 0])
	})
	it('test two', () => {
		expect(moveZeroes([0])).toStrictEqual([0])
	})
}
