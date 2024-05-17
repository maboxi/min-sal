use graphviz_rust::dot_generator::*;
use graphviz_rust::dot_structures::*;

#[derive(Clone, Copy)]
struct FibonacciHeapElement {
    value: usize,
    degree: usize,
    marked: bool,
    parent: Option<usize>,
    child: Option<usize>,
    left: usize,
    right: usize,
    depth: usize,
}

pub struct FibonacciHeap {
    elements: Vec<FibonacciHeapElement>,
    h_min: Option<usize>,
    max_depth: usize,
}

impl FibonacciHeap {
    pub fn new() -> Self {
        FibonacciHeap { elements: vec![], h_min: None, max_depth: 0, }
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
            depth: 0,
        };

        let new_i = self.elements.len();

        match self.h_min {
            None => {
                if self.elements.len() > 0 {
                    panic!("H_min cannot be None if there are elements in the heap!");
                }
                // neue wurzelliste
                new_elem.left = new_i;
                new_elem.right= new_i;
                self.h_min = Some(new_i);
            },
            Some(head) => {
                let head_left = self.elements[head].left;

                new_elem.left = head_left;
                self.elements[head_left].right = new_i;

                new_elem.right = head;
                self.elements[head].left = new_i;

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
            /*println!("Extracting {} [{}]: Swapping to back with {} [{}]",
                z_elem.value, z,
                self.elements[z_new].value, z_new);*/
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

    fn consolidate(&mut self) {
        let mut a: Vec<Option<usize>> = vec![None; self.size()];

        //println!("consolidating heap...");

        if let Some(h_min) = self.h_min {
            let w_end = self.elements[h_min].left;
            let mut w_cur = h_min;

            loop {
                let mut x: usize = w_cur;
                let w_next = self.elements[x].right;
                let mut d = self.elements[w_cur].degree;
                //println!("\tconsolidating {x} (value {}) with deg {d}, {}", self.elements[x].value, self.elements[x].right);

                while let Some(mut y) = a[d] {
                    //println!("\t\tdegree {d} conflict: {x} vs {y}");

                    if self.elements[x].value > self.elements[y].value {
                        // exchange x and y
                        //println!("\t\t\texchange {x}({}) and {y}({})", self.elements[x].value, self.elements[y].value);
                        let x_tmp = x;
                        x = y;
                        y = x_tmp;
                    }

                    self.link(y,x);
                    a[d] = None;
                    d = d + 1;
                }

                //println!("\t\tnow: a[{d}] = {x}");
                a[d] = Some(x);
                
                if w_cur == w_end { break; }
                w_cur = w_next;
                //println!("\tnext: {w_cur}");
            }
        
            self.h_min = None;

            /*for (i, a_i) in a.iter().enumerate() {
                println!("a[{i}]: {a_i:?}");
            }*/

            // create root ll
            for a_i in a {
                if let Some(a_i) = a_i {
                    if let Some(h_min) = self.h_min {
                        // root ll already exists
                        let h_min_left = self.elements[h_min].left;

                        self.elements[a_i].left = h_min_left;
                        self.elements[h_min_left].right = a_i;

                        self.elements[a_i].right = h_min;
                        self.elements[h_min].left = a_i;

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
        //println!("\t\t\tlinking {y}({}) to {x}({})", self.elements[y].value, self.elements[x].value);

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

    fn update_references(&mut self, z: usize, z_old: usize) {
        // update all references to u
        /*println!("\tUpdating references of {} from {} to {}: l: {}, r: {}, c: {:?}, p: {:?}",
            self.elements[z].value, z_old, z,
            self.elements[z].left,
            self.elements[z].right,
            self.elements[z].child,
            self.elements[z].parent);*/

        if z == z_old {
            //println!("\t\tNo position change -> do nothing!");
            return;
        }

        let mut z_right = self.elements[z].right;
        if z_right == z_old { z_right = z; }
        let mut z_left = self.elements[z].left;
        if z_left == z_old { z_left = z; }
        self.elements[z_right].left = z;
        self.elements[z_left].right = z;

        if let Some(child) = self.elements[z].child {
            // update all parent references in the list of the child
            //println!("\t\tUpdating references for children in ll, starting at {}", child);
            let child_start = child;
            let mut child_cur = child_start;

            loop {
                //println!("\t\t\tChild {}: prev p: {:?}", child_cur, self.elements[child_cur].parent);
                self.elements[child_cur].parent = Some(z);
                //println!("\t\t\tChild {}: new p: {:?}", child_cur, self.elements[child_cur].parent);

                child_cur = self.elements[child_cur].right;
                //println!("\t\t\tNext child: {}", child_cur);
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
        /*println!("\tAfter update of {} from {} to {}: l: {}, r: {}, c: {:?}, p: {:?}",
            self.elements[z].value, z_old, z,
            self.elements[z].left,
            self.elements[z].right,
            self.elements[z].child,
            self.elements[z].parent);*/
    }

    pub fn update_depths(&mut self) {
        if let Some(h_min) = self.h_min {
            // update depths
            let mut lls: Vec<(usize, usize)> = vec![(h_min, 0)];
            self.max_depth = 0;

            while let Some((ll_start, cur_depth)) = lls.pop() {
                let mut ll_cur = ll_start;

                if cur_depth > self.max_depth { self.max_depth = cur_depth; }

                loop {
                    if let Some(child) = self.elements[ll_cur].child {
                        lls.push((child, cur_depth + 1));
                    } 

                    self.elements[ll_cur].depth = cur_depth;

                    ll_cur = self.elements[ll_cur].right;
                    if ll_cur == ll_start { break; }
                }
            }
        }
   }
    
    pub fn to_graphviz_graph(&self) -> Graph {
        let mut graph = Graph::DiGraph {
            id: id!("fibheap"),
            strict: true,
            stmts: vec![]
        };

        graph.add_stmt(stmt!(attr!("rankdir", "BT")));
        graph.add_stmt(stmt!(attr!("nodesep", "0.7")));
        graph.add_stmt(stmt!(attr!("ranksep", "0.7")));

        let mut subgraph = Subgraph {
            id: id!("fibheap_sg"),
            stmts: vec![],
        };

        let mut edges: Vec<Edge> = vec![];
        // add dummy root node
        graph.add_stmt(stmt!(
            Subgraph {
                id: id!("sg_root"),
                stmts: vec![stmt!(
                Node {
                    id: node_id!("root"),
                    attributes: vec![]
                })]
            }
        ));

        if let Some(h_min) = self.h_min {
            for elem in &self.elements {
                edges.push(edge!(
                    node_id!(elem.value) => node_id!(self.elements[elem.right].value);
                    attr!("constraint", false)//,attr!("style", "invis")
                ));
                edges.push(edge!(
                    node_id!(elem.value) => node_id!(self.elements[elem.left].value);
                    attr!("constraint", false)//,attr!("style", "invis")
                ));
                if let Some(parent) = elem.parent {
                    edges.push(edge!(
                        node_id!(elem.value) => node_id!(self.elements[parent].value)//;attr!("style", "invis")
                    ));
                }

                if let Some(child) = elem.child {
                    edges.push(edge!(
                        node_id!(elem.value) => node_id!(self.elements[child].value);
                        attr!("constraint", false)
                    ));
                }
            }
           
            for cur_depth in 0..(self.max_depth + 1) {
                let mut depth_subgraph = Subgraph{
                    id: id!(format!("sg_ll_{cur_depth}")),
                    stmts: vec![]
                };
                depth_subgraph.stmts.push(stmt!(attr!("rank", "same"))); 

                for (i, elem) in self.elements.iter().enumerate().filter(|(_,elem)| elem.depth == cur_depth) {
                    let label = format!("\"{} [{}]\ndeg: {}\n\"",
                                                elem.value, i, elem.degree);
                    if h_min == i {
                        depth_subgraph.stmts.push(stmt!(
                            Node { 
                                id: node_id!(elem.value),
                                attributes: vec![
                                    Attribute(id!("label"), id!(label)),
                                    Attribute(id!("style"), id!("filled")),
                                    Attribute(id!("fillcolor"), id!("red")),
                                ]
                            }
                        ));
                        edges.push(edge!(node_id!(elem.value) => node_id!("root")));
                    } else {
                        depth_subgraph.stmts.push(stmt!(
                            Node { 
                                id: node_id!(elem.value),
                                attributes: vec![
                                    Attribute(id!("label"), id!(label)),
                                    Attribute(id!("style"), id!("filled")),
                                    Attribute(id!("fillcolor"), if elem.marked {id!("lightgrey")} else {id!("white")}),
                                ]
                            }
                        ));
                        if cur_depth == 0 {edges.push(edge!(node_id!(elem.value) => node_id!("root"); attr!("style", "invis")));}
                    }
                }

                subgraph.stmts.push(stmt!(depth_subgraph));

            }

            for edge in edges {
                subgraph.stmts.push(stmt!(edge));
            }
        }

        graph.add_stmt(stmt!(subgraph));

        graph
    }

    pub fn size(&self) -> usize { self.elements.len() }
}