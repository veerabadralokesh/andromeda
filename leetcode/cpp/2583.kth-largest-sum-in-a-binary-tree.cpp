/**
 * Definition for a binary tree node.
 * struct TreeNode {
 *     int val;
 *     TreeNode *left;
 *     TreeNode *right;
 *     TreeNode() : val(0), left(nullptr), right(nullptr) {}
 *     TreeNode(int x) : val(x), left(nullptr), right(nullptr) {}
 *     TreeNode(int x, TreeNode *left, TreeNode *right) : val(x), left(left), right(right) {}
 * };
 */
class Solution {
public:
    long long kthLargestLevelSum(TreeNode* root, int k) {
        vector<long> levelSums;
        levelSums.push_back(0);
        queue<TreeNode*> q;
        queue<int> levels;
        q.push(root);
        levels.push(0);

        while (!levels.empty()) {
            const int level = levels.front();
            levels.pop();
            if (levelSums.size() == level) {
                levelSums.push_back(0);
            }
            auto tnode = q.front();
            q.pop();
            levelSums[level] += tnode->val;
            if (tnode->left !=nullptr) {
                q.push(tnode->left);
                levels.push(level + 1);
            }
            if (tnode->right !=nullptr) {
                q.push(tnode->right);
                levels.push(level + 1);
            }
        }
        if (k > levelSums.size()) {
            return -1;
        }
        sort(levelSums.begin(), levelSums.end(), std::greater<>());
        return levelSums[k-1];
    }
};

/* */

/**
 * Definition for a binary tree node.
 * struct TreeNode {
 *     int val;
 *     TreeNode *left;
 *     TreeNode *right;
 *     TreeNode() : val(0), left(nullptr), right(nullptr) {}
 *     TreeNode(int x) : val(x), left(nullptr), right(nullptr) {}
 *     TreeNode(int x, TreeNode *left, TreeNode *right) : val(x), left(left), right(right) {}
 * };
 */
class Solution {
public:
    vector<long long int> v;
    int depth = -1;
    void getmaxdepth(TreeNode* root, int l = 0) {
        if (root == nullptr)
            return;
        depth = max(depth, l);
        getmaxdepth(root->right, l + 1);
        getmaxdepth(root->left, l + 1);
    }
    void search(TreeNode* root, int l = 0) {
        if (root == nullptr)
            return;
        v[l] += root->val;
        search(root->right, l + 1);
        search(root->left, l + 1);
    }
    long long kthLargestLevelSum(TreeNode* root, int k) {
        getmaxdepth(root);
        if (k > depth+1) {
            return -1;
        }
        v.resize(depth+1, 0);
        search(root);
        sort(v.begin(), v.end());
        return v[v.size()-k];
    }
};

