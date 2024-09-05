class Solution {
public:
    int robotSim(vector<int>& commands, vector<vector<int>>& obstacles) {
        const int dirs[4][2] = {{0,1}, {1, 0}, {0, -1}, {-1, 0}};
        int ans = 0;
        int dir = 0;
        int x = 0;
        int y = 0;

        unordered_set<pair<int,int>, PairHash> obstacleSet;
        for(auto o: obstacles) {
            obstacleSet.insert({o[0], o[1]});
        }

        for (auto c: commands) {
            if (c == -1) {
                dir = (dir + 1) % 4;
            } else if (c == -2) {
                dir = (dir + 3) % 4;
            } else {
                for (int i = 0; i < c; i++) {
                    if (obstacleSet.contains({x + dirs[dir][0], y + dirs[dir][1]})) {
                        break;
                    }
                    x += dirs[dir][0];
                    y += dirs[dir][1];
                }
                ans = max(ans, x * x + y * y);
            }
        }

        return ans;
    }
private:
    struct PairHash {
        size_t operator()(const pair<int, int>& p) const {
            return p.first ^ p.second;
        }
    };
};

