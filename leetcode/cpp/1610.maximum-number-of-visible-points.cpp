class Solution {
public:
    int visiblePoints(vector<vector<int>>& points, int angle, vector<int>& location) {
        const int x = location[0];
        const int y = location[1];
        int maxVisible = 0;
        int same = 0, px = 0, py = 0;
        vector<double> angles;

        for (const vector<int>& p: points) {
            px = p[0];
            py = p[1];
            if (px == x && py == y) {
                same++;
            } else {
                angles.push_back(getAngle(y - py, x - px));
            }
        }
        ranges::sort(angles);

        int n = angles.size();
        for (int i = 0; i < n ; i++) {
            angles.push_back(angles[i] + 360);
        }

        for (int l = 0, r = 0; r < angles.size() ; r++) {
            while (angles[r] - angles[l] > angle) {
                l++;
            }
            maxVisible = max(maxVisible, r - l + 1);
        }
        return same + maxVisible;
    }
private:
    double getAngle(int dx, int dy) {
        return atan2(dy, dx) * 180 / M_PI;
    }
};

