use std::fmt::Debug;
#[derive(Debug)]
pub struct Heap<K>
{
    heap: Vec<usize>,
    memory: Vec<(K, usize, bool)>,
}

impl<K: PartialOrd + Debug + Copy> Heap<K> {
    pub fn from(key_value_pairs: Vec<(K, usize)>) -> Self {
        let mut instance = Heap {
            heap: Vec::with_capacity(key_value_pairs.len()),
            memory: Vec::with_capacity(key_value_pairs.len())
        };
        instance.build_heap(key_value_pairs);
        instance
    }

    fn build_heap(&mut self, kvpairs: Vec<(K, usize)>) {

        println!("KV-Pairs:");
        for kvp in &kvpairs { println!("    {:?}", kvp); }

        for (i, (k,v)) in kvpairs.iter().enumerate() {
            self.heap.push(*v);
            self.memory.push((*k, i, true));
        }

        let n = self.heap.len();
        println!("Heap: {:?}", self.heap);
        println!("Memory:");
        for mem_item in &self.memory { println!("    {:?}", mem_item); }
 
        let mut i = (n-2) / 2;
        loop {
            self.heapify(i);
            if i == 0 { break; }
            i -= 1;
        }
        println!("Heap building complete!");
    }

    fn heapify(&mut self, root: usize) {
        if self.heap.len() == 0 { return; }

        let mut smallest: usize;
        let left = root*2 + 1;
        let right = root*2 + 2;
        let l = self.heap.len() - 1;

        if left <= l && self.get_key_from_heap(left) < self.get_key_from_heap(root) { smallest = left; }
        else { smallest = root; }

        if right <= l && self.get_key_from_heap(right) < self.get_key_from_heap(smallest) { smallest = right; }

        if smallest != root {
            self.swap_heapobjects(root, smallest);
            self.heapify(smallest);
        }
    }
    
    pub fn extract_min(&mut self) -> Option<(K, usize)>{
        if self.heap.len() == 0 { return None }

        let n = self.heap.len() - 1;
        self.swap_heapobjects(0, n);
        self.memory[self.heap[n]].2 = false;
        let min = self.heap.pop();
        if self.heap.len() > 0 { self.heapify(0); }
        min.map(|x| (self.memory[x].0, x))
    }

    pub fn decrease_key(&mut self, value: usize, new_key: K) {

        if !self.memory[value].2 {
            eprintln!("Decrease key: {value} is not in hash anymore!");
            return;
        }

        let cur_key = &self.memory[value].0;

        if cur_key <= &new_key {
            eprintln!("Decreasing key of {} from {:?} to {:?} makes no sense!", value, cur_key, new_key);
            return;
        }

        println!("Decreasing key of {} from {:?} to {:?}...", value, cur_key, new_key);
        self.memory[value].0 = new_key;

        let mut curpos = self.memory[value].1;
        let mut parent: usize;


        loop {
            if curpos == 0 { break; }

            parent = curpos / 2;

            if self.get_key_from_heap(curpos) < self.get_key_from_heap(parent) { self. swap_heapobjects(curpos, parent); }

            curpos = parent;
        }
    }

    fn swap_heapobjects(&mut self, a: usize, b: usize) {
        self.memory[self.heap[a]].1 = b;
        self.memory[self.heap[b]].1 = a;
        self.heap.swap(a, b);
    }

    pub fn is_empty(&self) -> bool {
        self.heap.len() == 0
    }

    pub fn get_key(&self, value: usize) -> K {
        self.memory[value].0
    }

    fn get_key_from_heap(&self, heap_index: usize) -> &K {
        &self.memory[self.heap[heap_index]].0
    }
}