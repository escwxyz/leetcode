/**
 * @param s - Array of string
 *
 * Problem 344:
 * https://leetcode.com/problems/reverse-string/
 */
export const reverseString = (s: string[]): void => {
	let head: number = 0
	let tail: number = s.length - 1

	while (head < tail) {
		const tmp = s[head]
		s[head++] = s[tail]
		s[tail--] = tmp
	}
}
