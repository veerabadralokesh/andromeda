/**
 * Definition for a binary tree node.
 * type TreeNode struct {
 *     Val int
 *     Left *TreeNode
 *     Right *TreeNode
 * }
 */
func longestZigZag(root *TreeNode) int {
    return dfs(root)[2]
}

func dfs(node *TreeNode) []int {
    if node == nil {
        return []int{-1,-1,-1}
    }
    left := dfs(node.Left)
    right := dfs(node.Right)
    left_zigzag := right[1] + 1
    right_zigzag := left[0] + 1
    subtree_max := Max(Max(left_zigzag, right_zigzag), Max(left[2], right[2]))
    return []int{left_zigzag,right_zigzag,subtree_max}
}

func Max(a int, b int) int {
    if a > b {return a}
    return b
}

