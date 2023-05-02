export function uniqueOccurrences(arr: number[]): boolean {
  const map: Map<number, number> = new Map()

  arr.forEach((a) => {
    if (map.has(a)) {
      map.set(a, map.get(a)! + 1)
    } else {
      map.set(a, 1)
    }
  })

  const set: Set<number> = new Set()

  for (const v of map.values()) {
    set.add(v)
  }

  let sum = 0

  set.forEach((s) => {
    sum += s
  })

  return sum === arr.length
}

if (import.meta.vitest) {
  const { test, expect } = import.meta.vitest
  test('test one', () => {
    expect(uniqueOccurrences([1, 2, 2, 1, 1, 3])).toBeTruthy()
  })
  test('test one', () => {
    expect(uniqueOccurrences([1, 2])).toBeFalsy()
  })
}
