import (
    "sort"
)
func minimumTimeRequired(jobs []int, k int) int {
    sort.Sort(sort.Reverse(sort.IntSlice(jobs)))
    ans := 0
    for _, j := range jobs {
        ans += j
    }
    workers := make([]int, k)
    backtrack(jobs, 0, workers, &ans)
    return ans
}

func backtrack(jobs []int, job_id int, workers []int, ans *int) {
    if job_id == len(jobs) {
        mx := workers[0]
        for i:=1; i<len(workers); i++ {
            mx = max(mx, workers[i])
        }
        *ans = min(*ans, mx)
        return
    }
    for worker_id:=0; worker_id < len(workers); worker_id++ {
        if workers[worker_id] + jobs[job_id] >= *ans {
            continue
        }
        workers[worker_id] += jobs[job_id]
        backtrack(jobs, job_id + 1, workers, ans)
        workers[worker_id] -= jobs[job_id]
        if workers[worker_id] == 0 {
            return
        }
    }
}

