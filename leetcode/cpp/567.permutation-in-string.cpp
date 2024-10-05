class Solution {
public:
    bool checkInclusion(string s1, string s2) {
        if (s1.length() > s2.length()) {
            return false;
        }

        vector<int> s1Counts(26, 0);
        for (char c : s1) {
            s1Counts[c - 'a']++;
        }

        vector<int> s2Counts(26, 0);
        int l = s1.length();

        for (int i = 0; i < l; ++i) {
            s2Counts[s2[i] - 'a']++;
        }

        for (int i = l; i < s2.length(); ++i) {
            if (equal(s1Counts, s2Counts)) {
                return true;
            }

            s2Counts[s2[i] - 'a']++;
            s2Counts[s2[i - l] - 'a']--;
        }

        if (equal(s1Counts, s2Counts)) {
            return true;
        }

        return false;
    }
private:
    bool equal(const vector<int>& a, const vector<int>& b) {
        for (int i = 0; i < a.size(); ++i) {
            if (a[i] != b[i]) {
                return false;
            }
        }
        return true;
    }
};

