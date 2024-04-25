
#[derive(Debug)]
pub struct LinkedList<T> (Option<(T, Box<LinkedList<T>>)>);

impl<T: Clone> LinkedList<T> {
    pub fn new() -> Self { LinkedList(None) }

    pub fn is_empty(&self) -> bool { self.0.is_none() }

    pub fn insert_front(&mut self, data: T) {
        let tail = self.0.take();
        self.0 = Some( (data, Box::new(LinkedList(tail))) );
    }

    pub fn insert_back(&mut self, data: T) {
        match self.0 {
            None => self.0 = Some( (data, Box::new(LinkedList(None)))),
            Some((_, ref mut tail)) => tail.insert_back(data),
        }
    }

    pub fn insert_at<F>(&mut self, data: T, comp: F)
    where
        F: Fn(&T, &T) -> bool
    {
        match self.0 {
            None => {
                self.insert_back(data)
            },
            Some((ref mydata, ref mut tail)) => {
                if comp(&mydata, &data)
                {
                    self.insert_front(data);
                }
                else { tail.insert_at(data, comp); }
            }
        }
    }

    pub fn pop_front(&mut self) -> Result<T,& 'static str> {
        match self.0 {
            None => Err("Linked list is empty!"),
            Some((ref mut data, ref mut ll)) => {
                let front = data.clone();
                self.0 = ll.0.take();
                Ok(front)
            }
        }
    }

    pub fn pop_search<F>(&mut self, search: F) -> Result<T,& 'static str>  
    where
        F: Fn(&T) -> bool
    {
        match self.0 {
            None => Err("Cannot pop element because linked list is empty!"),
            Some((ref mydata, ref mut tail)) => {
                if search(&mydata)
                {
                    self.pop_front()
                }
                else { tail.pop_search(search) }
            }
        }
    }

    pub fn delete_front(&mut self) -> Result<(), & 'static str> {
        match self.0 {
            None => Err("Linked list is empty!"),
            Some((ref mut _data, ref mut ll)) => {
                self.0 = ll.0.take();
                Ok(())
            }
        }
    }

    pub fn delete_search<F>(&mut self, search: F) -> Result<(),& 'static str>
    where
        F: Fn(&T) -> bool
    {
        match self.0 {
            None => Err("Element not found!"),
            Some((ref mydata, ref mut tail)) => {
                if search(&mydata)
                {
                    self.delete_front()
                }
                else { tail.delete_search(search) }
            }
        }
    }

    pub fn peek_front(&self) -> Result<&T,& 'static str> {
        match &self.0 {
            None => Err("Linked list is empty!"),
            Some((data, _)) => Ok(&data),
        }
    }

    pub fn peek_search<F>(&self, search: F) -> Result<&T, & 'static str>
    where
        F: Fn(&T) -> bool
    {
        match self.0 {
            None => {
                Err("Cannot peek at element because the element was not found or the list is empty!")
            },
            Some((ref mydata, ref tail)) => {
                if search(&mydata) {
                    self.peek_front()
                }
                else { tail.peek_search(search) }
            }
        }
    }

    pub fn iter(&self) -> LinkedListIter<'_, T> { LinkedListIter{cur: self} }
}

impl<T: PartialOrd + PartialEq + Clone> LinkedList<T> {
    pub fn insert_ordered(&mut self, data: T) {
        match self.0 {
            None => self.insert_back(data),
            Some((ref mydata, ref mut tail)) => {
                if mydata > &data {
                    self.insert_front(data);
                } else {
                    tail.insert_ordered(data);
                }
            }
        }
    }
}

pub struct LinkedListIter<'a, T> {
    cur: &'a LinkedList<T>,
}

impl<'a, T> Iterator for LinkedListIter<'a, T> {
    type Item = &'a T;

    fn next(&mut self) -> Option<Self::Item> {
        match &self.cur.0 {
            None => None,
            Some((data, ll_)) => {
                self.cur = &ll_;
                Some(&data)
            }
        }
    }
}

