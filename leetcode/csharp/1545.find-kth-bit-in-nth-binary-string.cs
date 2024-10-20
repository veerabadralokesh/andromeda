public class Solution {
    public char FindKthBit(int n, int k) {
        if (n == 1) {
            return '0';
        }
        int midIndex = 1 << (n-1);
        if (midIndex == k)  {
            return '1';
        }
        if (k < midIndex) {
            return FindKthBit(n-1, k);
        }
        if (FindKthBit(n-1, 2 * midIndex - k) == '0') {
            return '1';
        }
        return '0';
    }
}

