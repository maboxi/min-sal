pub struct Heap<T> {
    heap: Vec<T> ,
}

impl<T: PartialOrd> Heap<T> {
    pub fn from(arr: Vec<T>) -> Self {
        let mut instance = Heap {heap: arr};
        instance.build_heap();
        instance
    }

    pub fn build_heap(&mut self) {
        let n = self.heap.len();
        let mut i = (n-2) / 2;
        loop {
            self.heapify(i);
            if i == 0 { break; }
            i -= 1;
        }
    }

    fn heapify(&mut self, root: usize) {
        let mut smallest: usize;
        let left = root*2 + 1;
        let right = root*2 + 2;
        let l = self.heap.len() - 1;

        if left <= l && self.heap[left] < self.heap[root] { smallest = left; }
        else { smallest = root; }

        if right <= l && self.heap[right] < self.heap[smallest] { smallest = right; }

        if smallest != root {
            self.heap.swap(root, smallest);
            self.heapify(smallest);
        }
    }
    
    pub fn extract_max(&mut self) -> Option<T>{
        let n = self.heap.len() - 1;
        self.heap.swap(0, n);
        let min = self.heap.pop();
        self.heapify(0);
        min
    }

    pub fn decrease_key(&mut self, search: F)
    where
        F: Fn(&T) -> bool
    {
    }

    pub fn is_empty(&self) -> bool {
        self.heap.len() == 0
    }
}