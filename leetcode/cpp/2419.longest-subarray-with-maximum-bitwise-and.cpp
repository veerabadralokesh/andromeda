class Solution {
public:
    int longestSubarray(vector<int>& nums) {
        int maxn = nums[0];
        for (const int n: nums) {
            maxn = max(maxn, n);
        }
        int count = 0;
        int ans = 1;
        for (const int n: nums) {
            if (n == maxn) {
                count++;
                ans = max(ans, count);
            } else {
                count = 0;
            }
        }
        return ans;
    }
};

