class Solution {
public:
    int minSwaps(string s) {
        int swaps=0, swapPos = -1, open = 0;

        for (int i=0; i < s.length(); i++) {
            if (s[i] == '[') {
                open++;
            } else if (open > 0) {
                open--;
            } else {
                if (swapPos < 0) {
                    swapPos = i;
                    swaps++;
                } else {
                    swapPos = -1;
                }
            }
        }
        return swaps;
    }
};

