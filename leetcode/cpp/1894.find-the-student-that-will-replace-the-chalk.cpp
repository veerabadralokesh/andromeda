class Solution {
public:
    int chalkReplacer(vector<int>& chalk, int k) {
        long long total = 0;
        for (auto n : chalk)
            total += (long)n;
        
        long rem = (long)k % total;
        for (int i=0; i < chalk.size(); i++) {
            if (rem < chalk[i]) {
                return i;
            }
            rem -= (long)chalk[i];
        }
        return -1;
    }
};

