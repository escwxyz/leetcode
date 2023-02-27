/**
 * @param x - Position of cat A
 * @param y - Position of cat B
 * @param z - Position of mouse C
 *
 * Challenge:
 * https://www.hackerrank.com/challenges/cats-and-a-mouse/problem
 */

export const catAndMouse = (x: number, y: number, z: number): string => {
	const d1 = Math.abs(x - z)
	const d2 = Math.abs(y - z)

	if (d1 > d2) {
		return 'Cat B'
	} else if (d1 < d2) {
		return 'Cat A'
	} else {
		return 'Mouse C'
	}
}

if (import.meta.vitest) {
	const { it, expect } = import.meta.vitest

	it('Cat A', () => {
		expect(catAndMouse(1, 6, 2)).toBe('Cat A')
	})

	it('Cat B', () => {
		expect(catAndMouse(1, 2, 3)).toBe('Cat B')
	})

	it('Mouse C', () => {
		expect(catAndMouse(1, 3, 2)).toBe('Mouse C')
	})
}
