func poorPigs(buckets int, minutesToDie int, minutesToTest int) int {
    pigs, bucketsCovered, pigToBuckets := 0, 1, 1 + (minutesToTest/minutesToDie)
    for bucketsCovered < buckets {
        pigs++
        bucketsCovered *= pigToBuckets
    }
    return pigs
}

