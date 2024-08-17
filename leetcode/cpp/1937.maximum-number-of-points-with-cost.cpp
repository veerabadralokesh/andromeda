class Solution {
public:
    long long maxPoints(vector<vector<int>>& points) {
        int n = points[0].size(), m = points.size();
        vector<long long> dp;
        for (int i=0; i < n; i++) {dp.push_back(0);}
        long long maxi = 0;
        long long leftToRight[n];
        for (vector<int>& row: points) {
            maxi = 0;
            for (int j=0; j < n; j++) {
                maxi = max(maxi-1, dp[j]);
                leftToRight[j] = maxi;
            }
            maxi = 0;
            for (int j=n-1; j > -1; j--) {
                maxi = max(maxi-1, dp[j]);
                dp[j] = max(maxi, leftToRight[j]) + (long long)row[j];
            }
        }
        long long ans = 0;
        for (long long d: dp) {
            ans = max(ans, d);
        }
        return ans;
    }
};

