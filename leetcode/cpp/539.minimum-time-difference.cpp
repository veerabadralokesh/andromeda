#include <vector>
#include <string>
#include <algorithm>

using namespace std;

class Solution {
public:
    int findMinDifference(vector<string>& timePoints) {
        vector<int> minutes;
        for (const string& timePoint : timePoints) {
            int hour = stoi(timePoint.substr(0, 2));
            int minute = stoi(timePoint.substr(3, 2));
            minutes.push_back(hour * 60 + minute);
        }

        sort(minutes.begin(), minutes.end());

        int ans = 1440;
        for (int i = 1; i < minutes.size(); ++i) {
            ans = min(ans, minutes[i] - minutes[i - 1]);
        }

        ans = min(ans, 1440 - minutes.back() + minutes.front());

        return ans;
    }
};

