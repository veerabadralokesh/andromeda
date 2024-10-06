class Solution {
public:
    bool areSentencesSimilar(string sentence1, string sentence2) {
        if (sentence1.length() == sentence2.length()) {
            return sentence1 == sentence2;
        }
        auto words1 = split(sentence1, " ");
        auto words2 = split(sentence2, " ");
        int m = words1.size();
        int n = words2.size();
        if (m > n) {
            return areSentencesSimilar(sentence2, sentence1);
        }
        int i = 0;
        
        while (i < m && words1[i] == words2[i]) {
            i++;
        }
        while (i < m && words1[i] == words2[i + n - m]) {
            i++;
        }
        return i == m;
    }
private:
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

