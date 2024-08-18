class Solution {
public:
    vector<int> plusOne(vector<int>& digits) {
        int n = digits.size();
        int carry = 1;
        for (int i=n-1; i > -1; i--) {
            digits[i] = digits[i] + carry;
            if (digits[i] > 9) {
                digits[i] -= 10;
                carry = 1;
            } else {
                carry = 0;
                break;
            }
        }
        if (carry == 1) {
            digits.insert(digits.begin(), 1);
        }
        return digits;
    }
};

