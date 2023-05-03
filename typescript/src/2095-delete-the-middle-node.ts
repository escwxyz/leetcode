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
 * @param head - list node
 * @returns list node after middle is deleted
 *
 * Problem: https://leetcode.com/problems/delete-the-middle-node-of-a-linked-list
 */
export function deleteMiddle(head: ListNode | null): ListNode | null {
  if (head == null || head.next == null) return null

  let p: ListNode | null = head
  let middle = findMiddle(head)
  while (p != null) {
    if (p.next == middle) {
      p.next = p.next ? p.next.next : null
    }
    p = p.next
  }

  return head
}

function findMiddle(head: ListNode | null): ListNode | null {
  let slow = head
  let fast = head

  while (fast != null && fast.next != null) {
    slow = slow ? slow.next : null
    fast = fast.next.next
  }

  return slow
}

if (import.meta.vitest) {
  const { test, expect } = import.meta.vitest
  test('test one', () => {
    expect(
      deleteMiddle(
        new ListNode(1, new ListNode(2, new ListNode(3, new ListNode(4))))
      )
    ).toStrictEqual(new ListNode(1, new ListNode(2, new ListNode(4))))
  })
}
