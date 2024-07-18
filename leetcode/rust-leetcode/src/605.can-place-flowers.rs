impl Solution {
    pub fn can_place_flowers(flowerbed: Vec<i32>, n: i32) -> bool {
        let mut n = n;
        let mut fb = flowerbed;
        for i in 0..fb.len(){
            if(fb[i]==1){
                continue
            }
            if(i==0||fb[i-1]==0)&&(i==fb.len()-1||fb[i+1]==0){
                fb[i]=1;
                n-=1;
            }
        }
        n <= 0
    }
}

impl Solution2 {
    pub fn can_place_flowers(flowerbed: Vec<i32>, n: i32) -> bool {
        let mut num_zeros:i32 = 1;
        let mut total:i32 = 0;
        for i in flowerbed {
            if i == 0 {
                num_zeros += 1;
            } else {
                total += (num_zeros - 1)/2;
                num_zeros = 0;
            }
        }
        if num_zeros > 0 {
            total += num_zeros / 2;
        }
        total >= n
    }
}