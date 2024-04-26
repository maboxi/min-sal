use std::io::{stdin, stdout, Write};
use core::str::FromStr;
use core::fmt::Debug;

pub mod heap;
use crate::heap::Heap;

pub mod node;
use crate::node::Node;

fn main() {
    let n: usize = 9;
    let edges: Vec<(usize, usize, usize)> = [
        (0, 1, 4),
        (0, 7, 8),
        (1, 2, 8),
        (1, 7, 11),
        (2, 3, 7),
        (2, 5, 4),
        (2, 8, 2),
        (3, 4, 9),
        (3, 5, 14),
        (4, 5, 10),
        (5, 6, 2),
        (6, 7, 1),
        (6, 8, 6),
        (7, 8, 7)
    ].to_vec();

    println!("Edges:");
    for (i, j, d) in &edges { println!("    {: >2} <-> {: <2} = {: >2}", i, j, d); }

    let start: usize = get::<usize>(format!("Select index of starting node (0..{}) (0): ", n - 1), 0);

    let solution = dijkstra(edges, n, start);


    println!("Distances to node {start} from:");
    let mut i = 0;
    solution.iter().for_each(|dist| {
        if i != start {
            println!("    Node {}: {}", i, num_or_inf(dist));
        }
        i += 1;
    });
}


fn dijkstra(edges: Vec<(usize, usize, usize)>, n: usize, start_index: usize) -> Vec<Option<usize>>{
    println!("\nCalculating dijkstra for start node {start_index}...");

    let distances = vec![None; n];
    let markers: [bool] = [true; n];

    let mut heap: Heap<Node> = Heap::from((0..n)
        .map(|x| Node {index: x, distance: if x == start_index { Some(0) } else { None }})
        .collect());
    
    // create adjacency lists
    let mut adj_lists: Vec<Vec<(&usize, &usize)>> = vec![Vec::new(); n];
    for (i, j, d) in &edges {
        adj_lists[*i].push((j, d));
        adj_lists[*j].push((i, d));
    }
    
    let mut cur_node: Node;

    while !heap.is_empty() {
        cur_node = heap.extract_max().unwrap();
        let distance: usize;
        match cur_node.distance {
            None => continue,
            Some(dist) => distance = dist,
        }

        for (adj_node, adj_dist) in &adj_lists[cur_node.index] {
            if markers[adj_node] {
                let new_dist = match distances[adj_node] {
                    None => Some(distance + adj_dist),
                    Some(curdist) => if curdist > distance + adj_dist { Some(distance + adj_dist)) } else { None }
                }

                if !new_dist.is_none() {

                }
            }
        }
    }

    return distances;
}

fn get<T: FromStr>(preamble: String, default: T) -> T where <T as FromStr>::Err: Debug {
    print!("{}", preamble);
    stdout().flush().expect("Error flushing stdout!");
    let mut userinput = String::new();
    stdin().read_line(&mut userinput).expect("Error reading user input!");
    userinput.trim().parse::<T>().unwrap_or(default)
}

fn num_or_inf(n_opt: &Option<usize>) -> String { match n_opt { None => "∞".to_string(), Some(n) => n.to_string()}}