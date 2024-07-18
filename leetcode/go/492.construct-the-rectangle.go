func constructRectangle(area int) []int {
    w := int(math.Floor(math.Sqrt(float64(area))))
    l := area / w
    for (w * l != area) {
        w -= 1
        l = area/w
    }
    return []int{l, w}
}

