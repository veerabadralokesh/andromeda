impl Solution {
    pub fn flip_and_invert_image(image: Vec<Vec<i32>>) -> Vec<Vec<i32>> {
        let mut im:Vec<Vec<i32>> = Vec::with_capacity(image.len());
        for row in image.iter() {
            let mut rownew:Vec<i32> = row.to_vec().iter().map(|n| n ^ 1).collect();
            rownew.reverse();
            // Vec::with_capacity(row.len());
            // println!("{:?}", row);
            // println!("{:?}", rownew);
            im.push(rownew);
        }
        im
    }
}

impl Solution2 {
    pub fn flip_and_invert_image(image: Vec<Vec<i32>>) -> Vec<Vec<i32>> {
        let mut image = image;
        image.iter_mut()
        .for_each(|line| {
            line.reverse();
            line.iter_mut()
                .for_each(|x| {
                    *x = 1 - *x;
                });
        });
        image
    }
}

impl Solution3 {
    pub fn flip_and_invert_image(image: Vec<Vec<i32>>) -> Vec<Vec<i32>> {
        image
            .iter()
            .map(|row| row.iter().rev().map(|x| 1 - x).collect())
            .collect()
    }
}
