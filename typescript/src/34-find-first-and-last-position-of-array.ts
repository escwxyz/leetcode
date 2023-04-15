/**
 * @param nums - array of numbers
 * @param target - target number
 * @returns index range of target number
 *
 * Problem: https://leetcode.com/problems/find-first-and-last-position-of-element-in-sorted-array/
 */
export const searchRange = (nums: number[], target: number): number[] => {
	if (nums.length === 0 || !nums.includes(target)) return [-1, -1]

	let index = nums.indexOf(target)

	let left = index
	let right = index
	let start = left
	let end = right

	if (left === 0) {
		while (right < nums.length) {
			if (nums[++right] === target) {
				end = right
			}
		}
		return [0, end]
	}

	if (right === nums.length - 1) {
		while (left > 0) {
			if (nums[--left] === target) {
				start = left
			}
		}

		return [start, nums.length - 1]
	}

	while (left >= 0 && right <= nums.length - 1) {
		if (nums[--left] === target) {
			start = left
		}

		if (nums[++right] === target) {
			end = right
		}
	}

	return [start, end]
}

if (import.meta.vitest) {
	const { it, expect } = import.meta.vitest

	it('test one', () => {
		expect(searchRange([5, 7, 7, 8, 8, 10], 8)).toStrictEqual([3, 4])
	})

	it('test two', () => {
		expect(searchRange([5, 7, 7, 8, 8, 10], 6)).toStrictEqual([-1, -1])
	})
	it('test three', () => {
		expect(searchRange([], 0)).toStrictEqual([-1, -1])
	})

	it('test four', () => {
		expect(searchRange([1, 2, 3, 3, 3, 3, 4, 5, 9], 3)).toStrictEqual([2, 5])
	})
}
