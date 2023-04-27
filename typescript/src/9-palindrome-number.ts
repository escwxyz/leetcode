export const isPalindrome = (x: number): boolean => {
  if (x < 0) return false
  const str = x.toString()

  let left = 0
  let right = str.length - 1

  while (left <= right) {
    if (str.charAt(left) === str.charAt(right)) {
      left++
      right--
    } else {
      return false
    }
  }

  return true
}

export const isPalindromeImproved = (x: number): boolean => {
  if (x < 0) return false

  let temp = x
  let y = 0
  while (temp > 0) {
    const last_num = temp % 10
    temp = Math.floor(temp / 10)
    y = y * 10 + last_num
  }
  return y == x
}

if (import.meta.vitest) {
  const { test, expect } = import.meta.vitest
  test('test one', () => {
    expect(isPalindrome(121)).toBe(true)
    expect(isPalindromeImproved(121)).toBe(true)
  })
  test('test two', () => {
    expect(isPalindrome(-121)).toBe(false)
    expect(isPalindromeImproved(-121)).toBe(false)
  })
  test('test three', () => {
    expect(isPalindrome(10)).toBe(false)
    expect(isPalindromeImproved(10)).toBe(false)
  })
}
