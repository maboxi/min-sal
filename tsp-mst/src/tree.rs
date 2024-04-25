#[derive(Debug)]
pub struct Tree<T>(pub Option<(T, Vec<Tree<T> >)>);

impl<T: Copy> Tree<T> {
    pub fn new() -> Self { Tree(None) }

    pub fn from(data: T) -> Self { Tree(Some((data, Vec::new()))) }

    pub fn is_empty(&self) -> bool { self.0.is_none() }

    pub fn insert_search<F,G>(&mut self, data: T, search: &F, debug: &G)
    where
        F: Fn(&T) -> bool,
        G: Fn(&T, usize) -> ()
    {
        match self.0 {
            None => {
                debug(&data, 0);
                self.0 = Some((data, Vec::new()));
            },
            Some((ref mydata, ref mut children)) => {
                if search(mydata) {
                    debug(&data, 1);
                    children.push(Tree::from(data));
                } else {
                    debug(&data, 2);
                    for child in children {
                        child.insert_search(data, search, debug);
                    }
                }
            }
        }
    }

    pub fn print_preorder<F>(&self, print_func: &F)
    where
        F: Fn(Option<&T>) -> ()
    {
        match self.0 {
            None => print_func(None),
            Some((ref mydata, ref children)) => {
                for child in children {
                    child.print_preorder(print_func);
                }
                print_func(Some(mydata));
            }
        }
    }

    pub fn print_singlenodes<F>(&self, print_func: &F)
    where
        F:Fn(Option<&T>) -> String
    {
        match self.0 {
            None => println!("{}", print_func(None)),
            Some((ref mydata, ref children)) => {
                print!("Node {}", print_func(Some(mydata)));

                if children.len() == 0 {
                    print!(" is leaf");
                } else {
                    print!(": ");
                    for child in children {
                        match child.0 {
                            None => print!("-"),
                            Some(ref data_child) => {
                                print!("{} ", print_func(Some(&data_child.0)));
                            }
                        }
                    }
                }
                println!("");

                for child in children {
                    child.print_singlenodes(print_func);
                }
            }

        }
    }

    pub fn get_data(&self) -> Option<&T> {
        match self.0 {
            None => None,
            Some((ref data, _)) => Some(data)
        }
    }

    pub fn iter_preorder(&self) -> TreeIter<'_, T> {
        TreeIter {nodebuffer: vec![self]}
    }
}

pub struct TreeIter<'a, T> {
    nodebuffer: Vec<&'a Tree<T>>,
}

impl<'a, T> Iterator for TreeIter<'a, T> {
    type Item = &'a T;

    fn next(&mut self) -> Option<Self::Item> {
        let curitem = self.nodebuffer.pop();
        match curitem {
            None => None,
            Some(ref node) => {
                match node.0 {
                    None => None,
                    Some((ref data, ref children)) => {
                        for child in children.iter().rev() {
                            self.nodebuffer.push(child);
                        }
                        Some(data)
                    }
                }
            }
        }
    }
}