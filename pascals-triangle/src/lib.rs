pub struct PascalsTriangle(u32);

impl PascalsTriangle {
    pub fn new(row_count: u32) -> Self {
        PascalsTriangle(row_count)
    }

    pub fn rows(&self) -> Vec<Vec<u32>> {
        match self.0 {
            0 => vec![],
            1 => vec![vec![1]],
            _ => {
                let mut pt2: Vec<Vec<u32>> = vec![vec![1]];
                for _ in 1..self.0 {
                    let last = pt2.last().unwrap();
                    let mut new = vec![0];
                    new.extend(last.iter());
                    new.extend(vec![0].iter());
                    pt2
                        .push(
                            new
                                .windows(2)
                                .map(|pair| {
                                    match pair {
                                        &[x, y] => x + y,
                                        _ => panic!("This can never happen: Any other scenario")
                                    }
                                })
                                .collect()
                        );
                }

                pt2
            }
        }
    }
}
