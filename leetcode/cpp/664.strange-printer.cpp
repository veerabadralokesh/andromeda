class Solution {
public:
    int strangePrinter(string s) {
        int n = s.length();
        if (n == 0) {
            return 0;
        }
        vector<vector<int>> dp(n, vector<int>(n, n));
        for (int i = 0; i < n; i++) {
            dp[i][i] = 1;
        }

        for (int j = 0; j < n ; j++) {
            for (int i = j; i > -1; i--) {
                for (int k = i; k < j; k++) {
                    dp[i][j] = min(dp[i][j], dp[i][k] + dp[k+1][j] - (s[j] == s[k]));
                }
            }
        }
        return dp[0][n-1];
    }
};

/* */

class Solution {
public:
    int strangePrinter(string s) {
        int n = s.length();
        if (n == 0) {
            return 0;
        }
        vector<vector<int>> dp(n, vector<int>(n, 0));
        return strangePointer(s, 0, n-1, dp);
    }
private:
    int strangePointer(const string& s, int i, int j, vector<vector<int>>& dp) {
        if (i > j) {
            return 0;
        }
        if (dp[i][j] == 0) {
            dp[i][j] = strangePointer(s, i+1, j, dp) + 1;

            for (int k=i+1; k <= j; k++) {
                if (s[i] == s[k]) {
                    dp[i][j] = min(dp[i][j], 
                        strangePointer(s, i, k-1, dp) + strangePointer(s, k+1, j, dp)
                    );
                }
            }
        }
        return dp[i][j];
    }
};

