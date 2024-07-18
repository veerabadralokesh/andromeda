/**
 * Definition for a binary tree node.
 * type TreeNode struct {
 *     Val int
 *     Left *TreeNode
 *     Right *TreeNode
 * }
 */
func delNodes(root *TreeNode, to_delete []int) []*TreeNode {
    m := make(map[int]bool, len(to_delete))
    for _, n := range to_delete {
        m[n] = true
    }
    ans := make([]*TreeNode, 0, len(to_delete)*2)
    root_val := recurse(root, &ans, m)
    if _, ok := m[root_val]; !ok {
        ans = append(ans, root)
    }
    return ans
}

func recurse(root *TreeNode, ans *[]*TreeNode, m map[int]bool) int {
    if root == nil {
        return -1
    }
    delete_root := false
    if _, ok := m[root.Val]; ok {
        delete_root = true
    }
    
    if root.Left != nil {
        left_val := recurse(root.Left, ans, m)
        
        if delete_root {
            if _, ok := m[left_val]; !ok {
                *ans = append(*ans, root.Left)
            }
        }

        if _, ok := m[left_val]; ok {
            root.Left = nil
        }
    }
    if root.Right != nil {

        right_val := recurse(root.Right, ans, m)
        
        if _, ok := m[right_val]; !ok && delete_root {
            *ans = append(*ans, root.Right)
        }

        if _, ok := m[right_val]; ok {
            root.Right = nil
        }
    }
    return root.Val
}

