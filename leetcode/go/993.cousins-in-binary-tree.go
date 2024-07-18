/**
 * Definition for a binary tree node.
 * type TreeNode struct {
 *     Val int
 *     Left *TreeNode
 *     Right *TreeNode
 * }
 */
func isCousins(root *TreeNode, x int, y int) bool {
    nodes := [][]int{}
    dfs(root, x, y, -1, 0, &nodes)
    return nodes[0][0] == nodes[1][0] && nodes[0][1] != nodes[1][1]
}

func dfs(node *TreeNode, x int, y int, parent int, depth int, nodes *[][]int) {
    if node == nil || len(*nodes) == 2 {
        return
    }
    if (*node).Val == x || (*node).Val == y {
        *nodes = append(*nodes, []int{depth, parent})
    }
    dfs(node.Left, x, y, node.Val, depth+1, nodes)
    dfs(node.Right, x, y, node.Val, depth+1, nodes)
}


