class Solution {
    public char findKthBit(int n, int k) {
        if (n == 1) {
            return '0';
        }
        int midIndex = 1 << (n-1);
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
    }
}

