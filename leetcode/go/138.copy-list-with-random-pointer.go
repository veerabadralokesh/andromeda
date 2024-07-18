/**
 * Definition for a Node.
 * type Node struct {
 *     Val int
 *     Next *Node
 *     Random *Node
 * }
 */

func copyRandomList(head *Node) *Node {
    if head == nil {
        return nil
    }
    i := 0
    nodeIdxMap := map[*Node]int{}
    temp := head
    for temp != nil {
        nodeIdxMap[temp] = i
        i++
        temp = temp.Next
    }
    temp = head
    idxRandomIdxMap := map[int]int{}
    i = 0
    for temp != nil {
        if temp.Random != nil {
            idxRandomIdxMap[i] = nodeIdxMap[temp.Random]
        }
        i += 1
        temp = temp.Next
    }
    newNodes := make([]*Node, 0, i)
    newHead := Node {Val: head.Val}
    temp = &newHead
    for {
        temp.Val = head.Val
        newNodes = append(newNodes, temp)
        head = head.Next
        if head == nil {
            break;
        }
        temp.Next = &Node{}
        temp = temp.Next
    }
    temp = &newHead
    i = 0
    for i < len(newNodes) {
        if val, ok := idxRandomIdxMap[i]; ok {
            temp.Random = newNodes[val]
        }
        temp = temp.Next
        i++
    }
    return &newHead
}


