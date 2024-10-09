class Solution {
public:
    long long minIncrementOperations(vector<int>& nums, int k) {
        long dp = 0, p1 = 0, p2 = 0, p3 = 0;
        for (const int n: nums) {
            dp = min(p1, min(p2, p3)) + max(0, k - n);
            p3 = p2;
            p2 = p1;
            p1 = dp;
        }
        return min(p1, min(p2, p3));
    }
};

