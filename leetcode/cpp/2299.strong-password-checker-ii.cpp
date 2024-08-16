class Solution {
public:
    bool strongPasswordCheckerII(string password) {
        string specialChars = "!@#$%^&*()-+";
        bool len = password.length() > 7, lower = false, upper = false, digit = false, special = false;
        char c;
        for (int i=0; i < password.length(); i++) {
            c = password[i];
            if (i > 0 && password[i-1] == c) {
                return false;
            }
            if (c >= 'a' && c <= 'z') {
                lower = true;
            } else if (c >= 'A' && c <= 'Z') {
                upper = true;
            } else if (c >= '0' && c <= '9') {
                digit = true;
            } else if (specialChars.contains(c)) {
                special = true;
            }
        }
        return len && lower && upper && digit && special;
    }
};

