func memLeak(memory1 int, memory2 int) []int {
    process := 1
    for process <= memory1 || process <= memory2 {
        if memory1 >= memory2 {
            memory1 -= process
        } else {
            memory2 -= process
        }
        process += 1
    }
    return [] int {process, memory1, memory2}
}

