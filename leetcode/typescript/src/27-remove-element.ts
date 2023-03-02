/**
 * @param nums - Array of integer
 * @param val - Value to be removed from array in place
 * @returns Length of the modified array
 *
 * Problem 27:
 * https://leetcode.com/problems/remove-element/
 */

export const removeElement = (nums: number[], val: number): number => {
	let slow: number = 0
	let fast: number = 0

	while (fast < nums.length) {
		if (nums[fast] !== val) {
			nums[slow++] = nums[fast]
		}
		fast++
	}

	return slow
}

if (import.meta.vitest) {
	const { it, expect } = import.meta.vitest

	it('test one', () => {
		expect(removeElement([0, 1, 2, 2, 3, 0, 4, 2], 2)).toBe(2)
	})

	it('test two', () => {
		expect(removeElement([3, 2, 2, 3], 3)).toBe(2)
	})
}
