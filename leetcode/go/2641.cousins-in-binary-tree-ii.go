/**
 * Definition for a binary tree node.
 * type TreeNode struct {
 *     Val int
 *     Left *TreeNode
 *     Right *TreeNode
 * }
 */
func replaceValueInTree(root *TreeNode) *TreeNode {
    levelSums := make([]int, 0, 100)
    levelSums = append(levelSums, 0)

    dfs(root, 0, &levelSums)

    fmt.Println(levelSums)
    return buildTree(root, 0, &levelSums, root.Val)
}

func buildTree(root *TreeNode, depth int, levelSums *[]int, siblingSum int) *TreeNode {
    if root == nil {
        return nil
    }
    ans := TreeNode {
        Val: (*levelSums)[depth]-siblingSum,
    }
    childSum := 0
    if root.Left != nil {
        childSum += root.Left.Val
    }
    if root.Right != nil {
        childSum += root.Right.Val
    }
    ans.Left = buildTree(root.Left, depth+1, levelSums, childSum)
    ans.Right = buildTree(root.Right, depth+1, levelSums, childSum)
    return &ans
}

func dfs(root *TreeNode, depth int, levelSums *[]int) {
    if root == nil {
        return
    }
    if len(*levelSums) == depth {
        *levelSums = append(*levelSums, 0)
    }
    (*levelSums)[depth] += root.Val
    dfs(root.Left, depth + 1, levelSums)
    dfs(root.Right, depth + 1, levelSums)
}

