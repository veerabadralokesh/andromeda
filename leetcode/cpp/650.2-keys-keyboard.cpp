class Solution {
public:
    int minSteps(int n) {
        int dp[n+1];
        dp[1] = 0;
        for (int i = 2; i < n+1; i++) {
            dp[i] = i;
        }
        for (int i=2; i< n/2 + 1; i++) {
            int ops = dp[i]+1;
            for (int j=2*i; j < n+1; j+=i) {
                ops++;
                dp[j] = min(dp[j], ops);
            }
        }
        return dp[n];
    }
};

/* */

class Solution {
public:
    int minSteps(int n) {
        vector<int> primeFactors;
        int cnt=0;
        int d = 2;
        while (n>1){
            while (n%d==0) {
                cnt+=d;
                n/=d;
            }
            d++;
            if (d*d>n) break;
        }
        if (n!=1) cnt+=n;
        return cnt;
    }
};

