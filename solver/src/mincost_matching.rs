// https://github.com/koba-e964/contest/blob/master/comm/graph/MinCostFlow.rs

// Minimum cost flow.
// Verified by: yukicoder No.1301 Strange Graph Shortest Path
//              (https://yukicoder.me/submissions/590401)
//              AtCoder Library Practice Contest - E
//              (https://atcoder.jp/contests/practice2/submissions/22478556)
//              ACL Contest 1 - C
//              (https://atcoder.jp/contests/acl1/submissions/23898415)

pub type Cost = i64;
type Cap = isize;
#[derive(Debug, Clone, Copy)]
struct Edge {
    to: usize,
    cap: Cap,
    cost: Cost,
    rev: usize, // rev is the position of reverse edge in graph[to]
}

#[derive(Debug, Clone)]
struct MinCostFlow {
    n: usize,
    graph: Vec<Vec<Edge>>,
    h: Vec<Cost>,              // potential,
    dist: Vec<Cost>,           // minimum distance
    prev: Vec<(usize, usize)>, // previous vertex and edge
}

impl MinCostFlow {
    // Initializes this solver. n is the number of vertices.
    fn new(n: usize) -> Self {
        MinCostFlow {
            n,
            graph: vec![vec![]; n],
            h: vec![0; n],
            dist: vec![0; n],
            prev: vec![(0, 0); n],
        }
    }
    fn add_edge(&mut self, from: usize, to: usize, cap: Cap, cost: Cost) {
        let fst = Edge {
            to: to,
            cap: cap,
            cost: cost,
            rev: self.graph[to].len(),
        };
        self.graph[from].push(fst);
        let snd = Edge {
            to: from,
            cap: 0,
            cost: -cost,
            rev: self.graph[from].len() - 1,
        };
        self.graph[to].push(snd);
    }
    // Calcucates the minimum cost flow
    // whose source is s, sink is t, and flow is f.
    fn min_cost_flow(&mut self, s: usize, t: usize, mut f: Cap) -> Cost {
        let n = self.n;
        let inf: Cost = std::i64::MAX / 10; // ?????
        let mut res = 0;
        let h = &mut self.h;
        let dist = &mut self.dist;
        while f > 0 {
            let mut que = std::collections::BinaryHeap::<(Cost, usize)>::new();
            for i in 0..n {
                dist[i] = inf;
            }
            dist[s] = 0;
            que.push((0, s));
            while let Some((d, v)) = que.pop() {
                let d = -d;
                if dist[v] < d {
                    continue;
                }
                for (i, &e) in self.graph[v].iter().enumerate() {
                    if e.cap > 0 && dist[e.to] > dist[v] + e.cost + h[v] - h[e.to] {
                        dist[e.to] = dist[v] + e.cost + h[v] - h[e.to];
                        self.prev[e.to] = (v, i);
                        que.push((-dist[e.to], e.to));
                    }
                }
            }
            if dist[t] == inf {
                return -1; // Cannot add flow anymore
            }
            for i in 0..n {
                h[i] += dist[i];
            }
            // Add flow fully
            let mut d = f;
            let mut i = t;
            while i != s {
                let (pv, pe) = self.prev[i];
                d = std::cmp::min(d, self.graph[pv][pe].cap);
                i = pv;
            }
            f -= d;
            res += d as Cost * h[t];
            i = t;
            while i != s {
                let (pv, pe) = self.prev[i];
                self.graph[pv][pe].cap -= d;
                let erev = self.graph[pv][pe].rev;
                self.graph[i][erev].cap += d;
                i = pv;
            }
        }
        return res;
    }
}

/// distance_matrix[i][j] := i を j に割り当てるためのコスト
/// ret[i] = j := i を j に割り当てる
pub fn mincost_matching(distance_matrix: &Vec<Vec<Cost>>) -> (Cost, Vec<usize>) {
    let dim = distance_matrix.len();

    let mut flow = MinCostFlow::new(2 * dim + 2);

    let start_node = 2 * dim;
    let last_node = 2 * dim + 1;
    for i in 0..dim {
        flow.add_edge(start_node, i, 1, 0);
        flow.add_edge(dim + i, last_node, 1, 0);
    }

    for i in 0..dim {
        for j in 0..dim {
            flow.add_edge(i, dim + j, 1, distance_matrix[i][j]);
        }
    }

    let min_cost = flow.min_cost_flow(start_node, last_node, dim as Cap);

    let mut ret = vec![0; dim];
    for i in 0..dim {
        for j in 0..dim {
            let mut found = false;
            for e in flow.graph[i].iter() {
                if e.to == dim + j && e.cap == 0 {
                    ret[i] = j;
                    found = true;
                    break;
                }
            }
            if found {
                break;
            }
        }
    }
    (min_cost, ret)
}

#[cfg(test)]
mod tests {
    use super::mincost_matching;

    #[test]
    fn test_mincost_matching() {
        let distance = vec![vec![2, 3, 4, 1], vec![3, 4, 1, 2], vec![4, 1, 2, 3], vec![1, 2, 3, 4]];
        // distance[0][3] = 1
        // distance[1][2] = 1
        // distance[2][1] = 1
        // distance[3][0] = 1

        let (min_cost, matching) = mincost_matching(&distance);
        assert_eq!(min_cost, 4);
        assert_eq!(matching, vec![3, 2, 1, 0]);
    }
}
