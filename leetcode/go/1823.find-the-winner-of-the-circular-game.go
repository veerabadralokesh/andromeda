func findTheWinner(n int, k int) int {
    arr := make([]int, n)
    for i := range n {
        arr[i] = i+1
    }
    for len(arr) > 1 {
        rotate(arr, (k-1)%len(arr))
        arr = arr[1:]
    }
    return arr[0]
}

func rotate(arr []int, k int) {
    slices.Reverse(arr[:k])
    slices.Reverse(arr[k:])
    slices.Reverse(arr)
}

/* */

func findTheWinner(n int, k int) int {
  friends := make([]int, n)
  index := 0
  for i:=index; i< n ; i++ {
    friends[i] = i+1
  }

  for len(friends) > 1 {
    index = (index + k  - 1)% len(friends)
    friends = append(friends[:index], friends[index+1:]...)
  }

   return friends[0]
}

