class Solution {
public:
    int countKConstraintSubstrings(string s, int k) {
        int ans=0, zeros=0, ones=0, left=0, right=0;
        for (right=0; right < s.length(); right++) {
            if (s[right] == '0') {
                zeros++;
            } else {
                ones++;
            }
            while (zeros > k && ones > k) {
                if (s[left] == '0') {
                    zeros--;
                } else {
                    ones--;
                }
                left++;
            }
            ans += (right - left + 1);
        }
        return ans;
    }
};

