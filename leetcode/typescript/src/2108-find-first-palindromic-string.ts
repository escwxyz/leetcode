export const firstPalindrome = (words: string[]): string => {
	const a = words.find((value) => isPalindrome(value))

	return a || ''
}

const isPalindrome = (str: string): boolean => {
	let left = 0
	let right = str.length - 1

	while (left < right) {
		if (str.charAt(left) !== str.charAt(right)) {
			return false
		}
		left++
		right--
	}

	return true
}

if (import.meta.vitest) {
	const { it, expect } = import.meta.vitest

	it('true', () => {
		expect(firstPalindrome(['ada', 'fbj'])).toStrictEqual('ada')
	})

	it('false', () => {
		expect(firstPalindrome(['ado', 'fca'])).toStrictEqual('')
	})
}
