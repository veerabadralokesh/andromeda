class Solution {
public:
    int minSubarray(vector<int>& nums, int p) {
        long int sum = 0;
        for (const int n: nums) {
            sum += n;
        }
        long int rem = sum % p;
        if (rem == 0) {
            return 0;
        }
        long int prefix = 0, target = 0;
        unordered_map<long int, int> idxs;
        idxs[0] = -1;

        int ans = nums.size();

        for (int i = 0; i < nums.size(); i++) {
            prefix = (prefix + nums[i]) % p;
            target = (prefix - rem + p) % p;
            if (idxs.find(target) != idxs.cend()) {
                ans = min(ans, i - idxs[target]);
            }
            idxs[prefix] = i;
        }

        return ans == nums.size() ? -1 : ans;
    }
};

