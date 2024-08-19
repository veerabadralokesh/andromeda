class Solution {
public:
    long long maxEnergyBoost(vector<int>& energyDrinkA, vector<int>& energyDrinkB) {
        int n = energyDrinkA.size();
        long long dpa[n], dpb[n];
        dpa[0] = (long long)energyDrinkA[0], dpa[1] = dpa[0]+(long long)energyDrinkA[1];
        dpb[0] = (long long)energyDrinkB[0], dpb[1] = dpb[0]+(long long)energyDrinkB[1];
        for (int i=2; i < n; i++) {
            dpa[i] = (long long)energyDrinkA[i] + max(dpa[i-1], dpb[i-2]);
            dpb[i] = (long long)energyDrinkB[i] + max(dpa[i-2], dpb[i-1]);
        }
        return max(dpa[n-1], dpb[n-1]);
    }
};

