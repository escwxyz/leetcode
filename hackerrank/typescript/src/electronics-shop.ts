export const getMoneySpent = (
	keyboards: number[],
	drives: number[],
	b: number
): number => {
	const r = keyboards
		.filter((v) => v < b)
		.map((v) => {
			const d = drives.filter((l) => l + v <= b)
			if (d.length == 0) {
				return 0
			} else {
				return v + Math.max.apply(null, d)
			}
		})

	return r.every((v) => v == 0) ? -1 : Math.max.apply(null, r)
}

if (import.meta.vitest) {
	const { it, expect } = import.meta.vitest
	it('Non -1', () => {
		expect(getMoneySpent([5, 2, 8], [3, 1], 10)).toBe(9)
	})

	it('-1', () => {
		expect(getMoneySpent([5], [4], 5)).toBe(-1)
	})
}
