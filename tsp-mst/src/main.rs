use std::cmp::PartialOrd;

#[derive(Debug)]
struct LinkedList<T> (Option<(T, Box<LinkedList<T>>)>);

impl<T: PartialOrd + Clone> LinkedList<T>
{
    fn new() -> Self { LinkedList(None) }

    fn is_empty(&self) -> bool { self.0.is_none() }

    fn insert_front(&mut self, data: T)
    {
        let tail = self.0.take();
        self.0 = Some( (data, Box::new(LinkedList(tail))) );
    }

    fn insert_back(&mut self, data: T)
    {
        match self.0
        {
            None => self.0 = Some( (data, Box::new(LinkedList(None)))),
            Some((_, ref mut tail)) => tail.insert_back(data),
        }
    }

    fn insert_at<F>(&mut self, data: T, comp: F)
    where
        F: Fn(&T, &T) -> bool
    {
        match self.0
        {
            None => self.insert_back(data),
            Some((ref mydata, ref mut tail)) => 
                if comp(&mydata, &data) { self.insert_front(data) }
                else { tail.insert_at(data, comp) }
        }
    }

    fn pop_front(&mut self) -> Option<T>
    {
        match self.0
        {
            None => None,
            Some((ref mut data, ref mut ll)) => {
                let front = data.clone();
                self.0 = ll.0.take();
                Some(front)
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
    
    println!("MST-TSP Approximation: {res_mst}");
}

fn check_validity(n: usize, edges: &Vec<(usize, usize, usize)>)
{
    for (i, j, d) in edges
    {
        assert!(*i <= n && *i > 0, "E ({i},{j},{d}): source vertex invalid!");
        assert!(*j <= n && *j > 0, "E ({i},{j},{d}): dest vertex invalid!");
        assert!(*i != *j, "E ({i},{j},{d}): source vertex cannot be the same as dest vertex!");
        assert!(*d > 0, "E ({i},{j},{d}): distance must be > 0!");
    }

    println!("")
}

fn tsp_mst(n: usize, edges: &Vec<(usize, usize, usize)>) -> usize
{
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
    for adjlist in adjacencymatrix
    {
        print!("    Node {i}: ");
        for j in adjlist {print!("{j} ");}
        println!("");
        i += 1;
    }

    let mut hash: LinkedList<(usize,Option<usize>)> = LinkedList::new();

    for i in 1..n+1 
    {
        let data = if i == 1 {Some(0)} else {None};
        hash.insert_at((i, data), |(_, d1), (_, d2)| -> bool {
            match d1 {
                None => {
                    match d2 {
                        None => false,
                        Some(_) => true,
                    }
                },
                Some(d1_) => {
                    match d2 {
                        None => false,
                        Some(d2_) => d1_ > d2_,
                    }
                }
            }
        });
    }

    let markers = vec![false; n];
    while !hash.is_empty()
    {
        let curnode = hash.pop_front();
        
        match curnode {
            None => break,
            Some((index, data_option)) => {
                match data_option {
                    None => println!("Current node: index {index}, distance ∞"),
                    Some(dist) => println!("Current node: index {index}, distance {dist}")
                }
            } 
        }

        /*
        hash.insert_at((i, None), |(_, d1), (_, d2)| {
            match d1 {
                None => {
                    match d2 {
                        None => false,
                        Some(_) => true,
                    }
                },
                Some(d1_) => {
                    match d2 {
                        None => false,
                        Some(d2_) => d1_ > d2_
                    }
                }
            }
        });
        */
        break;
    }

    return usize::max_value();
}

