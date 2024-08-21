class Solution {
public:
    int arrayNesting(vector<int>& nums) {
        int n = nums.size();
        vector<bool> visited(n, false);
        int ans = 0;
        for (int i=0; i < n; i++) {
            if (!visited[i]) {
                int setLength = 0;
                int j = i;
                while (!visited[j]) {
                    visited[j] = true;
                    j = nums[j];
                    setLength++;
                    ans = max(ans, setLength);
                }
            }
        }
        return ans;
    }
};

