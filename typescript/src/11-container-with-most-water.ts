/**
 * @param height - Array of height
 * @returns max water area
 *
 *Problem: https://leetcode.com/problems/container-with-most-water/description/
 * */
export const maxArea = (height: number[]): number => {
	let left = 0
	let right = height.length - 1
	let maxArea = 0

	while (left < right) {
		maxArea = Math.max(
			maxArea,
			(right - left) * Math.min(height[left], height[right])
		)
		if (height[left] < height[right]) {
			left++
		} else {
			right--
		}
	}

	return maxArea
}

if (import.meta.vitest) {
	const { test, expect } = import.meta.vitest

	test('one', () => {
		expect(maxArea([1, 8, 6, 2, 5, 4, 8, 3, 7])).toBe(49)
	})
	test('two', () => {
		expect(maxArea([1, 1])).toBe(1)
	})
	test('two', () => {
		expect(maxArea([2, 3, 4, 5, 18, 17, 6])).toBe(17)
	})
}
