use std::io::Read;

struct UnionFind {
    roots: Vec<usize>,
}
impl UnionFind {
    fn new(size: usize) -> Self {
        let mut roots = Vec::new();
        roots.resize(size, 0);
        for (i, e) in roots.iter_mut().enumerate() {
            *e = i;
        }
        UnionFind { roots }
    }
    fn find(&mut self, a: usize) -> usize {
        if self.roots[a] == a {
            a
        } else {
            let root = self.find(self.roots[a]);
            self.roots[a] = root;
            root
        }
    }
    fn union(&mut self, a: usize, b: usize) {
        let b = self.find(b);
        self.roots[b] = self.find(a);
    }
    fn check_same(&mut self, a: usize, b: usize) -> bool {
        self.find(a) == self.find(b)
    }
}

fn main() -> std::io::Result<()> {
    let mut buffer = String::new();
    std::io::stdin().read_to_string(&mut buffer)?;
    let rows: Vec<&str> = buffer.split('\n').collect();
    let mut rows = rows.iter();
    let row: Vec<usize> = rows
        .next()
        .unwrap()
        .split(' ')
        .map(|e: &str| e.parse::<usize>().unwrap())
        .collect();
    let (v, e) = (row[0], row[1]);
    let mut edges = Vec::new();
    #[derive(Debug, Eq, Ord, PartialEq, PartialOrd)]
    struct Edge {
        weight: usize,
        from: usize,
        to: usize,
    }
    for _ in 0..e {
        let row: Vec<usize> = rows
            .next()
            .unwrap()
            .split(' ')
            .map(|e: &str| e.parse::<usize>().unwrap())
            .collect();
        let (from, to, weight) = (row[0], row[1], row[2]);
        edges.push(Edge { from, to, weight });
    }
    edges.sort();
    let mut total_cost = 0;
    let mut uf = UnionFind::new(v);
    for edge in edges.iter() {
        if uf.check_same(edge.from, edge.to) {
            continue;
        }
        total_cost += edge.weight;
        uf.union(edge.from, edge.to);
    }
    println!("{}", total_cost);

    Ok(())
}
