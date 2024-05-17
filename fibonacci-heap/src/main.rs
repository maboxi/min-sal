pub mod fibonacci_heap;
use std::io::Error;

use crate::fibonacci_heap::FibonacciHeap;


fn main() {
    println!("Fibonacci Heap main");
    fheap_test_helper();
}

#[derive(Debug, Clone)]
enum HeapOperation {
    Create(String),
    Insert(String, usize),
    ExtractMin(String),
    DecreaseKey(String, usize, usize),
}
struct HeapHelper {
    heaps: Vec<FibonacciHeap>,
    operations: Vec<HeapOperation>,
    output_dir: String,
}

impl HeapHelper {
    pub fn new(output_dir: String) -> Self {
        HeapHelper {
            heaps: Vec::with_capacity(1),
            operations: vec![],
            output_dir: output_dir,
        }
    }

    pub fn execute(&mut self, op: HeapOperation) -> Result<Option<usize>, Error> {
        self.operations.push(op.clone());

        match op {
            HeapOperation::Create(name) => {
                self.heaps.push(FibonacciHeap::new(self.output_dir.clone(), name));
                return Ok(None);
            },
            HeapOperation::Insert(name, value) => {
                let mut heap_i: Option<usize> = None;
                for (i, fh) in self.heaps.iter().enumerate() { if *fh.get_name() == name { heap_i = Some(i); }}
                let heap_i = heap_i.unwrap_or_else(|| panic!("HeapHelper: Heap {} not found!", name));

                self.heaps[heap_i].insert(value);
                self.heaps[heap_i].print()?;

                return Ok(None);
            },
            HeapOperation::DecreaseKey(name, value, new_key) => {
                let mut heap_i: Option<usize> = None;
                for (i, fh) in self.heaps.iter().enumerate() { if *fh.get_name() == name { heap_i = Some(i); }}
                let heap_i = heap_i.unwrap_or_else(|| panic!("HeapHelper: Heap {} not found!", name));

                let lookup_index = self.heaps[heap_i].get_lookup_index(value);
                self.heaps[heap_i].decrease_key(lookup_index, new_key);

                self.heaps[heap_i].print()?;

                return Ok(None);
            },
            HeapOperation::ExtractMin(name) => {
                let mut heap_i: Option<usize> = None;
                for (i, fh) in self.heaps.iter().enumerate() { if *fh.get_name() == *name { heap_i = Some(i); }}
                let heap_i = heap_i.unwrap_or_else(|| panic!("HeapHelper: Heap {} not found!", name));

                let ret = self.heaps[heap_i].extract_min();

                self.heaps[heap_i].print()?;

                return Ok(ret);
            }
        }

    }
}

fn fheap_test_helper() {
    let mut helper = HeapHelper::new("./output/helper-test".to_string());

    const HEAP_A: &str = "a";

    let ops = vec![
        HeapOperation::Create(String::from(HEAP_A)),

        HeapOperation::Insert(String::from(HEAP_A), 3),
        HeapOperation::Insert(String::from(HEAP_A), 6),
        HeapOperation::Insert(String::from(HEAP_A), 9),
        HeapOperation::Insert(String::from(HEAP_A), 12),
        HeapOperation::Insert(String::from(HEAP_A), 15),
        HeapOperation::Insert(String::from(HEAP_A), 18),
        HeapOperation::Insert(String::from(HEAP_A), 21),
        HeapOperation::Insert(String::from(HEAP_A), 24),
        HeapOperation::Insert(String::from(HEAP_A), 27),
        HeapOperation::Insert(String::from(HEAP_A), 30),

        HeapOperation::ExtractMin(String::from(HEAP_A)),

        HeapOperation::DecreaseKey(String::from(HEAP_A), 24, 8),
        HeapOperation::DecreaseKey(String::from(HEAP_A), 21, 7),
    ];

    for op in ops.iter() {
        match helper.execute(op.clone()) {
            Ok(None) => (),
            Ok(Some(_ret)) => (),
            Err(err) => eprintln!("HeapHelper OP exec error: {err}")
        }
    }
}

#[test]
fn union_test() -> Result<(), Error> {
    fheap_test_union()
}

#[test]
fn fheap_test_union() -> Result<(), Error> {
    let mut fheap1 = FibonacciHeap::new( 
        "./output/test/union".to_string(),
        "a".to_string());

    fheap1.insert(1);
    fheap1.insert(2);
    fheap1.insert(3);
    fheap1.insert(4);
    fheap1.insert(5);
    fheap1.insert(6);
    fheap1.print()?;
    _ = fheap1.extract_min();
    fheap1.print()?;

    let mut fheap2 = FibonacciHeap::new( 
        "./output/test/union".to_string(),
        "b".to_string());

    fheap2.insert(11);
    fheap2.insert(12);
    fheap2.insert(13);
    fheap2.insert(14);
    fheap2.insert(15);
    fheap2.insert(16);
    fheap2.print()?;
    _ = fheap2.extract_min();
    fheap2.print()?;

    let (mut fheap3,_) = FibonacciHeap::union(
        fheap1, 
        fheap2, 
        "./output/test/union".to_string()
    );

    fheap3.print()?;
    _ = fheap3.extract_min();
    fheap3.print()?;
    fheap3.decrease_key(9, 9);
    fheap3.print()?;

    Ok(())
}

#[test]
fn fheap_test_extractmin() -> Result<(), Error> {
    let mut fheap = FibonacciHeap::new(
        "./output/test/extract_min".to_string(),
        "test_extractmin".to_string());

    let data_vec = [23, 7, 3, 17, 24, 18, 52, 38, 30, 26, 46, 39, 41, 35].into_iter().collect::<Vec<usize>>();

    for i in 0..data_vec.len() {
        fheap.insert(data_vec[i]);
    }

    fheap.print()?;
    
    let heap_size = fheap.size();
    for _i in 0..heap_size {
        println!("ExtractMin: {:?}", fheap.extract_min());
        fheap.print()?;
    }

    Ok(())
}

#[test]
fn fheap_test_decreasekey() -> Result<(), Error> {
    let mut fheap = FibonacciHeap::new(
        "./output/test/decrease_key".to_string(),
        "test_decreasekey".to_string());

   let mut data_vec = [3, 6, 9, 12, 15, 18, 21, 24, 27, 30].into_iter().map(|x| (x,0)).collect::<Vec<(usize,usize)>>();

    for i in 0..data_vec.len() {
        data_vec[i].1 = fheap.insert(data_vec[i].0);
    }

    fheap.print()?;
    
    // 1x ExtractMin to 'heapify' to heap
    println!("ExtractMin: {:?}", fheap.extract_min());
    fheap.print()?;

    println!("DecreaseKey 24 -> 8");
    fheap.decrease_key(7, 8);
    fheap.print()?;
    
    println!("DecreaseKey 21 -> 7");
    fheap.decrease_key(6, 7);
    fheap.print()?;

    Ok(())
}