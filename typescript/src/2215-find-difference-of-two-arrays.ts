/**
 * @param nums1 - array one
 * @param nums2 - array two
 * @returns difference array
 *
 * Problem: https://leetcode.com/problems/find-the-difference-of-two-arrays/
 */
export function findDifference(nums1: number[], nums2: number[]): number[][] {
  const setOne: Set<number> = new Set()
  const setTwo: Set<number> = new Set()

  nums2.forEach((a) => {
    if (!nums1.includes(a)) {
      setOne.add(a)
    }
  })

  nums1.forEach((a) => {
    if (!nums2.includes(a)) {
      setTwo.add(a)
    }
  })

  return [[...setTwo], [...setOne]]
}

if (import.meta.vitest) {
  const { test, expect } = import.meta.vitest

  test('one', () => {
    expect(findDifference([1, 2, 3], [2, 4, 6])).toStrictEqual([
      [1, 3],
      [4, 6],
    ])
  })
  test('two', () => {
    expect(findDifference([1, 2, 3, 3], [1, 1, 2, 2])).toStrictEqual([[3], []])
  })
}
