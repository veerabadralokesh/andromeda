class Solution {
public:
    vector<int> arrayRankTransform(vector<int>& arr) {
        if (arr.size() == 0) {
            return {};
        }
        vector<int> arrSorted = arr;
        sort(arrSorted.begin(), arrSorted.end());
        
        unordered_map<int, int> ranks;
        ranks[arrSorted[0]] = 1;
        int rank = 2;
        for (int i = 1; i < arrSorted.size(); i++) {
            if (arrSorted[i] != arrSorted[i-1]) {
                ranks[arrSorted[i]] = rank;
                rank++;
            }
        }

        vector<int> ans(arr.size());
        for (int i=0; i < arr.size(); i++) {
            ans[i] = ranks[arr[i]];
        }
        return ans;
    }
};

