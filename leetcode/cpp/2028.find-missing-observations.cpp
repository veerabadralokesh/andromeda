class Solution {
public:
    vector<int> missingRolls(vector<int>& rolls, int mean, int n) {
        int rem = (n + rolls.size()) * mean;
        for (const int roll: rolls) {
            rem -= roll;
        }
        if (rem > n * 6 || rem < n) {
            return {};
        }
        vector<int> ans(n, rem/n);
        rem = rem % n;
        for (int i = 0; i < rem; i++) {
            ans[i]++;
        }
        return ans;
    }
};

