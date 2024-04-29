use crate::dijkstra::dijkstra_apsp;
use crate::bellmann_ford::bellmann_ford;
use crate::distance::Distance;
use crate::util::print_matrix;

pub fn johnson(edges: &Vec<(usize, usize, i64)>, n: usize) -> Result<Vec<Distance<i64>>, & 'static str>{

    let mut edges_withdummy = edges.clone();

    for i in 0..n { edges_withdummy.push((n, i, 0)); }
    let start = n;

    let solution = bellmann_ford(&edges_withdummy, n + 1, start);
    
    let shift_values: Vec<i64> = match solution {
        Err(err) => {
            eprintln!("ERROR: Bellmann-Ford returned err: {err}");
            return solution
        }, 
        Ok(distances) => {
            distances.iter().map(|distance| distance.value()).collect()
        }
    };

    println!("Shift values (h(i)): ");
    shift_values.iter().enumerate().filter(|(i, _)| *i < n).for_each(|(i, d)| println!("\t{: >2}: {: >4}", i, d));

    let edges_adjusted: Vec<(usize, usize, usize)> = edges.iter()
        .map(|(u, v,d)| (*u, *v, (d + shift_values[*u] - shift_values[*v]) as usize))
        .collect();
    
    let mut apsp_adjusted: Vec<Distance<i64>> = dijkstra_apsp(&edges_adjusted, n).iter()
        .map(|dist| Distance::from_other(dist, |n| *n as i64)).collect();

    println!("Dijkstra results before readjustment:");
    print_matrix(&apsp_adjusted, n);

    for u in 0..n {
        for v in 0..n {
            apsp_adjusted[u*n + v] = apsp_adjusted[u*n + v] + shift_values[v] - shift_values[u];
        }
    }

    Ok(apsp_adjusted)
}