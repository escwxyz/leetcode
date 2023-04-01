//todo
export const shortestToChar = (s: string, c: string): number[] => {
	let arr: number[] = []

	let fast: number = 0
	let slow: number = 0
	let prev :number = 0
	for (let i = 0; i < s.length; i++) {
		if (s.charAt(fast) === c) {
			prev = fast
			while (slow <= fast) {
				arr.push(Math.min(Math.abs(prev - slow), Math.abs(fast - slow)))
				slow++
			}
		}
		fast++
	}

	return arr
}
