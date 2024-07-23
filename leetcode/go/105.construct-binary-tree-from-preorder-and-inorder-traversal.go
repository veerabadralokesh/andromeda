/**
 * Definition for a binary tree node.
 * type TreeNode struct {
 *     Val int
 *     Left *TreeNode
 *     Right *TreeNode
 * }
 */
func buildTree(preorder []int, inorder []int) *TreeNode {
    if len(preorder) == 0 {
        return nil
    }
    m := make(map[int]int, len(inorder))
    for i, n := range inorder {
        m[n] = i
    }
    preOrderIdx := 0
    return build(preorder, &preOrderIdx, m, 0, len(preorder)-1)
}

func build(preorder []int, preOrderIdx *int, m map[int]int, start int, end int) *TreeNode {
    if start > end {
        return nil
    }
    rootVal := preorder[*preOrderIdx]
    root := TreeNode {
        Val: rootVal,
    }
    *preOrderIdx++
    if idx, ok := m[rootVal]; ok {
        root.Left = build(preorder, preOrderIdx, m, start, idx-1)
        root.Right = build(preorder, preOrderIdx, m, idx+1, end)
    }
    return &root
}

