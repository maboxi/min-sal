use std::{fs::{self, File}, io::{Error, Write}};
use graphviz_rust::{cmd::Format, exec, printer::PrinterContext};

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
    lookup_pos: usize,
}

pub struct FibonacciHeap {
    elements: Vec<FibonacciHeapElement>,
    lookup_table: Vec<(usize,bool)>,
    lookup_table_spaces: Vec<usize>,
    h_min: Option<usize>,

    max_depth: usize,
    operations: Vec<String>,
    name: String,
    printer: FibHeapPrinter,
}

impl FibonacciHeap {
    pub fn new(folder: String, name: String) -> Self {
        let printer = FibHeapPrinter::new(&folder, &name);
        FibonacciHeap {
            elements: vec![],
            lookup_table: vec![],
            lookup_table_spaces: vec![],
            h_min: None,

            max_depth: 0,
            operations: vec![format!("Heap Creation")],
            name: name,
            printer: printer,
        }
    }

    pub fn union(mut fheap1: FibonacciHeap, fheap2: FibonacciHeap, folder: String) -> (Self, usize) {
        let name = format!("{}_{}", fheap1.name, fheap2.name);
        let printer = FibHeapPrinter::new(&folder,&name);

        let elem_offset = fheap1.elements.len();
        let elem_length = fheap1.elements.len() + fheap2.elements.len();
        let lut_offset = fheap1.lookup_table.len();
        let lut_length = fheap1.lookup_table.len() + fheap2.lookup_table.len();

        let mut union_heap = FibonacciHeap {
            elements: Vec::with_capacity(elem_length),
            lookup_table: Vec::with_capacity(lut_length),
            lookup_table_spaces: vec![],
            h_min: None,

            max_depth: 0,
            operations: vec![format!("Heap Creation")],
            name: name,
            printer: printer,
        };

        union_heap.elements.append(&mut fheap1.elements);
        union_heap.elements.append(
            &mut fheap2.elements.into_iter().map(|mut elem| {
                elem.parent = elem.parent.map(|p| p + elem_offset);
                elem.child = elem.child.map(|c| c + elem_offset);
                elem.left += elem_offset;
                elem.right += elem_offset;
                elem.lookup_pos += lut_offset;
                elem
            }).collect()
        );

        // connect H2.h_min to H1.h_min and update h_min to new min
        if let Some(h1_min) = fheap1.h_min {
            if let Some(h2_min) = fheap2.h_min {
                let h2_min = h2_min + elem_offset;

                let h1_min_left = union_heap.elements[h1_min].left;
                let h2_min_left = union_heap.elements[h2_min].left;
                
                union_heap.elements[h1_min_left].right = h2_min;
                union_heap.elements[h2_min].left = h1_min_left;

                union_heap.elements[h2_min_left].right = h1_min;
                union_heap.elements[h1_min].left = h2_min_left;

                if union_heap.elements[h2_min].value < union_heap.elements[h1_min].value {
                    union_heap.h_min = Some(h2_min);
                } else {
                    union_heap.h_min = Some(h1_min);
                }
            } else {
                union_heap.h_min = Some(h1_min);
            }
        } else {
            union_heap.h_min = fheap2.h_min;
        }

        union_heap.lookup_table.append(&mut fheap1.lookup_table);
        union_heap.lookup_table.append(
            &mut fheap2.lookup_table.into_iter().map(|(index, valid)| (index + elem_offset, valid)).collect()
        );

        union_heap.lookup_table_spaces.append(&mut fheap1.lookup_table_spaces);
        union_heap.lookup_table_spaces.append(
            &mut fheap2.lookup_table_spaces.into_iter().map(|index| index + lut_offset).collect()
        );

        union_heap.operations.push(format!("Union: {} u {} -> {}", fheap1.name, fheap2.name, union_heap.name));

        return (union_heap, lut_offset);
    }

    pub fn insert(&mut self, data: usize) -> usize {
        self.operations.push(format!("Insert: {}", data));

        let new_i = self.elements.len();

        let lookup_pos = if let Some(pos) = self.lookup_table_spaces.pop() {
            if self.lookup_table[pos].1 {
                panic!("Insert Error: lookup table pos {} is already in use!", pos);
            }
            self.lookup_table[pos].1 = true;
            pos
        } else {
            self.lookup_table.push((new_i,true));
            self.lookup_table.len() - 1
        };

        let mut new_elem = FibonacciHeapElement {
            value: data,
            degree: 0,
            marked: false,
            parent: None,
            child: None,
            left: 0,
            right: 0,
            depth: 0,
            lookup_pos: lookup_pos,
        };

        match self.h_min {
            None => {
                if self.elements.len() > 0 {
                    panic!("H_min cannot be None if there are elements in the heap!");
                }
                // neue wurzelliste
                self.elements.push(new_elem);
                self.ll_insert(new_i, new_i, new_i);
                /*new_elem.left = new_i;
                new_elem.right= new_i;*/
                self.h_min = Some(new_i);
            },
            Some(h_min) => {
                self.elements.push(new_elem);
                self.ll_insert(self.elements[h_min].left, new_i, h_min);

                /*
                let head_left = self.elements[head].left;
                new_elem.left = head_left;
                self.elements[head_left].right = new_i;

                new_elem.right = head;
                self.elements[head].left = new_i;*/

                if self.elements[h_min].value > new_elem.value {
                    self.h_min = Some(new_i);
                }
            }
        }
        

        return lookup_pos;
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

            // remove element z from elements
            self.elements.truncate(self.elements.len() - 1);
            // free up space in lookup table
            self.lookup_table[z_elem.lookup_pos].1 = false;
            self.lookup_table_spaces.push(z_elem.lookup_pos);

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

            self.operations.push(format!("ExtractMin: {}", ret_val));
            return Some(ret_val);
        } else {
            self.operations.push(format!("ExtractMin: None"));
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
            
            // create root ll
            for a_i in a {
                if let Some(a_i) = a_i {
                    if let Some(h_min) = self.h_min {
                        // root ll already exists
                        self.ll_insert(self.elements[h_min].left, a_i, h_min);

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
            self.ll_insert(self.elements[x_child].left, y, x_child);
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

    pub fn decrease_key(&mut self, lookup_index: usize, new_key: usize) {
        if self.elements.len() == 0 {
            panic!("DecreaseKey Error: heap empty!");
        }

        let (x, x_valid) = self.lookup_table[lookup_index];
        if !x_valid {
            panic!("DecreaseKey Error: lookup entry {} is invalid!", lookup_index);
        }
        if self.elements[x].value <= new_key {
            panic!("DecreaseKey Error: new key {} for {} is bigger than current key {}!",
                new_key, x, self.elements[x].value);
        }

        // all good, can decrease key and update heap
        self.operations.push(format!("DecreaseKey: {} {}->{}", x, self.elements[x].value, new_key));

        self.elements[x].value = new_key;
        if let Some(y) = self.elements[x].parent {
            if self.elements[x].value < self.elements[y].value {
                self.cut(x, y);
                self.cascading_cut(y);
            }
        }

        if let Some(h_min) = self.h_min {
            if self.elements[x].value < self.elements[h_min].value {
                self.h_min = Some(x);
            }
        }
    }

    fn cut(&mut self, x: usize, y: usize) {
        // remove x from child list of y
        let x_left = self.elements[x].left;
        let x_right = self.elements[x].right;

        self.elements[x_left].right = x_right;
        self.elements[x_right].left = x_left;

        if let Some(y_child) = self.elements[y].child {
            if y_child == x {
                if x_right == x {
                    self.elements[y].child = None;
                } else {
                    self.elements[y].child = Some(x_right);
                }
            }
        }

        // decrease degree of y
        self.elements[y].degree -= 1;

        // add x to root list
        if let Some(h_min) = self.h_min {
            self.ll_insert(self.elements[h_min].left, x, h_min);

            /*let h_min_left = self.elements[h_min].left;

            self.elements[x].left = h_min_left;
            self.elements[h_min_left].right = x;

            self.elements[x].right = h_min;
            self.elements[h_min].left = x;*/
        }

        // set x.parent = None, x.marked = false
        self.elements[x].parent = None;
        self.elements[x].marked = false;
    }

    fn cascading_cut(&mut self, y: usize) {
        if let Some(z) = self.elements[y].parent {
            if self.elements[y].marked {
                self.cut(y, z);
                self.cascading_cut(z);
            } else {
                self.elements[y].marked = true;
            }
        }
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

        // update lookup table entry
        self.lookup_table[self.elements[z].lookup_pos].0 = z;

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

    fn ll_insert(&mut self, left: usize, insert: usize, right: usize) {
        if self.elements[left].right != right {
            panic!("FibHeap Error: cannot insert {} between {} and {} since {} isn't right neighbor of {} (its {})",
                insert, left, right, right, left, self.elements[left].right
            );
        }
        if self.elements[right].left != left {
            panic!("FibHeap Error: cannot insert {} between {} and {} since {} isn't left neighbor of {} (its {})",
                insert, left, right, left, right, self.elements[right].left
            );
        }


        println!("LL Insert: {} between {} and {}", insert, left, right);
        self.elements[left].right = insert;
        self.elements[insert].left = left;

        self.elements[right].left = insert;
        self.elements[insert].right = right;
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
        let root_label = if let Some(last_op) = self.operations.last() {
            format!("\"Heap {}\nlast op:\n{}\"", self.name, last_op)
        } else {
            format!("\"Heap {}\nlast op:\n-\"", self.name)
        };

        graph.add_stmt(stmt!(
            Subgraph {
                id: id!("sg_root"),
                stmts: vec![stmt!(
                Node {
                    id: node_id!("root"),
                    attributes: vec![
                        Attribute(id!("label"), id!(root_label))
                    ]
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
                    let label = format!("\"{} [{}]\ndeg: {}\nlookup: {}\"",
                                                elem.value, i, elem.degree, elem.lookup_pos);
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

    pub fn print(&mut self) -> Result<(), Error> {
        self.update_depths();
        self.printer.print(self)?;
        self.printer.increase_counter();
        Ok(())
    }

    pub fn size(&self) -> usize { self.elements.len() }
    
    pub fn get_name(&self) -> &String { &self.name }

    // only use if really needed
    pub fn get_lookup_index(&self, key: usize) -> usize {
        for elem in &self.elements {
            if elem.value == key {
                return elem.lookup_pos;
            }
        }
        panic!("Lookup Index Search Error: no element with value {key} in the heap!");
    }
}

struct FibHeapPrinter {
    _timestamp: String,
    counter: usize,
    output_folder: String,
}

impl FibHeapPrinter {
    fn new(output: &String, subfolder: &String) -> Self { 
        //let output_folder_path = format!("./output/{}", timestamp);
        let printer = FibHeapPrinter {
            _timestamp: format!("{}", chrono::prelude::Utc::now().format("%Y%m%d-%H%M")),
            counter: 0,
            output_folder: format!("./{output}/{subfolder}"),
        };

        //println!("Created FibHeap Printer with timestamp: {}", printer.timestamp);
        println!("Created FibHeap Printer with output folder '{}' and subfolder '{}'",
            output,
            subfolder);
        match fs::remove_dir_all(&printer.output_folder) {
            Ok(()) => (),
            Err(_) => ()
        };

        printer
    }

    fn print(&self, fheap: &FibonacciHeap) -> Result<(), Error> {
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
        
        println!("Writing svg to file {filename}");

        let mut file = File::create(filename)?;
        file.write_all(graph_svg.as_slice())?;

        Ok(())
    }

    fn increase_counter(&mut self) {
        self.counter += 1;
    }
}