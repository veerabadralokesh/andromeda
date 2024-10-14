class Solution {
public:
    long long maxKelements(vector<int>& nums, int k) {
        priority_queue<int> maxHeap;
        for (const int n: nums) {
            maxHeap.emplace(n);
        }
        long ans = 0;
        for (int i=0; i < k; ++i) {
            int maxNum = maxHeap.top();
            maxHeap.pop();
            ans += maxNum;
            maxHeap.emplace((maxNum+2)/3);
        }
        return ans;
    }
};

