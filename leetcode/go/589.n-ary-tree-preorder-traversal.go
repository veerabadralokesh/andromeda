/**
 * Definition for a Node.
 * type Node struct {
 *     Val int
 *     Children []*Node
 * }
 */

func preorder(root *Node) []int {
    preord := []int{}
    dfs(root, &preord)
    return preord
}

func dfs(node *Node, preord *[]int) {
    if node == nil {
        return
    }
    *preord = append(*preord, node.Val)
    for _, c := range node.Children {
        dfs(c, preord)
    }
}

