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
    let prices_orig: Vec<usize> = objects.clone().into_iter().map(|(_,p)| {p}).collect();

    let p_min = prices_orig.clone().into_iter().min().expect("Error while finding min price");
    let p_max = prices_orig.clone().into_iter().max().expect("Error while finding max price");
    let p_sum = prices_orig.clone().into_iter().sum::<usize>();

    let k: f32 = 10.0; // EPSILON * *p_max as f32 / N as f32;

    println!("N: {N}; min price: {p_min}; max price: {p_max}; sum: {p_sum}");
    println!("EPSILON: {EPSILON} => k = EPSILON * p_max / N = {k}");

    println!("\nBefore Conversion: ");
    print_objects(&objects);

    // conversion
    let objects: Vec<(usize, usize)> = objects.into_iter().map(|(w,v)| {
        let v_: f32 = v as f32 / k;
        (w, v_ as usize)//(w, v_.to_uint().expect("Cannot convert {v} to usize!"))
        }).collect();

    let weights: Vec<usize> = objects.clone().into_iter().map(|(w,_)| {w}).collect();
    let prices: Vec<usize> = objects.clone().into_iter().map(|(_,p)| {p}).collect();

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

    let mut matrix: Vec<usize> = vec![0; N];
    for alpha in 1..(p_sum_adjusted + 1)
    {
        if alpha % 100 == 1 && alpha != 1{ println!(""); }
        print!("{alpha:>5} | ");
        for i in 0..N
        {
            if i == 0
            {
                if prices[i] == alpha { matrix.push(weights[i]); }
                else { matrix.push(0); }
            }
            else
            {
                let mut sum_withnew = 0;
                if alpha >= prices[i] { 
                    if matrix[(alpha-prices[i]) * N + i - 1] > 0 || alpha - prices[i] == 0 {
                        sum_withnew = matrix[(alpha-prices[i]) * N + i - 1] + weights[i];
                    }
                    else { sum_withnew = 0; }
                }

                let sum_withoutnew = matrix[alpha*N + i - 1];
                /*
                if alpha == 39 || alpha == 38 { 
                    print!("<{sum_withnew},{sum_withoutnew},{}>", 
                                (if alpha >= prices[i] {alpha-prices[i]} else {0}));
                }
                */

                if sum_withnew == 0 { matrix.push(sum_withoutnew); }
                else if sum_withoutnew == 0 { matrix.push(sum_withnew); }
                else { matrix.push(min(sum_withnew, sum_withoutnew)); }
            }

            print!("{:>5} ", matrix[matrix.len() - 1]);
        } 

        if matrix[matrix.len() -1] != 0 && matrix[matrix.len() - 1] <= B { alpha_max = alpha; }
        println!(" | {alpha_max:>5}");
    }

    println!("");
    println!("a_max: {alpha_max}");
}

fn print_objects(objects: &Vec<(usize, usize)>)
{
    print!("        | ");
    for i in 0..objects.len() { print!("{:>5} ", i+1); }
    println!("\n{:-<1$}", "", 10+6*objects.len());
    print!("\nObjects | ");
    for (w, _) in objects { print!("{w:>5} "); }
    print!("\nPrices  | ");
    for (_, p) in objects { print!("{p:>5} "); }
    println!("");
}