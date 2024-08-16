class Solution {
public:
    int maxDistance(vector<vector<int>>& arrays) {
        // arrays already sorted
        int ans = 0;
        int a = -1000000;
        int b = 1000000;
        for (const vector<int>& arr: arrays) {
            ans = max({ans, arr.back() - b, a - arr.front()});
            b = min(b, arr.front());
            a = max(a, arr.back());
        }
        return ans;
    }
};

