/**
 * @param nums1 - Array one with extra space filled with 0
 * @param nums2 - Array two
 * @param m - Length of array one, not including extra 0
 * @param n - Length of array two
 *
 * Problem 88:
 * https://leetcode.com/problems/merge-sorted-array/submissions/
 */
export const mergeSortedArray = (
	nums1: number[],
	nums2: number[],
	m: number,
	n: number
): void => {
	let p = m + n - 1
	let p1 = m - 1
	let p2 = n - 1

	while (p1 >= 0 && p2 >= 0 && p1 <= p) {
		if (nums1[p1] < nums2[p2]) {
			nums1[p] = nums2[p2--]
		} else {
			nums1[p] = nums1[p1--]
		}
		p--
	}
	if (p2 >= 0) {
		for (let i = 0; i <= p2; i++) {
			nums1[i] = nums2[i]
		}
	}
}
