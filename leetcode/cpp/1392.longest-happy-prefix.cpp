class Solution {
public:
    string longestPrefix(string s) {
        auto prefixLen = calcPrefixLen(s);
        return s.substr(0, prefixLen.back());
    }
private:
    // calculating KMP prefix arr
    vector<int> calcPrefixLen(string& pattern) {
        int patternLen = pattern.length();

        vector<int> arr(patternLen+1, 0);
        arr[0] = -1;

        int prefixLen = 0;
        int i = 1;

        while (i < patternLen) {
            if (pattern[i] == pattern[prefixLen]) {
                prefixLen++;
                i++;
                arr[i] = prefixLen;
            } else if (prefixLen > 0) {
                prefixLen = arr[prefixLen];
            } else {
                i++;
                arr[i] = 0;
            }
        }
        return arr;
    }
};

