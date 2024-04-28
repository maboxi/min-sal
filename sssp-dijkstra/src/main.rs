use std::io::{stdin, stdout, Write};
use core::str::FromStr;
use core::fmt::Debug;

pub mod heap;
use crate::heap::Heap;

pub mod distance;
use crate::distance::Distance;

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
    solution.iter().for_each(|(i, dist)| {
        if *i != start {
            println!("    Node {}: {}", i, dist);
        }
    });
}


fn dijkstra(edges: Vec<(usize, usize, usize)>, n: usize, start_index: usize) -> Vec<(usize, Distance)>{
    println!("\nCalculating dijkstra for start node {start_index}...");

    let mut nodes: Vec<(usize, Distance, bool)> = Vec::new();

    for i in 0..n {
        nodes.push((
            i,
            Distance(if i == start_index {Some(0)} else {None}),
            true
        ));
    }

    let mut heap: Heap<Distance> = Heap::from(nodes.iter().map(|(i, d, _)| (*d, *i)).collect());

    // create adjacency lists
    let mut adj_lists: Vec<Vec<(&usize, &usize)>> = vec![Vec::new(); n];
    for (i, j, d) in &edges {
        adj_lists[*i].push((j, d));
        adj_lists[*j].push((i, d));
    }
    
    let mut cur_node: (Distance, usize);

    while !heap.is_empty() {
        cur_node = heap.extract_min().unwrap();
        nodes[cur_node.1].2 = false;
        nodes[cur_node.1].1 = cur_node.0;

        println!("Took {} out of heap with distance {}", cur_node.1, cur_node.0.num_or_inf());

        if cur_node.0.is_inf() {
            println!("  Node {} has no connection to {}", cur_node.1, start_index);
            continue;
        }

        for (adj_node, adj_dist) in &adj_lists[cur_node.1] {
            print!("    Checking adj node {} with adj dist {}... ", adj_node, adj_dist);
            if nodes[**adj_node].2{
                let adj_new_dist = cur_node.0 + *adj_dist;

                if !adj_new_dist.is_inf() {
                    let adj_cur_dist = heap.get_key(**adj_node); 
                    if adj_new_dist < adj_cur_dist {
                        print!("Updating from distance {} to {}", adj_cur_dist, adj_new_dist);
                        heap.decrease_key(**adj_node, adj_new_dist);
                    } else {
                        print!("Key in heap: {} shortert than new distance {}", adj_cur_dist, adj_new_dist);
                    }
                }
            } else {
                print!("already found shortest path!");
            }
            println!("");
        }
    }

    println!("\nDijkstra completed!");

    return nodes.iter().map(|(i, d, _)| (*i, *d)).collect();
}

fn get<T: FromStr>(preamble: String, default: T) -> T where <T as FromStr>::Err: Debug {
    print!("{}", preamble);
    stdout().flush().expect("Error flushing stdout!");
    let mut userinput = String::new();
    stdin().read_line(&mut userinput).expect("Error reading user input!");
    userinput.trim().parse::<T>().unwrap_or(default)
}
