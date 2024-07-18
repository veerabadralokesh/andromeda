/**
 * Definition for a Node.
 * type Node struct {
 *     Val int
 *     Left *Node
 *     Right *Node
 *     Next *Node
 * }
 */

type QNode struct {
    node *Node
    level int
}

func connect(root *Node) *Node {
    if root == nil {
        return root
    }
	q := make([]*QNode, 0, 3000)
    q = append(q, &QNode{node: root, level: 0})
    qnode := q[0]
    var tnode *Node = nil
    level := 0
    for len(q) > 0 {
        qnode = q[0]
        tnode, level = qnode.node, qnode.level
        q = q[1:]
        if len(q) > 0 && level == q[0].level {
            tnode.Next = q[0].node
        }
        if tnode.Left != nil {
            q = append(q, &QNode{node: tnode.Left, level: (level + 1)})
        }
        if tnode.Right != nil {
            q = append(q, &QNode{node: tnode.Right, level: (level + 1)})
        }
    }
    return root
}

