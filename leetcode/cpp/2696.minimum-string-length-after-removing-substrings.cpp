class Solution {
public:
    int minLength(string s) {
        vector<char> stack;
        for (const char c: s) {
            if (!stack.empty() && ((stack.back() == 'C' && c == 'D') || (stack.back() == 'A' && c == 'B'))) {
                stack.pop_back();
            } else {
                stack.push_back(c);
            }
        }
        return stack.size();
    }
};

/* */

class Solution {
public:
    int minLength(string s) {
        string stack = "";
        for (const char c: s) {
            if (!stack.empty() && ((stack.back() == 'C' && c == 'D') || (stack.back() == 'A' && c == 'B'))) {
                stack.pop_back();
            } else {
                stack.push_back(c);
            }
        }
        return stack.length();
    }
};

