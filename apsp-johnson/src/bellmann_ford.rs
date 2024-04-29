use crate::distance::Distance;

pub fn bellmann_ford(edges: &Vec<(usize, usize, i64)>, n: usize, start_index: usize) -> Result<Vec<Distance<i64>>, & 'static str> {
    println!("Starting Bellmann-Ford at node {start_index}...");

    let mut nodes: Vec<(Distance<i64>, Option<usize>)> = vec![(Distance::inf(), None); n];
    nodes[start_index].0 = Distance::from(0);

    for _ in 0..n-1 {
        for (u, v, d) in edges {
            // todo: relax
            let new_dist = nodes[*u].0 + *d;
            if nodes[*v].0 > new_dist {
                nodes[*v].0 = new_dist;
            }
        }
    }

    for (u, v, d) in edges {
        if nodes[*v].0 > (nodes[*u].0 + *d) {
            return Err("Found negative weight cycle!");
        }
    }

    println!("Bellmann-Ford done! Found {} nodes with path from start node {}", 
        nodes.iter().enumerate().filter(|(i, _)| *i != start_index).filter(|(_, (dist, _))| !dist.is_inf()).count(), 
        start_index);
    Ok(nodes.iter().map(|(d, _)| *d).collect())
}