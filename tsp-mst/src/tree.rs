#[derive(Debug)]
pub struct Tree<T>(Option<(T, Vec<Tree<T>>)>);

impl<T: Copy> Tree<T> {
    pub fn new() -> Self { Tree(None) }

    pub fn from(data: T) -> Self { Tree(Some((data, Vec::new()))) }

    pub fn is_empty(&self) -> bool { self.0.is_none() }

    pub fn insert_search<F>(&mut self, data: T, search: F)
    where
        F: Fn(&T) -> bool
    {
        match self.0 {
            None => {
                self.0 = Some((data, Vec::new()));
            },
            Some((ref mydata, ref mut children)) => {
                if search(mydata) {
                    children.push(Tree::from(data));
                } else {
                    for child in children {
                        child.insert_search(data, &search);
                    }
                }
            }
        }
    }
}