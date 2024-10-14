struct Entry {
    int num;
    int i;
    int j;
};

struct CompareEntry {
    bool operator()(const Entry& a, const Entry& b) {
        return a.num > b.num;
    }
};

class Solution {
public:
    vector<int> smallestRange(vector<vector<int>>& nums) {
        // auto compare = [&](const Entry& a, const Entry& b) {
        //     return a.num > b.num;
        // };
        priority_queue<Entry, vector<Entry>, CompareEntry> minHeap;
        int mn = INT_MAX;
        int mx = INT_MIN;

        for(int i=0; i < nums.size(); i++) {
            const int num = nums[i][0];

            minHeap.emplace(num, i, 0);
            mn = min(mn, num);
            mx = max(mx, num);
        }

        int minRange = mn;
        int maxRange = mx;

        while (minHeap.size() == nums.size()) {
            const auto [_, i, j] = minHeap.top();
            minHeap.pop();
            if (j + 1 < nums[i].size()) {
                minHeap.emplace(nums[i][j+1], i, j+1);
                mx = max(mx, nums[i][j+1]);
                mn = minHeap.top().num;
                if (mx - mn < maxRange - minRange) {
                    maxRange = mx;
                    minRange = mn;
                }
            }
        }

        return {minRange, maxRange};
    }
};

