class Solution {
public:
    unordered_map<int, int> map;
    vector<int> twoSum(vector<int>& nums, int target) {
        int rem = 0, n = 0, i=0;
        vector<int> ans;
        for (; i< nums.size();i++) {
            n = nums[i];
            if (map.find(n) != map.end()) {
                ans.push_back(i);
                ans.push_back(map[n]);
                break;
            }
            rem = target - n;
            map[rem] = i;
        }
        return ans;
    }
};

/* */

class Solution {
public:
    vector<int> twoSum(vector<int>& nums, int target) {
        unordered_map<int,int> map;
        int n = nums.size();

        for(int i=0;i<n;i++){
            int complement = target - nums[i];
            if(map.count(complement)){
                return {map[complement],i};
            }
            map[nums[i]] = i;
        }

        return {};
    }
};

/* */

class Solution {
 public:
  vector<int> twoSum(vector<int>& nums, int target) {
    unordered_map<int, int> numToIndex;

    for (int i = 0; i < nums.size(); ++i) {
      if (const auto it = numToIndex.find(target - nums[i]);
          it != numToIndex.cend())
        return {it->second, i};
      numToIndex[nums[i]] = i;
    }

    throw;
  }
};

