/**
 *
 * Problem: https://leetcode.com/problems/filter-elements-from-array/description/
 *
 * @param arr - array
 * @param fn - filter function
 * @returns filtered array
 */
export const filter = (
	arr: number[],
	fn: (n: number, i: number) => any
): number[] => {
	const result: Array<number> = []
	for (let i = 0; i < arr.length; i++) {
		if (fn(arr[i], i)) {
			result.push(arr[i])
		}
	}

	return result
}

if (import.meta.vitest) {
	const { it, expect } = import.meta.vitest

	it('test one', () => {
		expect(filter([0, 10, 20, 30], (n, _) => n > 10)).toStrictEqual([20, 30])
	})
	it('test two', () => {
		expect(filter([1, 2, 3], (_, i) => i === 0)).toStrictEqual([1])
	})
	it('test three', () => {
		expect(filter([-2, -1, 0, 1, 2], (n, _i) => n + 1)).toStrictEqual([
			-2, 0, 1, 2,
		])
	})
}
