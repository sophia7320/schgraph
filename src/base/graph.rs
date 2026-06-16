#[derive(Debug, Default)]
pub struct Graph {
    pub(super) cnt: usize,
    pub(super) matri: Vec<Vec<u64>>,
    pub(super) linking: Vec<Vec<(usize, u64)>>,
    pre: Vec<Vec<usize>>,
    pub(super) shortest: Vec<Vec<u64>>,

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

    pub fn find_path(&self, src: usize, dst: usize) -> (Vec<usize>, Option<u64>) {
        let mut res = vec![];

        let mut p = dst;

        if src >= self.cnt || dst >= self.cnt || self.pre[src][dst] == self.cnt {
            return (res, None);
        }

        loop {
            res.push(p);

            if p == src {
                break;
            }
            p = self.pre[src][p];
        }

        res.reverse();
        (res, Some(self.shortest[src][dst]))
    }

    fn tsp(&mut self) {
        let cnt = self.cnt;
        let max_mask: usize = 1 << cnt;

        let mut dp = vec![vec![u64::MAX / 2; cnt]; max_mask];
        let mut tsp_prev = vec![vec![cnt; cnt]; max_mask];

        dp[1][0] = 0;

        for mask in 0..max_mask {
            if (mask & 1) != 0 {
                for p in 0..self.cnt {
                    if mask & (1 << p) != 0 {
                        let mut temp = u64::MAX / 2;
                        let t_mask = mask ^ (1 << p);

                        for k in 0..self.cnt {
                            if t_mask & (1 << k) != 0 {
                                let candidate = dp[t_mask][k] + self.shortest[k][p];
                                if candidate < temp {
                                    temp = candidate;
                                    dp[mask][p] = candidate;
                                    tsp_prev[mask][p] = k;
                                }
                            }
                        }
                    }
                }
            }
        }

        let mut best_last: usize = 0;
        let mut best_cost = u64::MAX / 2;
        let full_mask = max_mask - 1;

        #[allow(clippy::needless_range_loop)]
        for node in 0..self.cnt {
            let candidate = dp[full_mask][node] + self.shortest[node][0];

            if candidate < best_cost {
                best_last = node;
                best_cost = candidate;
            }
        }

        if best_cost != u64::MAX / 2 {
            self.tsp_cost = Some(best_cost);
            let mut path = Graph::find_tsp_path_to_last(tsp_prev, best_last);
            path = self.tsp_to_realpath(path);
            path.extend(self.find_path(best_last, 0).0.drain(1..));
            self.tsp_path = path;
        } else {
            self.tsp_cost = None;
            self.tsp_path = vec![];
        }
    }

    fn find_tsp_path_to_last(prev: Vec<Vec<usize>>, last: usize) -> Vec<usize> {
        let n = prev[0].len();
        let mut res = vec![];

        let mut p = last;
        let mut mask: usize = (1 << n) - 1;
        res.push(p);

        while p != 0 {
            (p, mask) = (prev[mask][p], mask ^ (1 << p));
            res.push(p);
        }

        res.reverse();
        res
    }

    fn tsp_to_realpath(&self, tsp_path: Vec<usize>) -> Vec<usize> {
        let mut res = vec![];
        res.push(tsp_path[0]);
        for i in 0..(tsp_path.len() - 1) {
            res.extend(self.find_path(tsp_path[i], tsp_path[i + 1]).0.drain(1..));
        }
        res
    }

    pub fn tsp_path(&self) -> &Vec<usize> {
        &self.tsp_path
    }

    pub fn tsp_cost(&self) -> Option<u64> {
        self.tsp_cost
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

        assert_eq!(gra.find_path(0, 3).0, vec![0, 1, 2, 3]);
    }

    #[test]
    fn tsp() {
        let es = vec![(0, 1, 5), (0, 3, 10), (1, 2, 3), (2, 3, 1), (1, 3, 6)];
        let gra = Graph::new(4, es);

        assert_eq!(gra.tsp_path, vec![0, 1, 2, 3, 2, 1, 0]);
        assert_eq!(gra.tsp_cost, Some(18));
    }
}
