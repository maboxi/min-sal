pub mod fibonacci_heap;
use graphviz_rust::printer::{DotPrinter, PrinterContext};

use crate::fibonacci_heap::FibonacciHeap;


fn main() {
    let mut fheap = FibonacciHeap::new();
    let timestamp = format!("{}", chrono::prelude::Utc::now().format("%Y%m%d-%H%M"));
    println!("Timestamp: {timestamp}");

    let data = [23, 7, 3, 17, 24, 18, 52, 38, 30, 26, 46, 39, 41, 35].into_iter().collect::<Vec<usize>>();

    for (i, data) in data.iter().enumerate() {
        println!("Inserting {data} ({i})");
        fheap.insert(*data);
    }

    output_fibheap(&fheap, timestamp);

    let heap_size = fheap.size();
    for i in 0..heap_size {
        println!("ExtractMin: {:?}", fheap.extract_min());
    }
}

fn output_fibheap(fheap: &FibonacciHeap, timestamp: String) {
    println!("{}", fheap.to_graphviz_graph().print(&mut PrinterContext::default()));
}