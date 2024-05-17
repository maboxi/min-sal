pub mod fibonacci_heap;
use crate::fibonacci_heap::FibonacciHeap;

use std::{fs::{self, File}, io::{Error, Write}};
use graphviz_rust::{cmd::Format, exec, printer::PrinterContext};


fn main() {
    let mut fheap = FibonacciHeap::new();
    let mut fheap_printer = FibHeapPrinter::new();

    //let data = [23, 7, 3, 17, 24, 18, 52, 38, 30, 26, 46, 39, 41, 35].into_iter().collect::<Vec<usize>>();
    let mut data_vec = [3, 6, 9, 12, 15, 18, 21, 24, 27, 30].into_iter().map(|x| (x,0)).collect::<Vec<(usize,usize)>>();

    for i in 0..data_vec.len() {
        data_vec[i].1 = fheap.insert(data_vec[i].0);
    }

    fheap_printer.print(&mut fheap).unwrap();
    
    // 1x ExtractMin to 'heapify' to heap
    println!("ExtractMin: {:?}", fheap.extract_min());
    fheap_printer.print(&mut fheap).unwrap();

    println!("DecreaseKey 24 -> 8");
    fheap.decrease_key(7, 8);
    fheap_printer.print(&mut fheap).unwrap();
    
    println!("DecreaseKey 21 -> 7");
    fheap.decrease_key(6, 7);
    fheap_printer.print(&mut fheap).unwrap();


    /*let heap_size = fheap.size();
    for _i in 0..heap_size {
        println!("ExtractMin: {:?}", fheap.extract_min());
        fheap_printer.print(&mut fheap).unwrap();
    }*/
}


struct FibHeapPrinter {
    timestamp: String,
    counter: usize,
    output_folder: String,
}

impl FibHeapPrinter {
    fn new() -> Self { 
        //let output_folder_path = format!("./output/{}", timestamp);
        let printer = FibHeapPrinter {
            timestamp: format!("{}", chrono::prelude::Utc::now().format("%Y%m%d-%H%M")),
            counter: 0,
            output_folder: format!("./output"),
        };

        println!("Created FibHeap Printer with timestamp: {}", printer.timestamp);
        match fs::remove_dir_all(&printer.output_folder) {
            Ok(()) => (),
            Err(_) => ()
        };

        printer
    }

    fn print(&mut self, fheap: &mut FibonacciHeap) -> Result<(), Error> {
        fheap.update_depths();
        let graph = fheap.to_graphviz_graph();
        //println!("Graph: {}", graph.print(&mut PrinterContext::default()));
        let format = Format::Svg;
        let graph_svg = exec(graph, &mut PrinterContext::default(), vec![format.into()])?;

        let filename = format!("{}/output-{}.svg", self.output_folder, self.counter);

        match fs::create_dir_all(&self.output_folder) {
            Ok(_) => (),
            Err(err) => {
                println!("Error creating output folder: {err:?}");
            }
        }

        self.counter += 1;
        
        println!("Writing svg to file {filename}");

        let mut file = File::create(filename)?;
        file.write_all(graph_svg.as_slice())?;

        Ok(())
    }
}