pub mod distance;

pub mod util;
use util::print_matrix;

pub mod johnson;
use johnson::johnson;

pub mod dijkstra;

pub mod bellmann_ford;

pub mod heap;


fn main() {
    println!("Hello, world!");

    let edges: Vec<(usize, usize, i64)> = [
        (0, 1, 3),
        (0, 2, 7),
        (0, 3, -4),
        (1, 3, 8),
        (1, 4, 1),
        (2, 1, 4),
        (3, 4, 6),
        (4, 0, 2),
        (4, 2, -5)
        /*
        // Cormen
        (0, 1, 3),
        (0, 2, 8),
        (0, 4, -4),
        (1, 3, 1),
        (1, 4, 7),
        (2, 1, 4),
        (3, 2, -5),
        (3, 0, 2),
        (4, 3, 6),
        */

        /*
        (0, 1, -5),
        (0, 2, 2),
        (1, 2, 4),
        (2, 3, 1),
        (2, 3, 7),
        */
    ].to_vec();
    let n: usize = *edges.iter().map(|(i, j, _)| i.max(j)).max().unwrap() + 1;

    println!("Edges (n={n}):");
    edges.iter().for_each(|(i,j,d)| println!("\t{: >2} <-> {: <2} = {: >2}", i, j, d) );

    let apsp_johnson_res = johnson(&edges, n);

    match apsp_johnson_res {
        Err(err) => {
            eprintln!("Error occured during Johnson's alogrithm: {err}");
        },
        Ok(apsp_johnson) => {
    	    println!("Resulting APSP matrix after readjustments: {n}x{n}");
    	    print_matrix(&apsp_johnson, n);
        }
    }

}