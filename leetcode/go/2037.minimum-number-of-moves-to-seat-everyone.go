func minMovesToSeat(seats []int, students []int) int {
    slices.Sort(seats)
    slices.Sort(students)
    moves := 0
    for i, seat := range seats {
        if seat > students[i] {
            moves += seat - students[i]
        } else {
            moves += students[i] - seat
        }
    }
    return moves
}

/* */

func minMovesToSeat(seats []int, students []int) int {
    sort.Ints(seats)
    sort.Ints(students)
    moves := 0
    for i:=0; i< len(seats); i++ {
        moves += Abs(seats[i], students[i])
    }
    return moves
}

func Abs(x int, y int) int {
    if x > y {
        return x-y
    }
    return y-x
}

