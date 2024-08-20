class Solution {
public:
    int stoneGameII(vector<int>& piles) {
        int n = piles.size();
        vector<vector<int>> memo(n, vector<int>(n));
        vector<int> suffix = piles;
        for (int i=n-2; i > -1; i--) {
            suffix[i] += suffix[i+1];
        }
        return stoneGameII(memo, suffix, 0, 1);
    }
private:
    int stoneGameII(vector<vector<int>>& memo, vector<int>& suffix, int i, int m) {
        if (i + 2 * m >= suffix.size()) {
            return suffix[i];
        }
        if (memo[i][m] == 0) {
            int bob = suffix[i];
            for (int X=1; X <= 2 * m; X++) {
                bob = min(bob, stoneGameII(memo, suffix, i + X, max(m, X)));
            }
            memo[i][m] = suffix[i] - bob;
        }
        return memo[i][m];
    }
};

