/**
 * @param A - Array A
 * @param m - Length of Array's visible elements
 * @param B - Array B
 * @param n - Length of Array B
 *
 * Practice:
 * https://www.nowcoder.com/practice/89865d4375634fc484f3a24b7fe65665
 */
export const merge = (A: number[], m: number, B: number[], n: number): void => {
	let p = m + n - 1
	let p1 = m - 1
	let p2 = n - 1

	while (p1 >= 0 && p2 >= 0) {
		if (A[p1] > B[p2]) {
			A[p] = A[p1--]
		} else {
			A[p] = B[p2--]
		}
		p--
	}

	while (p1 >= 0) {
		A[p--] = A[p1--]
	}

	while (p2 >= 0) {
		A[p--] = B[p2--]
	}
}
