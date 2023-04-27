class ListNode {
  val: number
  next: ListNode | null
  constructor(val?: number, next?: ListNode | null) {
    this.val = val === undefined ? 0 : val
    this.next = next === undefined ? null : next
  }
}

export function mergeTwoLists(
  list1: ListNode | null,
  list2: ListNode | null
): ListNode | null {
  if (!list1 && !list2) return null

  if (!list1 && list2) return list2

  if (list1 && !list2) return list1

  let p1 = list1
  let p2 = list2

  let result = new ListNode(-1)

  let p = result

  while (p1 !== null && p2 !== null) {
    if (p1.val > p2.val) {
      p.next = p2
      p2 = p2.next
    } else {
      p.next = p1
      p1 = p1.next
    }
    p = p.next
  }

  if (p1 !== null) {
    p.next = p1
  }

  if (p2 !== null) {
    p.next = p2
  }

  return result.next
}

if (import.meta.vitest) {
  const { test, expect } = import.meta.vitest
  test('test one', () => {
    const list1 = new ListNode(1, new ListNode(2, new ListNode(4)))
    const list2 = new ListNode(1, new ListNode(3, new ListNode(4)))
    expect(mergeTwoLists(list1, list2)).toStrictEqual(
      new ListNode(
        1,
        new ListNode(
          1,
          new ListNode(2, new ListNode(3, new ListNode(4, new ListNode(4))))
        )
      )
    )
  })
  test('test two', () => {
    expect(mergeTwoLists(null, null)).toBeNull()
  })
}
