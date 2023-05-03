export function largestAltitude(gain: number[]): number {
  const arr = new Array(gain.length + 1).fill(0)

  for (let i = 0; i < gain.length; i++) {
    arr[i + 1] = arr[i] + gain[i]
  }

  return Math.max.apply(null, arr)
}

if (import.meta.vitest) {
  const { test, expect } = import.meta.vitest
  test('test one', () => {
    expect(largestAltitude([-5, 1, 5, 0, -7])).toBe(1)
  })
  test('test two', () => {
    expect(largestAltitude([-4, -3, -2, -1, 4, 3, 2])).toBe(0)
  })
}
