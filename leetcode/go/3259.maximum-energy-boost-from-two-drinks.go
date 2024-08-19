func maxEnergyBoost(energyDrinkA []int, energyDrinkB []int) int64 {
    n := len(energyDrinkA)
    dpa, dpb := make([]int, n), make([]int, n)
    dpa[0], dpb[0] = energyDrinkA[0], energyDrinkB[0]
    dpa[1], dpb[1] = dpa[0]+energyDrinkA[1], dpb[0]+energyDrinkB[1]
    for i := 2; i < n; i++ {
        dpa[i] = energyDrinkA[i] + max(dpa[i-1], dpb[i-2])
        dpb[i] = energyDrinkB[i] + max(dpa[i-2], dpb[i-1])
    }
    return int64(max(dpa[n-1], dpb[n-1]))
}

