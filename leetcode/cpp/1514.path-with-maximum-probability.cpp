class Solution {
public:
    double maxProbability(int n, vector<vector<int>>& edges, vector<double>& succProb, int start_node, int end_node) {
        using node = pair<double, int>;
        vector<vector<node>> graph(n, vector<node>());
        int u, v;
        for (int i = 0; i < edges.size(); i++) {
            u = edges[i][0], v = edges[i][1];
            graph[u].push_back({succProb[i], v});
            graph[v].push_back({succProb[i], u});
        }

        priority_queue<node, vector<node>> heap;
        heap.push({1.0, start_node});

        vector<double> probs(n, 0.0);
        probs[start_node] = 1.0;

        while (!heap.empty()) {
            auto [prob, u] = heap.top();
            heap.pop();
            if (u == end_node) {
                return prob;
            }

            for (auto [prob_uv, v]: graph[u]) {
                double new_prob = prob * prob_uv;
                if (new_prob > probs[v]) {
                    probs[v] = new_prob;
                    heap.push({new_prob, v});
                }
            }
        }

        return 0.0;
    }
};

