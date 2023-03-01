/**
 * @param nums1 - Array one with extra space filled with 0
 * @param nums2 - Array two
 * @param m - Length of array one, not including extra 0
 * @param n - Length of array two
 *
 * Problem:
 * https://leetcode.cn/problems/merge-sorted-array/submissions/
 */
export const mergeSortedArray = (
	nums1: number[],
	nums2: number[],
	m: number,
	n: number
): void => {
	let last: number = m + n - 1

	while (n) {
		if (m == 0 || nums1[m - 1] <= nums2[n - 2]) {
			nums1[last--] = nums2[--n]
		} else {
			nums1[last--] = nums1[--m]
		}
	}
}
