class Solution {
public:
    int nthUglyNumber(int n) {
        int dp[n];
        dp[0] = 1;
        int head2=0, head3=0, head5=0;
        for (int i=1; i < n; i++) {
            dp[i] = min({2*dp[head2], 3*dp[head3], 5*dp[head5]});
            if (dp[i] == 2 * dp[head2]) {
                head2++;
            }
            if (dp[i] == 3 * dp[head3]) {
                head3++;
            }
            if (dp[i] == 5 * dp[head5]) {
                head5++;
            }
        }
        return dp[n-1];
    }
};

