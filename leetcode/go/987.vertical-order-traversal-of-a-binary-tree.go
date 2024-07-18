/**
 * Definition for a binary tree node.
 * type TreeNode struct {
 *     Val int
 *     Left *TreeNode
 *     Right *TreeNode
 * }
 */
func verticalTraversal(root *TreeNode) [][]int {
    vot := map[int][][]int{}
    dfs(root, 0, 0, vot)
    keys := make([]int, 0, len(vot))
    ans := make([][]int, 0, len(vot))
    for k := range vot {
        keys = append(keys, k)
    }
    slices.Sort(keys)
    for _, k := range keys {
        v := vot[k]
        sort.Slice(v, func(i, j int) bool {
            x, y := v[i], v[j]
            if x[0] == y[0] {
                return x[1] < y[1]
            }
            return x[0] < y[0]
        })
        vals := make([]int, 0, len(v))
        for _, node_val := range v {
            vals = append(vals, node_val[1])
        }
        ans = append(ans, vals)
    }
    return ans
}

func dfs(root *TreeNode, x int, y int, vot map[int][][]int) {
    if root == nil {
        return
    }
    if _, ok := vot[y]; ok {
        vot[y] = append(vot[y], []int{x, root.Val})
    } else {
        v := [][]int{}
        v = append(v, []int{x, root.Val})
        vot[y] = v
    }
    dfs(root.Left, x+1, y-1, vot)
    dfs(root.Right, x+1, y+1, vot)
}


