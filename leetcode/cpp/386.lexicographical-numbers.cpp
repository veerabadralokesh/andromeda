class Solution {
public:
    vector<int> lexicalOrder(int n) {
        vector<int> ans;
        ans.reserve(n+1);
        for(int i = 1; i < 10 && i <= n; i++ ) {
            genNums(i, n, ans);
        }
        return ans;
    }
private:
    void genNums(int x, int n, vector<int>& ans) {
        ans.push_back(x);
        for(int y=x * 10; y < x * 10 + 10 && y <= n; y++) {
            genNums(y, n, ans);
        }
    }
};

