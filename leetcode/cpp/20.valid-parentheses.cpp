class Solution {
public:
    bool isValid(string s) {
        vector<char> stack;
        for (char c: s) {
            switch (c) {
                case ')':
                    if (stack.empty() || stack.back() != '(') {
                        return false;
                    }
                    stack.pop_back();
                    break;
                case ']':
                    if (stack.empty() || stack.back() != '[') {
                        return false;
                    }
                    stack.pop_back();
                    break;
                case '}':
                    if (stack.empty() || stack.back() != '{') {
                        return false;
                    }
                    stack.pop_back();
                    break;
                default:
                    stack.push_back(c);
                    break;
            }
        }
        return stack.size() == 0;
    }
};

/* */

class Solution {
public:
    bool isValid(string s) {
        stack<char> stk;

        for (char& c : s) {
            if (c == '{' || c == '(' || c == '[') {
                stk.push(c);
            } else if (c == '}' || c == ']' || c == ')') {
                if (stk.empty()) return false;
                char open = stk.top();
                stk.pop();
                if ((c == '}' && open != '{') || (c == ']' && open != '[') || (c == ')' && open != '(')) return false;
            }
        }

        return stk.empty();
    }
};

