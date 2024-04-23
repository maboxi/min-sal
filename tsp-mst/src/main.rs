use std::cmp::{PartialEq, PartialOrd, Ordering};

#[derive(Debug, Clone)]
struct Node {
    index: usize,
    distance: Option<usize>
}

impl PartialEq for Node {
    fn eq(&self, other: &Self) -> bool {
        self.index == other.index
    }
}

impl PartialOrd for Node {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering>{
        match self.distance {
            None => {
                 match other.distance {
                    None => None,
                    Some(_) => Some(Ordering::Greater),
                 }
            },
            Some(d1) => {
                match other.distance {
                    None => Some(Ordering::Less),
                    Some(d2) => d1.partial_cmp(&d2)
                }
            }
        }
    }
}

#[derive(Debug)]
struct LinkedList<T> (Option<(T, Box<LinkedList<T>>)>);

impl<T: PartialOrd + PartialEq + Clone> LinkedList<T> {
    fn new() -> Self { LinkedList(None) }

    fn is_empty(&self) -> bool { self.0.is_none() }

    fn insert_front(&mut self, data: T) {
        let tail = self.0.take();
        self.0 = Some( (data, Box::new(LinkedList(tail))) );
    }

    fn insert_back(&mut self, data: T) {
        match self.0 {
            None => self.0 = Some( (data, Box::new(LinkedList(None)))),
            Some((_, ref mut tail)) => tail.insert_back(data),
        }
    }

    fn insert_at<F>(&mut self, data: T, comp: F)
    where
        F: Fn(&T, &T) -> bool
    {
        match self.0 {
            None => {
                self.insert_back(data)
            },
            Some((ref mydata, ref mut tail)) => {
                if comp(&mydata, &data)
                {
                    self.insert_front(data);
                }
                else { tail.insert_at(data, comp); }
            }
        }
    }

    fn insert_ordered(&mut self, data: T) {
        match self.0 {
            None => self.insert_back(data),
            Some((ref mydata, ref mut tail)) => {
                if mydata > &data {
                    self.insert_front(data);
                } else {
                    tail.insert_ordered(data);
                }
            }
        }
    }

    fn pop_front(&mut self) -> Result<T,& 'static str> {
        match self.0 {
            None => Err("Linked list is empty!"),
            Some((ref mut data, ref mut ll)) => {
                let front = data.clone();
                self.0 = ll.0.take();
                Ok(front)
            }
        }
    }

    fn pop_search<F>(&mut self, search: F) -> Result<T,& 'static str>  
    where
        F: Fn(&T) -> bool
    {
        match self.0 {
            None => Err("Cannot pop element because linked list is empty!"),
            Some((ref mydata, ref mut tail)) => {
                if search(&mydata)
                {
                    self.pop_front()
                }
                else { tail.pop_search(search) }
            }
        }
    }

    fn peek_front(&self) -> Result<&T,& 'static str> {
        match &self.0 {
            None => Err("Linked list is empty!"),
            Some((data, _)) => Ok(&data),
        }
    }

    fn peek_search<F>(&self, search: F) -> Result<&T, & 'static str>
    where
        F: Fn(&T) -> bool
    {
        match self.0 {
            None => {
                Err("Cannot peek at element because the element was not found or the list is empty!")
            },
            Some((ref mydata, ref tail)) => {
                if search(&mydata) {
                    self.peek_front()
                }
                else { tail.peek_search(search) }
            }
        }
    }

    fn iter(&self) -> LinkedListIter<'_, T> { LinkedListIter{cur: self} }
}

struct LinkedListIter<'a, T> {
    cur: &'a LinkedList<T>,
}

impl<'a, T> Iterator for LinkedListIter<'a, T> {
    type Item = &'a T;

    fn next(&mut self) -> Option<Self::Item> {
        match &self.cur.0 {
            None => None,
            Some((data, ll_)) => {
                self.cur = &ll_;
                Some(&data)
            }
        }
    }
}

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
    for node in hash.iter() {
        match node.distance {
            None => println!("  Node {}: Distance ∞", node.index),
            Some(distance) => println!("  Node {}: Distance {distance}", node.index)
        }
    }
}

fn tsp_mst(n: usize, edges: &Vec<(usize, usize, usize)>) -> Result<usize, & 'static str> {
    let mut result: Result<usize, & 'static str> = Err("Error before start of algorithm!");

    println!("\nUsing MST algorithmn on graph with {} nodes and {} vertices!", n, edges.len());

    // build adjacency matrix
    let mut adjacencymatrix: Vec<Vec<usize>> = Vec::new();
    for x in 1..n + 1
    {
        let adjacencylist: Vec<usize> = edges.iter()
            .filter(|(i,j,_)| *i == x || *j == x)
            .map(|(i,j,_)| if *i == x {*j} else {*i}).collect();
        adjacencymatrix.push(adjacencylist);
    }

    println!("Adjacency lists:");
    let mut i: usize = 1;
    for adjlist in &adjacencymatrix
    {
        print!("    Node {i}: ");
        for j in adjlist {print!("{j} ");}
        println!("");
        i += 1;
    }

    let mut hash: LinkedList<Node> = LinkedList::new();

    for i in 1..n+1 
    {
        let d = if i == 1 {Some(0)} else {Some(i*2) /*None*/};
        println!("\n    Inserting {i} ({d:?})...");
        hash.insert_ordered(Node{index: i, distance: d});
    }

    print_hash(&hash);

    println!("\n\n{0:-<1$} Starting TSP-MST {0:-<1$}", "", 32);

    let mut markers = vec![true; n];
    while !hash.is_empty() {
        let curnode = hash.pop_front();
        
        let distance: usize;
        let index: usize;

        match curnode {
            Err(errmsg) => {
                println!("Hash now empty!");
                eprintln!("{errmsg}");
                break
            },
            Ok(node) => {
                index = node.index;
                match node.distance {
                    None => {
                        println!("Current node: index {index}, distance ∞");
                        eprintln!(" => Current node is not accessible by other nodes already in MST -> cannot run TSP");
                        result = Err("Node {index} is not accessible in MST creation!");
                        return result;
                    },
                    Some(dist) => { 
                        distance = dist;
                    }
                }
            } 
        }

        println!("Current node: index {index}, distance {distance}");
        
        // Update hash with adjacency list of current node
        markers[index - 1] = false;

        for adjnode in &adjacencymatrix[index - 1] {
            print!("  Checking adjacency for {adjnode}: ");
            if markers[adjnode - 1] {
                print!("Node is still in hash!\n    Checking if update of dist is needed");

                match hash.peek_search(|node| node.index == *adjnode) {
                    Err(E) => { 
                        println!("Error regarding node {adjnode} in adjacency list check for {index}: {E}");
                        result = Err("Error occured during adjacency list distance update!"); break; }
                    Ok(node) => {

                    }
                }

            } else { print!("Skipping node..."); }
            println!("");
        }
    }

    return result;
}