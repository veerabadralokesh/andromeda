class Solution {
public:
    bool lemonadeChange(vector<int>& bills) {
        int five=0, ten=0, twenty=0;
        for (int b: bills) {
            if (b == 5) {
                five++;
                continue;
            }
            if (b == 10) {
                if (five == 0) {
                    return false;
                }
                ten++;
                five--;
            } else {
                if (ten > 0 && five > 0) {
                    twenty++;
                    ten--;
                    five--;
                } else if (five > 2) {
                    five -= 3;
                    twenty++;
                } else {
                    return false;
                }
            }
            // cout << five << "," << ten << "," << twenty << endl;
        }
        return true;
    }
};

