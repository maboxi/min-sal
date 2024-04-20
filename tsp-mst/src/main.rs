fn main() {
    println!("Hello, world!");

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

    for (i, j, d) in edges
    {
        assert!(i <= numnodes && i > 0, "E ({i},{j},{d}): source vertex invalid!");
        assert!(j <= numnodes && j > 0, "E ({i},{j},{d}): dest vertex invalid!");
        assert!(i != j, "E ({i},{j},{d}): source vertex cannot be the same as dest vertex!");
        assert!(d > 0, "E ({i},{j},{d}): distance must be > 0!");
    }

    tsp_mst(numnodes, edges);
}

fn tsp_mst(n: usize, edges: Vec<(usize, usize, usize)>) -> usize
{
    println!("Using MST algorithmn on graph with {} nodes and {} vertices!", n, edges.len());
    return 0;
}
