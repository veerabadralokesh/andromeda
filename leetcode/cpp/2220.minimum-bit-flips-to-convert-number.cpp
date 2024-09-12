class Solution {
public:
    int minBitFlips(int start, int goal) {
        return countOnes(start | goal) - countOnes(start & goal);
    }
private:
    int countOnes(int x) {
        int count = 0;
        while (x > 0) {
            if (x & 1 == 1) {
                count++;
            }
            x >>= 1;
        }
        return count;
    }
};

