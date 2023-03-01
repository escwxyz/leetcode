export const sortedSquares = (nums: number[]): number[] => {
	let result = new Array(nums.length)
	let left = 0
	let right = nums.length - 1

	let index = right

	while (left <= right) {
		if (Math.abs(nums[left]) > Math.abs(nums[right])) {
			result[index] = nums[left] ** 2
			left++
		} else {
			result[index] = nums[right] ** 2
			right--
		}
		index--
	}

	return result
}

if (import.meta.vitest) {
	const { it, expect } = import.meta.vitest

	it('test', () => {
		expect(sortedSquares([-4, -1, -0, 3, 10])).toStrictEqual([0, 1, 9, 16, 100])
	})
}
