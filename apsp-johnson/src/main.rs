pub mod distance;
use distance::Distance;

pub mod util;

fn main() {
    println!("Hello, world!");

    let n: usize = 4;
    let edges: Vec<(usize, usize, i64)> = [
        (0, 1, -5),
        (0, 2, 2),
        (1, 2, 4),
        (2, 3, 1),
        (2, 3, 7),
    ].to_vec();

    println!("Edges:");
    for (i, j, d) in &edges { println!("    {: >2} <-> {: <2} = {: >2}", i, j, d); }

    let start: usize = util::get::<usize>(format!("Select index of starting node (0..{}) (0): ", n - 1), 0);

    let solution = bellmann_ford(edges, n, start);
    println!("{:#?}", solution);
}

fn bellmann_ford(edges: Vec<(usize, usize, i64)>, n: usize, start_index: usize) -> Result<Vec<Distance<i64>>, & 'static str> {
    let mut nodes: Vec<(Distance<i64>, Option<usize>)> = vec![(Distance::inf(), None); n];
    nodes[start_index].0 = Distance::from(0);

    for _ in 0..n-1 {
        for (u, v, d) in &edges {
            // todo: relax
            let new_dist = nodes[*u].0 + *d;
            if nodes[*v].0 > new_dist {
                nodes[*v].0 = new_dist;
            }
        }
    }

    for (u, v, d) in &edges {
        if nodes[*v].0 > (nodes[*u].0 + *d) {
            return Err("Found negative weight cycle!");
        }
    }

    Ok(nodes.iter().map(|(d, _)| *d).collect())
}