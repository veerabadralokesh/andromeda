class Solution {
public:
    int findTheLongestSubstring(string s) {
        int mask = 0;
        unordered_map<int, int> prefix_index = {
            {0, -1}
        };
        string vowels = "aeiou";
        int i = 0;
        int ans = 0;
        for (const char c: s) {
            if (vowels.find(c) != std::string::npos) {
                mask ^= (1 << (c - (int)'a'));
            }
            if (prefix_index.find(mask) != prefix_index.cend()) {
                ans = max(ans, i - prefix_index[mask]);
            } else {
                prefix_index[mask] = i;
            }
            i++;
        }
        return ans;
    }
};

