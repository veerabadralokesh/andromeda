class Solution {
public:
    bool canArrange(vector<int>& arr, int k) {
        vector<int> rems(k, 0);

        for (const int n: arr) {
            rems[((n % k) + k) % k]++;
        }

        if (rems[0] & 1 == 1) {
            return false;
        }

        for (int i = 1; i < 1 + k/2; i++) {
            if (rems[i] != rems[k-i]) {
                return false;
            }
        }
        return true;
    }
};

