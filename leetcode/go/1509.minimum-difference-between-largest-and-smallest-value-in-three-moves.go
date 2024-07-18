package main

import "slices"

func minDifference(nums []int) int {
	if len(nums) < 5 {
		return 0
	}
	slices.Sort(nums)

	left, right := 0, len(nums)-1
	return min(
		nums[right-3]-nums[left],
		min(
			nums[right-2]-nums[left+1],
			min(
				nums[right-1]-nums[left+2],
				nums[right]-nums[left+3],
			),
		),
	)
}

func min(a, b int) int {
	if a < b {
		return a
	}
	return b
}
