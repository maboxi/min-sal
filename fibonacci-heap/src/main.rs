pub mod fibonacci_heap;
use crate::fibonacci_heap::FibonacciHeap;


fn main() {
    println!("Fibonacci Heap main");
    fheap_test_union();
}

#[test]
fn union_test() {
    fheap_test_union();
}

fn fheap_test_union() {
    let mut fheap1 = FibonacciHeap::new( 
        "./output/test/union".to_string(),
        "a".to_string());

    fheap1.insert(1);
    fheap1.insert(2);
    fheap1.insert(3);
    fheap1.insert(4);
    fheap1.insert(5);
    fheap1.insert(6);
    fheap1.print();
    _ = fheap1.extract_min();
    fheap1.print();

    let mut fheap2 = FibonacciHeap::new( 
        "./output/test/union".to_string(),
        "b".to_string());

    fheap2.insert(11);
    fheap2.insert(12);
    fheap2.insert(13);
    fheap2.insert(14);
    fheap2.insert(15);
    fheap2.insert(16);
    fheap2.print();
    _ = fheap2.extract_min();
    fheap2.print();

    let (mut fheap3,_) = FibonacciHeap::union(
        fheap1, 
        fheap2, 
        "./output/test/union".to_string()
    );

    fheap3.print();
    _ = fheap3.extract_min();
    fheap3.print();
    fheap3.decrease_key(9, 9);
    fheap3.print();
}

#[test]
fn fheap_test_extractmin() {
    let mut fheap = FibonacciHeap::new(
        "./output/test/extract_min".to_string(),
        "test_extractmin".to_string());

    let data_vec = [23, 7, 3, 17, 24, 18, 52, 38, 30, 26, 46, 39, 41, 35].into_iter().collect::<Vec<usize>>();

    for i in 0..data_vec.len() {
        fheap.insert(data_vec[i]);
    }

    fheap.print();
    
    let heap_size = fheap.size();
    for _i in 0..heap_size {
        println!("ExtractMin: {:?}", fheap.extract_min());
        fheap.print();
    }
}

#[test]
fn fheap_test_decreasekey() {
    let mut fheap = FibonacciHeap::new(
        "./output/test/decrease_key".to_string(),
        "test_decreasekey".to_string());

   let mut data_vec = [3, 6, 9, 12, 15, 18, 21, 24, 27, 30].into_iter().map(|x| (x,0)).collect::<Vec<(usize,usize)>>();

    for i in 0..data_vec.len() {
        data_vec[i].1 = fheap.insert(data_vec[i].0);
    }

    fheap.print();
    
    // 1x ExtractMin to 'heapify' to heap
    println!("ExtractMin: {:?}", fheap.extract_min());
    fheap.print();

    println!("DecreaseKey 24 -> 8");
    fheap.decrease_key(7, 8);
    fheap.print();
    
    println!("DecreaseKey 21 -> 7");
    fheap.decrease_key(6, 7);
    fheap.print();
}