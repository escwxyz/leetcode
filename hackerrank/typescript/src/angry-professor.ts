export const angryProfessor = (k: number, a: number[]): string => {
	return a.filter((v) => v <= 0).length >= k ? 'NO' : 'YES'
}

if (import.meta.vitest) {
	const { it, expect } = import.meta.vitest
	it('Cancelled', () => {
		expect(angryProfessor(3, [-1, -3, 4, 2])).toBe('YES')
	})

	it('GO ON', () => {
		expect(angryProfessor(2, [0, -1, 2, 1])).toBe('NO')
	})
}
