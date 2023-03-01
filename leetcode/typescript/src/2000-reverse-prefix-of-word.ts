/**
 * @param word - String
 * @param ch - Target char
 * @returns Modified string
 *
 * Problem:
 * https://leetcode.cn/problems/reverse-prefix-of-word/
 */

export const reversePrefix = (word: string, ch: string): string => {
	const index = word.indexOf(ch)

	if (index == -1) {
		return word
	}

	let left = 0
	let right = index
	let arr = word.slice(0, right + 1).split('')

	while (left < right) {
		const temp = arr[left]
		arr[left] = arr[right]
		arr[right] = temp
		left++
		right--
	}

	return arr.join('') + word.slice(index + 1)
}

if (import.meta.vitest) {
	const { it, expect } = import.meta.vitest

	it('NOP', () => {
		expect(reversePrefix('hello', 'z')).toStrictEqual('hello')
	})

	it('test', () => {
		expect(reversePrefix('hello', 'l')).toStrictEqual('lehlo')
	})
}
