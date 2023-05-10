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

if (import.meta.vitest) {
  const { test, expect } = import.meta.vitest

  test('one', () => {
    let str = ['h', 'e', 'l', 'l', 'o']

    reverseString(str)

    expect(str).toStrictEqual(['o', 'l', 'l', 'e', 'h'])
  })

  test('two', () => {
    let str = ['H', 'a', 'n', 'n', 'a', 'h']

    reverseString(str)

    expect(str).toStrictEqual(['h', 'a', 'n', 'n', 'a', 'H'])
  })
}
