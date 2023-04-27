export const strStr = (haystack: string, needle: string): number => {
  if (haystack.length < needle.length) return -1

  let index = -1

  for (let i = 0; i < haystack.length; i++) {
    let j = 0

    while (j < needle.length) {
      if (needle.charAt(j) === haystack.charAt(i + j)) {
        j++
      } else {
        break
      }
    }

    if (j === needle.length) {
      index = i
      break
    }
  }

  return index
}

if (import.meta.vitest) {
  const { test, expect } = import.meta.vitest
  test('test one', () => {
    expect(strStr('sadbutsad', 'sad')).toBe(0)
  })

  test('test two', () => {
    expect(strStr('leetcode', 'leeto')).toBe(-1)
  })
}
