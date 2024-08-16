class Solution {
public:
    string validIPAddress(string queryIP) {
        bool ipv4, ipv6;
        ipv4 = queryIP.contains('.');
        ipv6 = queryIP.contains(':');
        if (ipv4 && ipv6) {
            return "Neither";
        }
        string delimiter = ".";
        string token;
        int count = 0;
        if (ipv4) {
            for (string token: split(queryIP, delimiter)) {
                count++;
                if (!isValidIPv4Token(token)) {
                    return "Neither";
                }
            }
            if (count != 4) {
                return "Neither";
            }
            return "IPv4";
        } else {
            delimiter = ":";
            for (string token: split(queryIP, delimiter)) {
                count++;
                if (!isValidIPv6Token(token)) {
                    return "Neither";
                }
            }
            if (count != 8) {
                return "Neither";
            }
            return "IPv6";
        }
    }
private:
    bool isValidIPv4Token(string token) {
        if (token.length() == 0 || (token.length() > 1 && token[0] == '0') || token.length() > 3) {
            return false;
        }
        std::string::const_iterator it = token.begin();
        int num = 0;
        while (it != token.end() && std::isdigit(*it)) {
            num = (num * 10) + ((*it) - '0');
            ++it;
        }
        return !token.empty() && it == token.end() && num < 256;
    }
    bool isValidIPv6Token(string token) {
        if (token.length() < 1 || token.length() > 4) return false;
        for (char c: token) {
            if ((c < 'a' && c > 'F') || c  > 'f') {
                return false;
            }
        }
        return true;
    }
    std::vector<std::string> split(std::string s, std::string delimiter) {
        size_t pos_start = 0, pos_end, delim_len = delimiter.length();
        std::string token;
        std::vector<std::string> res;

        while ((pos_end = s.find(delimiter, pos_start)) != std::string::npos) {
            token = s.substr (pos_start, pos_end - pos_start);
            pos_start = pos_end + delim_len;
            res.push_back (token);
        }

        res.push_back (s.substr (pos_start));
        return res;
    }
};

