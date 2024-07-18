/**
 * Definition for a QuadTree node.
 * type Node struct {
 *     Val bool
 *     IsLeaf bool
 *     TopLeft *Node
 *     TopRight *Node
 *     BottomLeft *Node
 *     BottomRight *Node
 * }
 */

func construct(grid [][]int) *Node {
    n := len(grid)
    if n == 0 {
        return nil
    }
    val := grid[0][0]
    isLeaf := true
    for i:=0; i < n && isLeaf ; i++ {
        for j:=0 ; j < n ; j++ {
            if grid[i][j] != val {
                isLeaf = false
                break
            }
        }
    }
    var topLeft, topRight, bottomLeft, bottomRight *Node = nil, nil, nil, nil
    if !isLeaf {
        mid := n/2
        topLeft = construct(generateGrid(grid, 0, mid, 0, mid))
        topRight = construct(generateGrid(grid, 0, mid, mid, n))
        bottomLeft = construct(generateGrid(grid, mid, n, 0, mid))
        bottomRight = construct(generateGrid(grid, mid, n, mid, n))
    }
    node_val := false
    if isLeaf && val == 1 {
        node_val = true
    }
    node := Node {
        Val: node_val,
        IsLeaf: isLeaf,
        TopLeft: topLeft,
        TopRight: topRight,
        BottomLeft: bottomLeft,
        BottomRight: bottomRight,
    }
    return &node
}

func generateGrid(grid [][]int, row_start int, row_end int, col_start int, col_end int) [][]int {
    n := len(grid)
    if n == 0 {
        return [][]int{}
    }
    m, n := row_end-row_start, col_end-col_start
    subGrid := make([][]int, m)
    for i := range m {
        subGrid[i] = grid[row_start + i][col_start:col_end];
    }
    return subGrid
}

