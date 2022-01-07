#[derive(Debug, Copy, Clone)]
struct Entry {
    node: usize,
    distance: usize,
}

fn solve(adj: &Vec<Vec<Entry>>, colors: &mut Vec<usize>, n: usize, parent: usize, col: usize) {
    colors[n] = col;

    for next in &adj[n] {
        if next.node == parent {
            continue;
        }
        solve(
            adj,
            colors,
            next.node,
            n,
            if next.distance & 1 == 1 { col ^ 1 } else { col },
        )
    }
}
fn main() -> std::io::Result<()> {
    use std::io::Read;
    let mut buffer = String::new();
    std::io::stdin().read_to_string(&mut buffer)?;
    let rows: Vec<&str> = buffer.split('\n').collect();
    let mut iter = rows.iter();
    let n = iter.next().unwrap().parse::<usize>().unwrap();

    //println!("N={}", n);

    let mut adj: Vec<Vec<Entry>> = Vec::new();
    adj.resize(n, vec![]);
    for _ in 0..(n - 1) {
        let cols: Vec<&str> = iter.next().unwrap().split(' ').collect();
        let lhs = cols[0].parse::<usize>().unwrap();
        let rhs = cols[1].parse::<usize>().unwrap();
        let distance = cols[2].parse::<usize>().unwrap();
        adj[lhs - 1].push(Entry {
            node: rhs - 1,
            distance,
        });
        adj[rhs - 1].push(Entry {
            node: lhs - 1,
            distance,
        });
    }

    //println!("{:?}", adj);

    let mut colors: Vec<usize> = Vec::new();
    colors.resize(n, 0);
    solve(&adj, &mut colors, 0, n /* non-existent node */, 0);

    //println!("{:?}", colors);
    for v in colors {
        println!("{}", v);
    }

    Ok(())
}
