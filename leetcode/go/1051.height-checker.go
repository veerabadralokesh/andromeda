import (
	"sort"
)

func heightChecker(heights []int) int {
	expected := make([]int, len(heights))
	copy(expected, heights)
	sort.Ints(expected)
	ans := 0
	for i, v := range heights {
		if v != expected[i] {
			ans++
		}
	}
	return ans
}

/* */

func heightChecker(heights []int) int {
	var res int
	expected := make([]int, len(heights))
	copy(expected, heights)
	slices.Sort(expected)
	for i := 0; i < len(heights); i++ {
		if heights[i] != expected[i] {
			res++
		}
	}
	return res
}
