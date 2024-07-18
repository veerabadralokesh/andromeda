/**
 * Definition for a binary tree node.
 * type TreeNode struct {
 *     Val int
 *     Left *TreeNode
 *     Right *TreeNode
 * }
 */
func dfs(root *TreeNode, sum int, path []int, target int) [][]int {
    if root == nil {
        return [][]int{}
    }
    path = append(path, root.Val)
    sum += root.Val
    if root.Left == nil && root.Right == nil && sum == target {
        return [][]int{path}
    }
    path_copy := make([]int, len(path))
    copy(path_copy, path)

    return append(dfs(root.Left, sum, path_copy, target), dfs(root.Right, sum, path_copy, target)...)
}

func pathSum(root *TreeNode, targetSum int) [][]int {
    return dfs(root, 0, []int{}, targetSum)
}


