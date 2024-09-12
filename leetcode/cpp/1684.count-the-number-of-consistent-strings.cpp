class Solution {
public:
    int countConsistentStrings(string allowed, vector<string>& words) {
        vector<bool> dict(26, false);
        for (const char c: allowed) {
            dict[c - (int)'a'] = true;
        }
        int ans = words.size();
        for (const string word: words) {
            for (const char c: word) {
                if (!dict[c - (int)'a']) {
                    ans--;
                    break;
                }
            }
        }
        return ans;            
    }
};

