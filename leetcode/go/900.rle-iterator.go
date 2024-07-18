type RLEIterator struct {
    encoding []int
    i int
}


func Constructor(encoding []int) RLEIterator {
    return RLEIterator {
        encoding: encoding,
        i: 0,
    }
}


func (this *RLEIterator) Next(n int) int {
    if this.i >= len(this.encoding) {
        return -1;
    }
    ans, count := this.encoding[this.i+1], 0
    for n > 0 {
        for this.encoding[this.i] == 0 {
            this.i += 2
            if this.i >= len(this.encoding) {
                return -1
            }
            ans = this.encoding[this.i+1]
        }
        count = min(n, this.encoding[this.i])
        this.encoding[this.i] -= count
        n -= count
    }
    return ans
}

func min(a, b int) int {
    if a < b {
        return a
    }
    return b
}

/**
 * Your RLEIterator object will be instantiated and called as such:
 * obj := Constructor(encoding);
 * param_1 := obj.Next(n);
 */

