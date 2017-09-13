// TODO: everything

#[derive(Debug)]
pub struct Tree {
    root: Link,
}

#[derive(Debug)]
pub struct Node {
    elem: i32,
    left: Link,
    right: Link,
}

#[derive(Debug)]
pub enum Link {
    Empty,
    More(Box<Node>),
}

impl Tree {
    pub fn new() -> Self {
        Tree { root: Link::Empty }
    }

    pub fn insert(&mut self, elem: i32) -> bool {
        return self.root.insert(elem);
    }

    pub fn search(&self, elem: i32) -> bool {
        return self.root.search(elem);
    }
}

impl Link {
    pub fn insert(&mut self, elem: i32) -> bool {
        let target_node = self;
         let new_node = Node {
            elem: elem,
            left: Link::Empty,
            right: Link::Empty,
        };
        let boxed_link = Link::More(Box::new(new_node));
        match target_node {
            &mut Link::More(ref mut node) => {
                if node.elem == elem {
                    return false;
                } else if node.elem < elem {
                    node.left.insert(elem)
                } else {
                    node.right.insert(elem)
                }
            },
            _ => {
                *target_node = boxed_link;
                return true;
            }
        }
    }

    pub fn search(&self, elem: i32) -> bool {
        let root = self;
        match root {
            &Link::More(ref node) => {
                if node.elem == elem {
                    return true;
                }else if node.elem < elem {
                    node.left.search(elem)
                }else {
                    node.right.search(elem)
                }
            }
            &Link::Empty => {
                return false;
            }
        }
    }
}

#[cfg(test)]
mod test {
    use super::Tree;

    #[test]
    fn test_insert() {
        let mut bst = Tree::new();

        for i in 0 .. 1000 {
            assert!(bst.insert(i));
        }

        for i in 0 .. 1000 {
            assert!(bst.search(i));
        }
    }

    #[test]
    fn test_search() {
        let mut bst = Tree::new();
        assert!(bst.insert(1));
        assert!(bst.insert(10));
        assert!(bst.insert(4));
        assert!(bst.insert(7));
        assert!(bst.insert(2));

        assert!(bst.search(1));
        assert!(!bst.search(11));
        assert!(bst.search(7));
        assert!(bst.search(10));
        assert!(!bst.search(13));
        assert!(bst.search(4));
        assert!(bst.search(2));
        assert!(bst.search(10));
    }
}

