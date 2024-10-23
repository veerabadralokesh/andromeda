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
    int levelSums[100001];
    TreeNode* replaceValueInTree(TreeNode* root) {
        // vector<int> levelSums(100000, 0);
        dfs(root, 0);
        return buildTree(root, 0, levelSums[0]);
    }
private:
    TreeNode* buildTree(TreeNode* root, int depth, int siblingSum) {
        if (root == nullptr) {
            return nullptr;
        }
        TreeNode* ans = new TreeNode(levelSums[depth] - siblingSum);
        int childSum = 0;
        if (root->left != nullptr) {
            childSum += root->left->val;
        }
        if (root->right != nullptr) {
            childSum += root->right->val;
        }
        ans->left = buildTree(root->left, depth+1, childSum);
        ans->right = buildTree(root->right, depth+1, childSum);
        return ans;
    }
    void dfs(TreeNode* root, int depth) {
        if (root == nullptr) {
            return;
        }
        levelSums[depth] += root->val;
        dfs(root->left, depth + 1);
        dfs(root->right, depth + 1);
    }
};

