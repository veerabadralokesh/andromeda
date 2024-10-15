class Solution {
public:
    long long minimumSteps(string s) {
        long ans = 0, ones = 0;
        for (const char c: s) {
            switch (c) {
                case '1':
                    ones++;
                    break;
                default:
                    ans += ones;
                    break;
            }
        }
        return ans;
    }
};

