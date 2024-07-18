type IntHeap []int
func (h IntHeap) Len() int           { return len(h) }
func (h IntHeap) Less(i, j int) bool { return h[i] > h[j] }
func (h IntHeap) Swap(i, j int)      { h[i], h[j] = h[j], h[i] }
func (h *IntHeap) Push(x interface{}) { *h = append(*h, x.(int)) }
func (h *IntHeap) Pop() interface{} {
	old := *h
	n := len(old)
	x := old[n-1]
	*h = old[0 : n-1]
	return x
}

func minimumDeviation(nums []int) int {
    n, minNum, deviation := len(nums), math.MaxInt32, math.MaxInt32
    h := make(IntHeap, n)
    for i := range nums {
        if nums[i] % 2 == 0 {
            h[i] = nums[i]
        } else {
            h[i] = nums[i] * 2
        }
        minNum = min(minNum, h[i])
    }
	heap.Init(&h)
    for {
        maxNum := heap.Pop(&h).(int)
        deviation = min(deviation, maxNum - minNum)
        if maxNum % 2 == 1 {
            break
        }
        maxNum >>= 1
        heap.Push(&h, maxNum)
        minNum = min(minNum, maxNum)
    }
    return deviation
}


/* */

// LEARN

func Max(a, b int) int {
	if a > b {
		return a
	}
	return b
}

func Min(a, b int) int {
	if a < b {
		return a
	}
	return b
}

func minimumDeviation(nums []int) int {

	// step 1. identify right and left poin
	left := 2147483647
	right := 0
	for i, num := range nums {
		if num%2 == 1 {
			doubled := num * 2
			nums[i] = doubled
			right = Max(num, right)
			left = Min(left, doubled)
		} else {
			left = Min(left, num)
			// get the highest odd of the even number
			for divided := num; divided > right; divided /= 2 {
				if divided%2 == 1 {
					right = Max(divided, right)
					break
				}
			}
		}
	}

	// no even numbers left to midpoint, so left = right
	left = Min(left, right)

	// iteration to exclude values within a range
	// iand save both even and even/2 out-of range
	high := []int{right}
	low := []int{left}
	for _, num := range nums {
		var even int
		for ; num >= left; num /= 2 {
			if num <= right {
				even = 0
				break
			} else {
				even = num
			}
		}
		// note after "for" num is even/2 :  num ..left.. middle ... even

		if even == 0 { //withing a range, skip
			continue
		}

		high = append(high, even)
		low = append(low, num)

	}

	sort.Ints(high)
	sort.Ints(low)
	dev := 2147483647
	for i, highEven := range(high) {
		dev = Min(dev, highEven-low[i])
	}
	return dev
}


