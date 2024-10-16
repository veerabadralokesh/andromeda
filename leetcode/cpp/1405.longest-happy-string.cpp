class Solution {
public:
    string longestDiverseString(int a, int b, int c, char A='a', char B='b', char C='c') {
        if (a < b) {
            return longestDiverseString(b, a, c, B, A, C);
        }
        if (b < c) {
            return longestDiverseString(a, c, b, A, C, B);
        }
        if (b == 0) {
            return string(min(a, 2), A);
        }
        int aCount = min(a, 2);
        int bCount = (a - aCount >= b) ? 1 : 0;
        return string(aCount, A) + string(bCount, B) + longestDiverseString(a-aCount, b-bCount, c, A, B, C);
    }
};

