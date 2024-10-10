class Solution {
public:
    int maxWidthRamp(vector<int>& nums) {
        vector<int> stack;
        for (int i=0; i < nums.size(); i++) {
            if (stack.empty() || nums[stack.back()] >= nums[i]) {
                stack.push_back(i);
            }
        }
        int ans = 0;
        for (int i = nums.size() - 1; i > ans; --i) {
            while (!stack.empty() && nums[i] >= nums[stack.back()])
                ans = max(ans, i - stack.back()), stack.pop_back();
        }
        return ans;
    }
};

