func relativeSortArray(arr1 []int, arr2 []int) []int {
    indices := make(map[int]int)
    for i, n := range arr2 {
        indices[n] = i-len(arr2);
    }
    sort.Slice(arr1, func(i, j int) bool {
        x, y := arr1[i], arr1[j]
        idxi, idxj := indices[x], indices[y]
        if idxi != idxj {
            return idxi < idxj
        }
        return x < y
    })
    return arr1
}

/* */

func relativeSortArray(arr1 []int, arr2 []int) []int {
    m := make(map[int]int)

    for i,n := range arr2 {
        m[n] = i
    }

    sort.Slice(arr1, func(i,j int) bool{
        _, ok1 := m[arr1[i]]
        _, ok2 := m[arr1[j]]
        if ok1 && ok2 {
            return m[arr1[i]] < m[arr1[j]]
        } else if ok1 {
            return true
        } else if ok2 {
            return false
        } else {
            return arr1[i] < arr1[j]
        }

    })
    return arr1
}

