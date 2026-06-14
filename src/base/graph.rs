#![allow(unused_variables)]
#![allow(dead_code)]

use std::u64::{self, MAX};

#[derive(Debug, Default)]
pub struct Graph {
    cnt: usize,
    matri: Vec<Vec<u64>>,
    linking: Vec<Vec<(usize, u64)>>,
    pre: Vec<Vec<usize>>,
    shortest: Vec<Vec<u64>>,

    tsp_path: Vec<usize>,
    tsp_cost: Option<u64>,
}

impl Graph {
    pub fn new(size: usize, v: Vec<(usize, usize, u64)>) -> Self {
        let matri = vec![vec![u64::MAX / 2; size]; size];
        let linking: Vec<Vec<(usize, u64)>> = vec![vec![]; size];
        let pre = vec![vec![size; size]; size];
        let shortest = vec![vec![u64::MAX / 2; size]; size];
        let tsp_path = vec![];
        let tsp_cost = None;

        let mut res = Self {
            cnt: size,
            matri,
            linking,
            pre,
            shortest,
            tsp_path,
            tsp_cost,
        };

        res.build(v);

        res.floyd();

        res.tsp();

        res
    }

    fn build(&mut self, v: Vec<(usize, usize, u64)>) {
        for i in 0..self.cnt {
            self.matri[i][i] = 0;
            self.pre[i][i] = i;
            self.shortest[i][i] = 0;
        }

        for (u, v, w) in v {
            self.matri[u][v] = w;
            self.matri[v][u] = w;

            self.shortest[u][v] = w;
            self.shortest[v][u] = w;

            self.linking[u].push((v, w));
            self.linking[v].push((u, w));

            self.pre[u][v] = u;
            self.pre[v][u] = v;
        }
    }

    fn floyd(&mut self) {
        let n = self.cnt;

        for k in 0..n {
            for i in 0..n {
                for j in 0..n {
                    let s = &mut self.shortest;
                    let tmp = s[i][k] + s[k][j];
                    if tmp < s[i][j] {
                        s[i][j] = tmp;
                        self.pre[i][j] = self.pre[k][j];
                    }
                }
            }
        }
    }

    pub fn find_path(&self, src: usize, dst: usize) -> Vec<usize> {
        let mut res = vec![];

        let mut p = dst;

        loop {
            res.push(p);

            if p == src {
                break;
            }
            p = self.pre[src][p];
        }

        res.reverse();
        res
    }

    fn tsp(&mut self) {
        let cnt = self.cnt;
        let max_mask: usize = 1 << cnt;

        let mut dp = vec![vec![u64::MAX; cnt]; max_mask];

        for mask in 0..max_mask {
            if (mask & 1) == 1 {
                for p in 1..self.cnt {
                    if mask & (1 << p) == 1 {
                        let mut temp = u64::MAX;
                        let t_mask = mask ^ (1 << p);

                        for k in 0..self.cnt {
                            if t_mask & (1 << k) == 1 {
                                let candidate = dp[t_mask][k] + self.shortest[k][p];
                                if candidate < temp {
                                    temp = candidate;
                                    dp[mask][p] = candidate;
                                }
                            }
                        }
                    }
                }
            }
        }
    }

    pub fn tsp_path(&self) -> &Vec<usize> {
        &self.tsp_path
    }
}

#[cfg(test)]
mod test {
    use crate::base::graph::Graph;

    #[test]
    fn floyd() {
        let es = vec![(0, 1, 5), (0, 3, 10), (1, 2, 3), (2, 3, 1), (1, 3, 6)];

        let gra = Graph::new(4, es);

        assert_eq!(gra.shortest[1][3], 4);
        assert_eq!(gra.pre[0][3], 2);
    }

    #[test]
    fn find_path() {
        let es = vec![(0, 1, 5), (0, 3, 10), (1, 2, 3), (2, 3, 1), (1, 3, 6)];
        let gra = Graph::new(4, es);

        assert_eq!(gra.find_path(0, 3), vec![0, 1, 2, 3]);
    }

    #[test]
    fn tsp() {
        let es = vec![(0, 1, 5), (0, 3, 10), (1, 2, 3), (2, 3, 1), (1, 3, 6)];
        let gra = Graph::new(4, es);

        assert_eq!(gra.tsp_path, vec![0, 1, 2, 3, 0]);
        assert_eq!(gra.tsp_cost, Some(19));
    }
}
