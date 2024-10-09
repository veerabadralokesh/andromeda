class Solution {
public:
    int minAddToMakeValid(string s) {
        int openCount = 0, closedCount = 0;
        for (const char c: s) {
            if (c == '(') {
                openCount++;
            } else if (openCount > 0) {
                openCount--;
            } else {
                closedCount++;
            }
        }
        return openCount + closedCount;
    }
};

