// Definition for singly-linked list.
class ListNode {
  val: number
  next: ListNode | null
  constructor(val?: number, next?: ListNode | null) {
    this.val = val === undefined ? 0 : val
    this.next = next === undefined ? null : next
  }
}

/**
 * @param head - linked list
 * @returns sorted linked list
 *
 * Problem: https://leetcode.com/problems/odd-even-linked-list/description
 */
export function oddEvenList(head: ListNode | null): ListNode | null {
  if (head == null || head.next == null) return head

  const odd = new ListNode(-1)
  const even = new ListNode(-1)

  let p1 = odd
  let p2 = even

  let p: ListNode | null = head
  let index = 1

  while (p != null) {
    if (index % 2 != 0) {
      p1.next = p
      p1 = p1.next
    } else {
      p2.next = p
      p2 = p2.next
    }
    const tmp: ListNode | null = p.next
    p.next = null
    p = tmp
    index++
  }

  p1.next = even.next

  return odd.next
}

if (import.meta.vitest) {
  const { test, expect } = import.meta.vitest
  test('test one', () => {
    expect(
      oddEvenList(
        new ListNode(
          1,
          new ListNode(2, new ListNode(3, new ListNode(4, new ListNode(5))))
        )
      )
    ).toStrictEqual(
      new ListNode(
        1,
        new ListNode(3, new ListNode(5, new ListNode(2, new ListNode(4))))
      )
    )
  })
}
