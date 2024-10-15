public class Solution {
    public long MinimumSteps(string s) {
        long ans = 0, ones = 0;
        for (int i=0; i < s.Length;i++) {
            if (s[i] == '1') {
                ones++;
            } else {
                ans += ones;
            }
        }
        return ans;
    }
}

