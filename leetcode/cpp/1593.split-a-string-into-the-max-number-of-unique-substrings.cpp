class Solution {
public:
    int maxUniqueSplit(string s) {
        size_t ans = 0;
        unordered_set<string> set;
        backtrack(0, std::move(set), s, ans);
        return ans;
    }
private:
    void backtrack(int i, unordered_set<string>&& set, const string& s, size_t& ans) {
        if (i == s.length()) {
            ans = max(ans, set.size());
            return;
        }
        for (int j = 1; j <= s.length()-i; j++) {
            const string substr = s.substr(i, j);
            if (set.contains(substr)) {
                continue;
            }
            set.insert(substr);
            backtrack(i + j, std::move(set), s, ans);
            set.erase(substr);
        }
    }
};

