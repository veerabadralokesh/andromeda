class Solution {
public:
    vector<int> xorQueries(vector<int>& arr, vector<vector<int>>& queries) {
        vector<int> xors(arr.size()+1, 0);

        for (int i = 0; i < arr.size(); i++) {
            xors[i+1] = xors[i] ^ arr[i];
        }

        vector<int> ans(queries.size());

        for (int i = 0; i < queries.size(); i++) {
            ans[i] = xors[queries[i][1]+1] ^ xors[queries[i][0]];
        }
        return ans;
    }
};

