type KthLargest struct {
    maxHeap IntHeap
    k int
}


func Constructor(k int, nums []int) KthLargest {
    maxHeap := make(IntHeap, 0, len(nums))
    heap.Init(&maxHeap)
    for _, n := range nums {
        heap.Push(&maxHeap, -n)
    }
    for k < maxHeap.Len() {
        fmt.Println(heap.Pop(&maxHeap).(int))
    }
    return KthLargest {
        maxHeap: maxHeap,
        k: k,
    }
}


func (this *KthLargest) Add(val int) int {
    heap.Push(&this.maxHeap, -val)
    if this.k < this.maxHeap.Len() {
        heap.Pop(&this.maxHeap)
    }
    return -this.maxHeap.Peek()
}

type IntHeap []int
func (h IntHeap) Len() int           { return len(h) }
func (h IntHeap) Peek() int           { return h[0] }
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


/**
 * Your KthLargest object will be instantiated and called as such:
 * obj := Constructor(k, nums);
 * param_1 := obj.Add(val);
 */

