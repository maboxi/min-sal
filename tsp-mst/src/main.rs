pub mod linkedlist;
use crate::linkedlist::LinkedList;

pub mod node;
use crate::node::Node;

pub mod tree;
use crate::tree::Tree;

fn main() {
    let numnodes: usize = 6;
    let edges: Vec<(usize, usize, usize)> = [
        (1, 2, 1),
        (1, 5, 2),
        (1, 6, 7),
        (2, 3, 3),
        (2, 6, 6),
        (3, 4, 4),
        (3, 6, 5),
        (4, 5, 6),
        (4, 6, 9),
        (5, 6, 8)
    ].to_vec();

    check_validity(numnodes, &edges);
    let res_mst = tsp_mst(numnodes, &edges);
    
    match res_mst {
        Err(e) => eprintln!("Error while solving TPS via MST: {e}"),
        Ok(res) => println!("MST-TSP Approximation: {res}"),
    }
}

fn check_validity(n: usize, edges: &Vec<(usize, usize, usize)>) {
    for (i, j, d) in edges
    {
        assert!(*i <= n && *i > 0, "E ({i},{j},{d}): source vertex invalid!");
        assert!(*j <= n && *j > 0, "E ({i},{j},{d}): dest vertex invalid!");
        assert!(*i != *j, "E ({i},{j},{d}): source vertex cannot be the same as dest vertex!");
        assert!(*d > 0, "E ({i},{j},{d}): distance must be > 0!");
    }

    println!("Validity check for graph complete! No problems found.")
}

fn print_hash(hash: &LinkedList<Node>) {
    println!("Contents of hash:");
    if hash.is_empty() {
        println!("  <empty hash>")
    } else {
        for node in hash.iter() {
            let dist_str = node.distance.map(|d| d.to_string()).unwrap_or("∞".to_string());
            let predecessor_str = node.predecessor.map(|d| d.to_string()).unwrap_or("-".to_string());
            println!("  Node {}: Distance {}, Predecessor: {}", node.index, dist_str, predecessor_str);
        }
    }
}

fn tsp_mst(n: usize, edges: &Vec<(usize, usize, usize)>) -> Result<usize, & 'static str> {
    let mut result: Result<usize, & 'static str> = Ok(usize::max_value());

    println!("\nUsing MST algorithmn on graph with {} nodes and {} vertices!", n, edges.len());

    // build adjacency matrix
    let mut adjacencymatrix: Vec<Vec<(usize, usize)>> = Vec::new();
    for x in 1..n + 1
    {
        let adjacencylist: Vec<(usize, usize)> = edges.iter()
            .filter(|(i,j,_)| *i == x || *j == x)
            .map(|(i,j, d)| if *i == x { (*j,*d) } else {(*i, *d)}).collect();
        adjacencymatrix.push(adjacencylist);
    }

    println!("Adjacency lists:");
    let mut i: usize = 1;
    for adjlist in &adjacencymatrix
    {
        print!("    Node {i}: ");
        for (j, dj) in adjlist {print!("{j}{dj} ");}
        println!("");
        i += 1;
    }

    let mut hash: LinkedList<Node> = LinkedList::new();

    for i in 1..n+1 
    {
        let d = if i == 1 {Some(0)} else {None};
        println!("\n    Inserting {i} ({d:?})...");
        hash.insert_ordered(Node{index: i, distance: d, predecessor: None});
    }


    println!("\n\n{0:-<1$} Starting TSP-MST {0:-<1$}", "", 32);

    println!("Contents of hash at start of algorithm:");
    print_hash(&hash);
    println!("{:-<1$}", "", 50);

    let mut mst: Tree<Node> = Tree::new();
    let mut edgesum: usize = 0;

    let mut markers = vec![true; n];
    while !hash.is_empty() {
        let curnode = hash.pop_front()?;
        
        let distance: usize;
        let index: usize = curnode.index;
        match curnode.distance {
            None => {
                println!("Current node: index {index}, distance ∞");
                eprintln!(" => Current node is not accessible by other nodes already in MST -> cannot run TSP");
                result = Err("Node {index} is not accessible in MST creation!");
                return result;
            },
            Some(dist) => { 
                distance = dist;
                edgesum += dist;
            }
        }

        println!("Current node: index {index}, distance {distance}");

        // add to mst
        mst.insert_search(Node{index: index, distance: Some(distance), predecessor: None}, &|node: &Node| node.index == curnode.predecessor.expect("Error: trying to insert node without predecessor into non-empty MST!"), &|node: &Node, mode: usize| {
            let dist_str = node.distance.map(|d| d.to_string()).unwrap_or("∞".to_string());
            let predecessor_str = node.predecessor.map(|d| d.to_string()).unwrap_or("-".to_string());

            match mode {
                0 => print!("First element in MST: "),
                1 => print!("Adding to children: "),
                2 => print!("Recursive send: "),
                _ => print!("Unknown debug code '{mode}': ")
            }
            
            println!("Node {}: Distance {}, Predecessor: {}", node.index, dist_str, predecessor_str);
        });
        
        // Update hash with adjacency list of current node
        markers[index - 1] = false;

        for (adjnode, adjdist) in &adjacencymatrix[index - 1] {
            print!("  Checking adjacency for {adjnode} ({adjdist}): ");
            if markers[adjnode - 1] {
                print!("Node is still in hash!\n    Checking if update of dist is needed");

                let node = hash.peek_search(|node| node.index == *adjnode)?;

                let do_update: bool;

                match node.distance {
                    None => {
                        do_update = true;
                        print!("; w({index},{adjnode})={adjdist} < ∞ -> Decrease key");
                    },
                    Some(distance) => {
                        do_update = *adjdist < distance;
                        if do_update {
                            print!("; w({index},{adjnode})={adjdist} < {distance} -> Decrease key");
                        } else {
                            print!("; w({index},{adjnode})={adjdist} >= {distance} -> No update needed!");
                        }
                    }
                }

                if do_update {
                    // decrease key
                    hash.delete_search(|node| node.index == *adjnode)?;
                    hash.insert_ordered(Node{index: *adjnode, distance: Some(*adjdist), predecessor: Some(index)});
                }
                
            } else {
                print!("Skipping node..."); 
            }
            println!("");
        }

        println!("\nHash after processing of node {index}:");
        print_hash(&hash);
        println!("{:-<1$}", "", 50);
    }

    println!("\n");

    println!("Sum of edges * 2 = {}", edgesum * 2);

    println!("MST after creation:");

    mst.print_preorder(&|n_opt: Option<&Node>| {
        match n_opt {
            None => println!("MST empty!"),
            Some(node) => {
                let dist_str = node.distance.map(|d| d.to_string()).unwrap_or("∞".to_string());
                let predecessor_str = node.predecessor.map(|d| d.to_string()).unwrap_or("-".to_string());
                println!("  Node {}: Distance {}, Predecessor: {}", node.index, dist_str, predecessor_str);
            }
        }
    });

    mst.print_singlenodes(&|t_opt| {
        match t_opt {
            None => "-".to_string(),
            Some(node) => node.index.to_string()
        }
    });

    return result;
}