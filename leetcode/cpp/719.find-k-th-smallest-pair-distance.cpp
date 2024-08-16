class Solution {
public:
    int smallestDistancePair(vector<int>& nums, int k) {
        
        int n = nums.size();
        sort(nums.begin(), nums.end());

        auto is_valid = [&](int distance) {
            int count = 0, i = 0, j = 0;
            while (i < n || j < n) {
                while (j < n && nums[j]-nums[i] <= distance) {
                    j++;
                }
                count += (j - i - 1);
                i++;
            }
            return count >= k;
        };

        int left = 0, right = nums[n-1]-nums[0], mid =0;

        while (left < right) {
            mid = left + (right - left)/2;
            if (is_valid(mid)) {
                right = mid;
            } else {
                left = mid+1;
            }
        }

        return left;
    }
};

