class Solution {
public:
    int lengthOfLastWord(string s) {
        int ans = 0;
        char prev=' ';
        for (char c: s) {
            if (c != ' ') {
                if (prev == ' ') {
                    ans = 1;
                } else {
                    ans++;
                }
            }
            prev = c;
        }
        return ans;
    }
};

