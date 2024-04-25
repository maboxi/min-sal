use core::cmp::min;

fn main() {
    const N: usize = 8;
    const EPSILON: f32 = 0.1;
    const B: usize = 645;

    let objects_: [(usize, usize); N] = 
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
    ];

    let objects: Vec<(usize, usize)> = objects_.into_iter().collect();

    //let weights_orig: Vec<usize> = objects.clone().into_iter().map(|(w,_)| {w}).collect();
    let prices_orig: Vec<_> = objects.iter().map(|(_,p)| {p}).collect();

    let p_min = prices_orig.iter().min().expect("Error while finding min price");
    let p_max = prices_orig.iter().max().expect("Error while finding max price");
    let p_sum = prices_orig.clone().into_iter().sum::<usize>();

    let k: f32 = 1.0; // EPSILON * *p_max as f32 / N as f32;

    println!("N: {N}; min price: {p_min}; max price: {p_max}; sum: {p_sum}");
    println!("EPSILON: {EPSILON} => k = EPSILON * p_max / N = {k}");

    println!("\nBefore Conversion: ");
    print_objects(&objects);

    // conversion
    let objects: Vec<(usize, usize)> = objects.into_iter().map(|(w,p)| {
        let p_: f32 = p as f32 / k;
        (w, p_ as usize)
        }).collect();

    let (weights, prices) : (Vec<usize>, Vec<usize>) = objects.iter().cloned().unzip();

    let p_sum_adjusted = prices.clone().iter().sum::<usize>();

    println!("\nAfter Conversion: ");
    print_objects(&objects);

    println!("\nNew sum: {p_sum_adjusted}");

    // Start of algorithm
    let mut alpha_max: usize = 0;

    println!("\n");
    println!("Matrix: {}x{}\n", p_sum_adjusted + 1, N);

    print!("      | ");
    for i in 0..N { print!("{:>5} ", i+1); }
    println!(" | a_max");
    println!("{:-<1$}", "", 8 + 6*N + 8);

    let num_or_inf = |n_opt: Option<usize>| { match n_opt { None => "∞".to_string(), Some(n) => n.to_string()}};

    let mut matrix: Vec<Option<usize>> = vec![Some(0); N];
    for alpha in 1..(p_sum_adjusted + 1)
    {
        if alpha % 100 == 1 && alpha != 1{ println!(""); }
        print!("{alpha:>5} | ");
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
                    matrix[(alpha-prices[i]) * N + i - 1].map(|x| x + weights[i])
                } else { None };

                let sum_withoutnew = matrix[alpha*N + i - 1];

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
            }

            print!("{:>5} ", num_or_inf(matrix[matrix.len() - 1]));
        } 

        let n_last_opt = matrix[matrix.len() -1];
        match n_last_opt {
            None => (),
            Some(n_last) => {
                if n_last <= B { alpha_max = alpha; }
            }
        }
        println!(" | {alpha_max:>5}");
    }

    println!("");
    println!("a_max: {alpha_max}");
    println!(" => p_max: {}", num_or_inf(matrix[alpha_max*N + N - 1]));
}

fn print_objects(objects: &Vec<(usize, usize)>)
{
    print!("        | ");
    for i in 0..objects.len() { print!("{:>5} ", i+1); }
    println!("\n{:-<1$}", "", 10+6*objects.len());
    print!("\nWeights | ");
    for (w, _) in objects { print!("{w:>5} "); }
    print!("\nPrices  | ");
    for (_, p) in objects { print!("{p:>5} "); }
    println!("");
}
