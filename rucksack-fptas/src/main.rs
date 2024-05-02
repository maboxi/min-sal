use std::io::{stdin, stdout, Write};
use core::str::FromStr;


fn userinput_or_default<T: FromStr>(default: T, preamble: &str) -> T {
    let mut userinput = String::new(); 
    
    print!("{}", preamble);
    stdout().flush().expect("Error flushing stdout!"); 

    stdin().read_line(&mut userinput).expect("Failed to read user input!");
    match userinput.trim().parse() {
        Ok(res) => res,
        Err(_) => {
            default
        }
    }
}

fn main() {
    /*
    let objects: Vec<(usize, usize)> = 
    [
    // Weight, Price
        (153, 232),
        (54, 73),
        (191, 201),
        (66, 50),
        (239, 141),
        (137, 79),
        (148, 48),
        (249, 38)
    ].into_iter().collect();
    */

    let objects: Vec<(usize, usize)> = [
        (75, 34),
        (34, 12),
        (7, 14),
        (58, 16),
        (72, 23),
        (43, 27),
        (26, 45)
    ].into_iter().collect();
    let N: usize = objects.len();

    let epsilon: f32 = 0.1;
    
    let prices_orig: Vec<_> = objects.clone().into_iter().map(|(_,p)| {p}).collect();

    let p_min = prices_orig.iter().min().expect("Error while finding min price");
    let p_max = prices_orig.iter().max().expect("Error while finding max price");
    let p_sum = prices_orig.clone().into_iter().sum::<usize>();

    let B = userinput_or_default(650, "Value B (max. weight) (default 650): ");
    let k: f32 = userinput_or_default(epsilon * *p_max as f32 / N as f32, "Input custom value for scaling constant 'k' or leave empty for default value: ");

    println!("Constants:");
    println!("    min price: {p_min}; max price: {p_max}; sum: {p_sum}");
    println!("    N: {N}; B: {B}; ε: {epsilon}; k: {k}");

    // conversion
    let objects: Vec<(usize, usize)> = objects.into_iter().map(|(w,p)| {
        let p_: f32 = p as f32 / k;
        (w, p_ as usize)
        }).collect();

    let (weights, prices) : (Vec<usize>, Vec<usize>) = objects.iter().cloned().unzip();

    let p_sum_adjusted = prices.clone().iter().sum::<usize>();

    println!("\nObjects:");
    print!("         | ");
    for i in 0..objects.len() { print!("{:>5} ", i+1); }
    print!("\n{:-<1$}", "", 10+6*objects.len());
    print!("\nWeights  | ");
    for w in &weights { print!("{:>5} ", *w); }
    print!("\nPrices   | ");
    for p in &prices_orig { print!("{:>5} ", *p); }
    print!("\nAdj. Pr. | ");
    for p in &prices { print!("{:>5} ", *p); }
    println!("\n");

    // print graph?
    print!("Print graph? (y/N) ");
    stdout().flush().expect("Error while flushing stdout!");
    let mut userinput = String::new();
    stdin().read_line(&mut userinput).expect("Error while reading user input!");
    println!("");
    let ui_trimmed = userinput.trim();

    let print_graph: bool = ui_trimmed == "y" || ui_trimmed == "yes";

    // print lines without non inf values?
    print!("Print lines without non inf values? (y/N) ");
    stdout().flush().expect("Error while flushing stdout!");
    let mut userinput = String::new();
    stdin().read_line(&mut userinput).expect("Error while reading user input!");
    println!("");
    let ui_trimmed = userinput.trim();

    let print_inf_lines: bool = ui_trimmed == "y" || ui_trimmed == "yes";


    // Start of algorithm
    println!("Calculating solution...");
    let mut alpha_max: usize = 0;

    println!("\n");
    if print_graph {
        println!("Matrix: {}x{}\n", p_sum_adjusted, N);
        print!("         | ");
        for i in 0..N { print!("{:>5} ", i+1); }
        println!(" | a_max");
        println!("{:-<1$}", "", 11 + 6*N + 8);
    }

    let num_or_inf = |n_opt: Option<usize>| { match n_opt { None => "∞".to_string(), Some(n) => n.to_string()}};

    let mut matrix: Vec<Option<usize>> = vec![Some(0); N];
    let ind = |i: usize, j: usize| i*N + j;

    let mut linestr: String;
    let mut hasnoninfvalues: bool;

    {
        use std::fmt::Write;
        for alpha in 1..(p_sum_adjusted + 1) {
            linestr = String::new();
            hasnoninfvalues = false;

            if print_graph {
                if alpha % 100 == 1 && alpha != 1 { println!(""); }
                write!(&mut linestr, "{alpha:>8} | ");
            }
            for i in 0..N
            {
                if i == 0
                {
                    if prices[i] == alpha { matrix.push(Some(weights[i])); }
                    else { matrix.push(None); }
                }
                else
                {
                    let sum_withnew = if alpha >= prices[i] {
                        matrix[ind(alpha-prices[i], i - 1)].map(|x| x + weights[i])
                    } else { None };

                    let sum_withoutnew = matrix[ind(alpha,i - 1)];

                    let sum_min = match sum_withnew {
                        None => sum_withoutnew,
                        Some(s1) => {
                            match sum_withnew {
                                None => Some(s1),
                                Some(s2) => {
                                    Some(if s1 < s2 { s1 } else { s2 })
                                }
                            }
                        }
                    };

                    matrix.push(sum_min);
                    if sum_min.is_some() { hasnoninfvalues = true; }
                }

                write!(&mut linestr, "{:>5} ", num_or_inf(matrix[matrix.len() - 1])).unwrap();
            } 

            let n_last_opt = matrix[matrix.len() -1];
            match n_last_opt {
                None => (),
                Some(n_last) => {
                    if n_last <= B { alpha_max = alpha; }
                }
            }
            writeln!(&mut linestr, " | {:>5}", alpha_max).unwrap();

            if print_graph && (print_inf_lines || hasnoninfvalues) {print!("{}", linestr); }
        }
    }

    if print_graph {println!("") };

    println!("Solution: adjusted price sum {} with weight {}\n",
        alpha_max,
        num_or_inf(matrix[alpha_max*N + N - 1]));

    // Backtracking of used objects
    let mut cur_index = N - 1;
    let mut cur_pricesum = alpha_max;
    let mut orig_pricesum = 0;
    let matrix_val = |row, col| matrix[row * N + col];
    let mut markers = vec![false; N];

    loop {
        if cur_index == 0 {
            if matrix_val(cur_pricesum, cur_index).is_some() && cur_pricesum != 0 {
                markers[0] = true;
                orig_pricesum += prices_orig[cur_index];
            }
            break;
        } else {
            if matrix_val(cur_pricesum, cur_index - 1) == matrix_val(cur_pricesum, cur_index) {
                cur_index -= 1;
            } else {
                markers[cur_index] = true;
                orig_pricesum += prices_orig[cur_index];
                cur_pricesum -= prices[cur_index];
                cur_index -= 1;
            }
        }
    }

    println!("Used objects: index (weight, adjusted price, original price)");
    for (i, marker) in markers.iter().enumerate() {
        if !marker {continue; }
        println!("              {: >5} ({: >5}, {: >5}, {: >5})", i + 1, weights[i], prices[i], prices_orig[i]);
    }
    println!("          => Sum:    {: >5}  {: >5}  {: >5}", num_or_inf(matrix_val(alpha_max, N-1)), alpha_max, orig_pricesum);

}
