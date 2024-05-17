pub mod fibonacci_heap;
use crate::fibonacci_heap::FibonacciHeap;

use std::{fs::{self, File}, io::{Error, Write}};
use graphviz_rust::{cmd::Format, exec, printer::{DotPrinter, PrinterContext}};



fn main() {
    let mut fheap = FibonacciHeap::new();
    let timestamp = format!("{}", chrono::prelude::Utc::now().format("%Y%m%d-%H%M"));
    let mut output_counter: usize = 0;
    println!("Timestamp: {timestamp}");

    let data = [23, 7, 3, 17, 24, 18, 52, 38, 30, 26, 46, 39, 41, 35].into_iter().collect::<Vec<usize>>();

    for (i, data) in data.iter().enumerate() {
        println!("Inserting {data} ({i})");
        fheap.insert(*data);
    }

    output_fibheap(&mut fheap, &timestamp, &mut output_counter).unwrap();
    println!("ExtractMin: {:?}", fheap.extract_min());
    output_fibheap(&mut fheap, &timestamp, &mut output_counter).unwrap();

    /*let heap_size = fheap.size();
    for i in 0..heap_size {
        println!("ExtractMin: {:?}", fheap.extract_min());

        output_fibheap(&fheap, &timestamp, &mut output_counter).unwrap();

    }*/
}

fn output_fibheap(fheap: &mut FibonacciHeap, timestamp: &String, counter: &mut usize) -> Result<(), Error> {
    let graph = fheap.to_graphviz_graph();
    //println!("Graph: {}", graph.print(&mut PrinterContext::default()));
    let format = Format::Svg;
    let graph_svg = exec(graph, &mut PrinterContext::default(), vec![format.into()])?;

    //let output_folder_path = format!("./output/{}", timestamp);
    let output_folder_path = format!("./output");
    let filename = format!("{}/output-{}.svg", output_folder_path, counter);

    match fs::create_dir_all(output_folder_path) {
        Ok(_) => (),
        Err(err) => {
            println!("Error creating output folder: {err:?}");
        }
    }

    *counter += 1;
    
    println!("Writing svg to file {filename}");

    let mut file = File::create(filename)?;
    file.write_all(graph_svg.as_slice())?;

    Ok(())
}