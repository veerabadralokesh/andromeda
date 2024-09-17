class Solution {
public:
    vector<string> uncommonFromSentences(string s1, string s2) {
        unordered_map<string, int> counts;

        string combined = s1 + " " + s2;
        string token = "";
        istringstream iss(combined);

        while (iss >> token) {
            ++counts[token];
        }

        vector<string> result;

        for (auto it : counts) {
            if (it.second == 1) {
                result.push_back(it.first);
            }
        }

        return result;
    }
};

