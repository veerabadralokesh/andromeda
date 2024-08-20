class Solution {
public:
    vector<int> sortArray(vector<int>& nums) {
        vector<int> counts(100001, 0);
        for (int n: nums) {
            counts[50000+n]++;
        }
        vector<int> ans(nums.size());
        int k = 0;
        for (int i = 0; i < 100001 && k < nums.size(); i++) {
            for(int j = 0; j < counts[i]; j++) {
                ans[k] = i - 50000;
                k++;
            }
        }
        return ans;
    }
};

