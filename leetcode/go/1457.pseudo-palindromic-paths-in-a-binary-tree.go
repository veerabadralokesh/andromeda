/**
 * Definition for a binary tree node.
 * type TreeNode struct {
 *     Val int
 *     Left *TreeNode
 *     Right *TreeNode
 * }
 */
func pseudoPalindromicPaths (root *TreeNode) int {
    counts, ans := make([]int, 10), 0
    dfs(root, counts, &ans)
    return ans
}

func dfs(root *TreeNode, counts []int, ans *int) {
    if root == nil {
        return
    }
    counts[root.Val] += 1
    if root.Left == nil && root.Right == nil {
        odd := 0
        for _, c := range counts {
            if c & 1 == 1 {
                odd += 1
            }
        }
        if odd < 2 {
            *ans++
        }
    } else {
        dfs(root.Left, counts, ans)
        dfs(root.Right, counts, ans)
    }
    counts[root.Val] -= 1
}

