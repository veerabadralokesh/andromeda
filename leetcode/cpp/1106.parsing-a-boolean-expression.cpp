class Solution {
public:
    bool parseBoolExpr(string expression) {
        int i = 0;
        return parse(expression, i);
    }
private:
    bool parse(const string& expr, int& i) {
        if (expr[i] == 't') {
            i++;
            return true;
        }
        if (expr[i] == 'f') {
            i++;
            return false;
        }
        if (expr[i] == '!') {
            i += 2;
            bool ans = !parse(expr, i);
            i++;
            return ans;
        }

        bool isAnd = false;
        if (expr[i] == '&') {
            isAnd = true;
        }
        i += 2;
        bool ans = isAnd;

        while (expr[i] != ')') {
            bool parsedExpr = parse(expr, i);
            if (isAnd) {
                ans &= parsedExpr;
            } else {
                ans |= parsedExpr;
            }
            if (expr[i] == ',') {
                i++;
            }
        }
        i++;
        return ans;
    }
};

