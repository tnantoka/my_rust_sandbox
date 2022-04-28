#[derive(Debug)]
pub struct Node {
    pub data: String,
    pub link: Option<Box<Node>>,
}

impl Node {
    pub fn new(data: String, link: Option<Box<Node>>) -> Self {
        Self { data, link }
    }
}

pub struct List {
    pub head: Option<Box<Node>>,
}

impl List {
    pub fn new() -> Self {
        Self { head: None }
    }

    pub fn unshift(&mut self, data: String) {
        let node = Node::new(data, self.head.take());
        self.head = Some(Box::new(node));
    }

    pub fn push(&mut self, data: String) {
        let node = Node::new(data, None);

        match self.head {
            None => {
                self.head = Some(Box::new(node));
            }
            Some(ref mut head) => {
                let mut n = head;
                loop {
                    match n.link {
                        None => {
                            n.link = Some(Box::new(node));
                            break;
                        }
                        Some(ref mut link) => n = link,
                    }
                }
            }
        }
    }

    pub fn get(&self, index: isize) -> Option<String> {
        match self.head {
            None => None,
            Some(ref root) => {
                let mut n = root;
                let mut i = 0;
                loop {
                    if i == index {
                        return Some(n.data.clone());
                    }
                    match n.link {
                        None => return None,
                        Some(ref link) => n = link,
                    }
                    i += 1;
                }
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn it_works() {
        let mut list = List::new();
        list.push(String::from("push1"));
        list.push(String::from("push2"));
        list.unshift(String::from("unshift1"));
        list.unshift(String::from("unshift2"));

        assert_eq!("unshift2", list.get(0).unwrap());
        assert_eq!("unshift1", list.get(1).unwrap());
        assert_eq!("push1", list.get(2).unwrap());
        assert_eq!("push2", list.get(3).unwrap());
    }
}
