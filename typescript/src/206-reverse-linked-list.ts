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
 * Problem: https://leetcode.com/problems/reverse-linked-list/
 * @param head - linked list
 * @returns reversed linked list
 */
export function reverseList(head: ListNode | null): ListNode | null {
  if (head == null || head.next == null) return head

  const last = reverseList(head.next)
  head.next.next = head
  head.next = null
  return last
}

if (import.meta.vitest) {
  const { test, expect } = import.meta.vitest
  test('test one', () => {
    expect(
      reverseList(
        new ListNode(1, new ListNode(2, new ListNode(3, new ListNode(4))))
      )
    ).toStrictEqual(
      new ListNode(4, new ListNode(3, new ListNode(2, new ListNode(1))))
    )
  })

  test('test two', () => {
    expect(reverseList(new ListNode(1))).toStrictEqual(new ListNode(1))
  })
}
