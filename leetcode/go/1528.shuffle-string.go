func restoreString(s string, indices []int) string {
    sb := []byte(s)
    rb := make([]byte, len(s))
    for i, b := range sb {
        rb[indices[i]] = b
    }
    return string(rb)
}

