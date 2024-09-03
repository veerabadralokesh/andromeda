class Solution {
public:
    int getLucky(string s, int k) {
        int num = 0;
        for (char c: s) {
            num += digitSum(c - (int)'a' + 1);
        }
        while (k > 1 && num > 9) {
            num = digitSum(num);
            k--;
        }
        return num;
    }
private:
    int digitSum(int x) {
        if (x < 10) {
            return x;
        } else {
            int y = 0;
            while (x > 0) {
                y += x % 10;
                x /= 10;
            }
            return y;
        }
    }
};

