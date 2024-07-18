impl Solution {
    pub fn largest_overlap(img1: Vec<Vec<i32>>, img2: Vec<Vec<i32>>) -> i32 {
        let n = img1.len();
        let (mut empty1, mut empty2) = (true, true);
        for i in 0..n {
            for j in 0..n {
                if img1[i][j] == 1 {
                    empty1 = false;
                }
                if img2[i][j] == 1 {
                    empty2 = false;
                }
            }
        }
        if empty1 || empty2 {
            return 0;
        }
        let m = 3 * n - 2;
        let mut im = vec![vec![0; m]; m];
        let (start, end) = (n-1, 2*n-1);
        for (i, ti) in (0..n).zip(start..end) {
            for (j, tj) in (0..n).zip(start..end) {
                im[ti][tj] = img1[i][j];
            }
        }
        let (mut overlap, mut max_overlap) = (0, 0);
        for bi in 0..end {
            for bj in 0..end {
                overlap = 0;
                for i in 0..n {
                    for j in 0..n {
                        overlap += im[bi+i][bj+j] * img2[i][j];
                    }
                }
                max_overlap = max_overlap.max(overlap);
            }
        }
        max_overlap
    }
}

/* */

// LEARN

const MAX_SIZE: usize = 30;

impl Solution {
  pub fn largest_overlap(img1: Vec<Vec<i32>>, img2: Vec<Vec<i32>>) -> i32 {
    let enumerate_ones = |img: Vec<Vec<i32>>| {
      img.into_iter().enumerate().flat_map(|(i, row)| {
        row
          .into_iter()
          .enumerate()
          .filter_map(move |(j, val)| (val == 1).then(|| (i, j)))
      })
    };

    let ones2 = enumerate_ones(img2).collect::<Vec<_>>();

    let mut transform_freqs = [[0; 2 * MAX_SIZE]; 2 * MAX_SIZE];

    let mut rez = 0;

    for (r1, c1) in enumerate_ones(img1) {
      for (r2, c2) in ones2.iter() {
        let entry = &mut transform_freqs[MAX_SIZE + r1 - r2][MAX_SIZE + c1 - c2];

        *entry += 1;

        rez = rez.max(*entry);
      }
    }

    rez
  }
}

