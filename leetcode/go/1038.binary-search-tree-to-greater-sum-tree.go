/**
 * Definition for a binary tree node.
 * type TreeNode struct {
 *     Val int
 *     Left *TreeNode
 *     Right *TreeNode
 * }
 */
func bstToGst(root *TreeNode) *TreeNode {
    sum := 0
    traverse(root, &sum)
    return root
}

func traverse(node *TreeNode, sum *int) {
    if node == nil {return}
    if node.Left == nil && node.Right == nil {
        *sum += node.Val
        node.Val = *sum
        return
    }
    traverse(node.Right, sum)
    *sum += node.Val
    node.Val = *sum
    traverse(node.Left, sum)
}

