/**
 * @param nums - Array of integers
 * @param target - Target number
 * @returns Indice of two numbers whose sum is equal to target number
 *
 * Problem 1:
 * https://leetcode.com/problems/two-sum/
 */
export const twoSum = (nums: number[], target: number): number[] => {
	const arr = []
	const map = new Map()
	for (let i = 0; i < nums.length; i++) {
		const d = target - nums[i]
		if (map.has(d)) {
			arr.push(i, map.get(d))
		}
		map.set(nums[i], i)
	}
	return arr
}

if (import.meta.vitest) {
	const { test, expect } = import.meta.vitest
	test('test one', () => {
		expect(twoSum([2, 7, 11, 15], 9)).toStrictEqual([1, 0])
	})
	test('test two', () => {
		expect(twoSum([3, 2, 4], 6)).toStrictEqual([2, 1])
	})
	test('test three', () => {
		expect(twoSum([3, 3], 6)).toStrictEqual([1, 0])
	})
}
