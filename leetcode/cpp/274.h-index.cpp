class Solution {
public:
    int hIndex(vector<int>& citations) {
        int n = citations.size();
        vector<int> counts(n+1, 0);
        for(int c: citations) {
            if (c > n) {
                c = n;
            }
            counts[c]++;
        }
        for (int i=n-1; i>-1; i--) {
            counts[i] += counts[i+1];
        }
        for (int c=n; c > -1; c--) {
            if (counts[c] >= c) {
                return c;
            }
        }
        return 0;
    }
};

