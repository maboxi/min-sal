use graphviz_rust::dot_generator::*;
use graphviz_rust::dot_structures::*;
use graphviz_rust::{
    attributes::*,
    cmd::{CommandArg, Format},
    exec, exec_dot, parse,
    printer::{DotPrinter, PrinterContext},
};


#[derive(Clone, Copy)]
struct FibonacciHeapElement {
    value: usize,
    degree: usize,
    marked: bool,
    parent: Option<usize>,
    child: Option<usize>,
    left: usize,
    right: usize,
}

pub struct FibonacciHeap {
    elements: Vec<FibonacciHeapElement>,
    h_min: Option<usize>,
}

impl FibonacciHeap {
    pub fn new() -> Self {
        FibonacciHeap { elements: vec![], h_min: None, }
    }

    pub fn insert(&mut self, data: usize) {
        let mut new_elem = FibonacciHeapElement {
            value: data,
            degree: 0,
            marked: false,
            parent: None,
            child: None,
            left: 0,
            right: 0,
        };

        let new_i = self.elements.len();

        match self.h_min {
            None => {
                if self.elements.len() > 0 {
                    panic!("H_min cannot be None if there are elements in the heap!");
                }
                // neue wurzelliste
                new_elem.left = 0;
                new_elem.right= 0;
                self.h_min = Some(0);
            },
            Some(head) => {
                new_elem.right = self.elements[head].right;
                new_elem.left = head;
                self.elements[head].right = new_i;
                if self.elements[head].value > new_elem.value {
                    self.h_min = Some(new_i);
                }
            }
        }
        
        self.elements.push(new_elem);

    }

    pub fn extract_min(&mut self) -> Option<usize> {
        if let Some(z) = self.h_min {
            // füge alle x kinder von z: füge x in wurzelliste & setze x.parent = None

            if let Some(x_first) = self.elements[z].child {
                // setze x_parent auf None
                let x_start = x_first;
                let mut x_cur = x_start;
                loop {
                    self.elements[x_cur].parent = None;

                    x_cur = self.elements[x_cur].right;
                    if x_cur == x_start { break; }
                }
 
                // alle in wurzelliste =^ doppelt verk. liste [x] links von z einhängen
                let x_first_left = self.elements[x_first].left;
                let z_left = self.elements[z].left;

                self.elements[x_first].left = self.elements[z].left;
                self.elements[z_left].right = x_first;

                self.elements[x_first_left].right = z;
                self.elements[z].left = x_first_left;
            }

            let ret_val = self.elements[z].value;

            // remove z from root list & heap: swap z with last element in elements list, then update references

            let z_left = self.elements[z].left;
            let z_right = self.elements[z].right;
            self.elements[z_left].right = z_right;
            self.elements[z_right].left = z_left;

            let z_elem = self.elements[z];
            let z_new = self.elements.len() - 1;
            self.elements.swap(z, z_new);
            self.update_references(z, z_new);

            self.elements.truncate(self.elements.len() - 1);

            if z_elem.right == z {
                // heap is empty after removal
                self.h_min = None;
            } else {
                let z_right = // check if z_right was the swapped one
                    if z_elem.right == z_new {
                        z
                    } else {
                        z_elem.right
                    };

                self.h_min = Some(z_right);
                self.consolidate();
            }


            return Some(ret_val);
        } else {
            return None;
        }
    }

    fn update_references(&mut self, z: usize, z_old: usize) {
        // update all references to u

        // check if left element was the swapped one
        if self.elements[z].left == z {

        }

        let z_right = self.elements[z].right;
        let z_left = self.elements[z].left;
        self.elements[z_right].left = z;
        self.elements[z_left].right = z;

        if let Some(child) = self.elements[z].child {
            // update all parent references in the list of the child
            let child_start = child;
            let mut child_cur = child_start;

            loop {
                self.elements[child_cur].parent = Some(z);

                child_cur = self.elements[child_cur].right;
                if child_cur == child_start { break; }
            }
        }

        if let Some(parent) = self.elements[z].parent {
            if let Some(cur_child) = self.elements[parent].child {
                if cur_child == z_old {
                    self.elements[parent].child = Some(z);
                }
            }
        }
    }

    fn consolidate(&mut self) {
        let mut a: Vec<Option<usize>> = vec![None; self.size()];

        if let Some(h_min) = self.h_min {
            let w_end = self.elements[h_min].left;
            let mut w_cur = h_min;

            loop {
                let mut x = w_cur;
                let mut d = self.elements[w_cur].degree;

                while let Some(mut y) = a[d] {
                    if self.elements[x].value > self.elements[y].value {
                        // exchange x and y
                        let x_tmp = x;
                        x = y;
                        y = x_tmp;
                    }
                    self.link(y,x);
                    a[d] = None;
                    d = d + 1;
                }

                a[d] = Some(x);
                
                if w_cur == w_end { break; }
                w_cur = self.elements[w_cur].right;
            }
        
            self.h_min = None;

            for a_i in a {
                if let Some(a_i) = a_i {
                    if let Some(h_min) = self.h_min {
                        // root ll already exists
                        let h_min_left = self.elements[h_min].left;
                        let h_min_right = self.elements[h_min].right;

                        self.elements[a_i].left = h_min_left;
                        self.elements[h_min_left].right = a_i;

                        self.elements[a_i].right = h_min_right;
                        self.elements[h_min_right].left = a_i;

                        if self.elements[a_i].value < self.elements[h_min].value {
                            self.h_min = Some(a_i);
                        }
                    } else {
                        // root ll is empty
                        self.elements[a_i].right = a_i;
                        self.elements[a_i].left = a_i;
                        self.elements[a_i].parent = None;
                        self.h_min = Some(a_i);
                    }
                }
            }
        }
    }

    fn link(&mut self, y: usize, x: usize) {
        // remove y from root list
        let y_right = self.elements[y].right;
        let y_left = self.elements[y].left;
        self.elements[y_left].right = y_right;
        self.elements[y_right].left = y_left;

        // make y a child of x
        if let Some(x_child) = self.elements[x].child {
            let x_child_left = self.elements[x_child].left;

            self.elements[y].left = x_child_left;
            self.elements[x_child_left].right = y;

            self.elements[y].right = x_child;
            self.elements[x_child].left = y;
        } else {
            // create new ll with only y in it
            self.elements[y].right = y;
            self.elements[y].left = y;

            // y becomes child list reference of x
            self.elements[x].child = Some(y);
        }
        self.elements[y].parent = Some(x);            

        // increase x.degree
        self.elements[x].degree += 1;
        // remove y.mark
        self.elements[y].marked = false;
    }

    pub fn to_graphviz_graph(&self) -> Graph {
        // TODO: graphviz stuff
        let mut graph = Graph::DiGraph {
            id: id!("fibheap"),
            strict: true,
            stmts: vec![]
        };

        let mut nodes: Vec<Node> = vec![];
        let mut edges: Vec<Edge> = vec![];

        for elem in &self.elements {
            nodes.push(node!(
                elem.value;
                attr!("deg", elem.degree)
            ));

            edges.push(edge!(
                node_id!(elem.value) => node_id!(self.elements[elem.right].value)
            ));
            edges.push(edge!(
                node_id!(elem.value) => node_id!(self.elements[elem.left].value)
            ));

            if let Some(parent) = elem.parent {
                edges.push(edge!(
                    node_id!(elem.value) => node_id!(self.elements[parent].value)
                ));
            }

            if let Some(child) = elem.child {
                edges.push(edge!(
                    node_id!(elem.value) => node_id!(self.elements[child].value)
                ));
            }

        }
   
        for node in nodes {
            graph.add_stmt(Stmt::Node(node));
        }

        for edge in edges {
            graph.add_stmt(Stmt::Edge(edge));
        }

        return graph;
    }

    pub fn size(&self) -> usize { self.elements.len() }
}