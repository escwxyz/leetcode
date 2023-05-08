/**
 * @param word1 - string one
 * @param word2 - string two
 * @returns boolean, if string one and string two are close
 *
 * Problem: https://leetcode.com/problems/determine-if-two-strings-are-close
 */
// TODO: improve
export function closeStrings(word1: string, word2: string): boolean {
  if (word1.length !== word2.length) return false

  const mapOne: Map<string, number> = new Map()

  word1.split('').forEach((s) => {
    if (mapOne.has(s)) {
      mapOne.set(s, mapOne.get(s)! + 1)
    } else {
      mapOne.set(s, 1)
    }
  })

  const arrOne = []

  for (const v of mapOne.values()) {
    arrOne.push(v)
  }

  const mapTwo: Map<string, number> = new Map()

  word2.split('').forEach((s) => {
    if (mapTwo.has(s)) {
      mapTwo.set(s, mapTwo.get(s)! + 1)
    } else {
      mapTwo.set(s, 1)
    }
  })

  const arrTwo = []

  for (const v of mapTwo.values()) {
    arrTwo.push(v)
  }

  if (arrOne.length != arrTwo.length) return false

  arrOne.sort((a, b) => a - b)

  arrTwo.sort((a, b) => a - b)

  for (let i = 0; i < arrOne.length; i++) {
    if (arrOne[i] != arrTwo[i]) {
      return false
    }
  }

  const resultOne = word2.split('').every((s) => mapOne.has(s))

  const resultTwo = word1.split('').every((s) => mapTwo.has(s))

  return resultTwo && resultOne
}

if (import.meta.vitest) {
  const { test, expect } = import.meta.vitest
  test('test one', () => {
    expect(closeStrings('abc', 'bca')).toBeTruthy()
  })
  test('test two', () => {
    expect(closeStrings('aa', 'a')).toBeFalsy()
  })
  test('test three', () => {
    expect(closeStrings('cabbba', 'abbccc')).toBeTruthy()
  })
  test('test four', () => {
    expect(closeStrings('cabbba', 'aabbss')).toBeFalsy()
  })
  test('test five', () => {
    expect(closeStrings('abbzzca', 'babzzcz')).toBeFalsy()
  })
  test('test six', () => {
    expect(
      closeStrings('aaabbbbccddeeeeefffff', 'aaaaabbcccdddeeeeffff')
    ).toBeFalsy()
  })
}
