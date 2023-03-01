export const mergeAlternately = (word1: string, word2: string): string => {
	let str = ''
	let p1 = 0
	let p2 = 0

	while (p1 < word1.length && p2 < word2.length) {
		str = str + word1.charAt(p1++) + word2.charAt(p2++)
	}

	while (p1 < word1.length) {
		str += word1.charAt(p1++)
	}

	while (p2 < word2.length) {
		str += word2.charAt(p2++)
	}

	return str
}

if (import.meta.vitest) {
	const { it, expect } = import.meta.vitest

	it('test', () => {
		expect(mergeAlternately('abc', 'pqr')).toStrictEqual('apbqcr')
	})
}
