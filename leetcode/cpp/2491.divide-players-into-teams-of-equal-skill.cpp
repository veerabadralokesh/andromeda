class Solution {
public:
    long long dividePlayers(vector<int>& skill) {
        int total = 0;
        int n = skill.size()/2;
        for (const int s: skill) {
            total += s;
        }
        if (total % n != 0)
            return -1;
        
        int teamSkill = total / n;

        vector<int> rem(teamSkill+1, 0);

        for (const int s: skill) {
            if (s >= teamSkill)
                return -1;
            rem[s]++;
        }

        long int ans = 0;

        for (int i=1; i < 1 + teamSkill/2; i++) {
            if (rem[i]!=rem[teamSkill-i]) {
                return -1;
            }
            if (i != teamSkill - i) {
                ans += ((long)i * (long)(teamSkill - i) * (long)rem[i]);
            } else {
                if (rem[i] & 1 == 1)
                    return -1;
                ans += ((long)i * (long)(teamSkill - i) * (long)rem[i])/2;
            }
        }
        return ans;
    }
};

