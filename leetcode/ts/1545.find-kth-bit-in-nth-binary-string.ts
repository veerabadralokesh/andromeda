function findKthBit(n: number, k: number): string {
    if (n == 1) {
        return '0';
    }
    let midIndex = 1 << (n-1);
    if (midIndex == k)  {
        return '1';
    }
    if (k < midIndex) {
        return findKthBit(n-1, k);
    }
    if (findKthBit(n-1, 2 * midIndex - k) == '0') {
        return '1';
    }
    return '0';
};

