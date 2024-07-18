/**
 * Definition for a binary tree node.
 * type TreeNode struct {
 *     Val int
 *     Left *TreeNode
 *     Right *TreeNode
 * }
 */

import "math"

func dfs(node *TreeNode, height *float64) bool {
    if node == nil {
        return true
    }
    left_height := 0.0
    right_height := 0.0
    if !dfs(node.Left, &left_height) || !dfs(node.Right, &right_height) {
        return false
    }
    if math.Abs(left_height-right_height) > 1 {
        return false
    }
    *height = math.Max(left_height, right_height) + 1
    return true
}

func isBalanced(root *TreeNode) bool {
    height := 0.0
    return dfs(root, &height)
}

