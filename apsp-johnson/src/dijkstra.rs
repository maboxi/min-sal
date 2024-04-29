use crate::distance::Distance;
use crate::heap::Heap;

pub fn dijkstra_apsp(edges: &Vec<(usize, usize, usize)>, n: usize) -> Vec<Distance<usize>> {
    let mut result: Vec<Distance<usize>> = Vec::with_capacity(n*n);

    for i in 0..n {
        let mut result_row: Vec<Distance<usize>> = dijkstra(edges, n, i).iter().map(|(_,d)| *d).collect();
        result.append(&mut result_row);
    }

    result
}

pub fn dijkstra(edges: &Vec<(usize, usize, usize)>, n: usize, start_index: usize) -> Vec<(usize, Distance<usize>)>{
    println!("\nCalculating dijkstra for start node {start_index}...");

    let mut nodes: Vec<(usize, Distance<usize>, bool)> = Vec::new();

    for i in 0..n {
        nodes.push((
            i,
            if i == start_index {Distance::from(0)} else { Distance::inf() },
            true
        ));
    }

    let mut heap: Heap<Distance<usize>> = Heap::from(nodes.iter().map(|(i, d, _)| (*d, *i)).collect());

    // create adjacency lists
    let mut adj_lists: Vec<Vec<(&usize, &usize)>> = vec![Vec::new(); n];
    for (i, j, d) in edges {
        adj_lists[*i].push((j, d));
        adj_lists[*j].push((i, d));
    }
    
    let mut cur_node: (Distance<usize>, usize);

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
                let adj_new_dist = cur_node.0 + **adj_dist;

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