class Solution {
public:
    int hIndex(vector<int>& citations) {
        int n=citations.size(), l=0, r=citations.size(), m=0;
        while (l < r) {
            m = (l + r)/2;
            if (citations[m] >= n - m) {
                r = m;
            } else {
                l = m+1;
            }
        }
        return n-l;
    }
};

