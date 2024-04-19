use core::cmp::min;

fn main() {
    const N: usize = 8;

    let objects: [(usize, usize); N] = 
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

    const B: usize = 645;

    let mut matrix: Vec<usize> = vec![0; N];

    let weights: Vec<usize> = objects.into_iter().map(|(w,_)| {w}).collect();
    let prices: Vec<usize> = objects.into_iter().map(|(_,p)| {p}).collect();

    let p_min = prices.iter().min().expect("Error while finding min price");
    let p_max = prices.iter().max().expect("Error while finding max price");
    let p_sum = prices.iter().sum::<usize>();

    println!("Min price: {p_min}; max price: {p_max}; sum: {p_sum}");

    let mut alpha_max: usize = 0;

    println!("Matrix: {}x{}\n", p_sum + 1, N);

    print!("      | ");
    for i in 0..N { print!("{i:>5} "); }
    println!(" | a_max");
    println!("{:-<1$}", "", 8 + 6*N + 8);

    for alpha in 1..(p_sum + 1)
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
