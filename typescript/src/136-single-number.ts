// TODO
export const singleNumber = (nums: number[]): number => {
	if (nums.length === 1) {
		return nums[0]
	}

	let slow: number = 0
	let fast: number = 1

	while (fast < nums.length) {
		if (nums[slow] === nums[fast]) {
			slow++
		}
		fast++
	}

	return nums[slow]
}
