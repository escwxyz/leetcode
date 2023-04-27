declare global {
  interface Array<T> {
    last(): T | -1
  }
}
Array.prototype.last = function () {
  return this[this.length - 1] ?? -1
}

if (import.meta.vitest) {
  const { test, expect } = import.meta.vitest
  test('test one', () => {
    expect([1, 2, 3].last()).toBe(3)
  })
  test('test two', () => {
    expect([1, 2, 3].last()).toBe(3)
  })
  test('test three', () => {
    expect([0, 0, 0].last()).toBe(0)
  })
}

export {}
