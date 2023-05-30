use std::collections::VecDeque;

type Leaf<T> = Box<BinaryTree<T>>;

#[derive(Debug, PartialEq)]
pub struct BinaryTree<T: PartialOrd> {
    pub value: T,
    pub left: Option<Leaf<T>>,
    pub right: Option<Leaf<T>>,
}

impl<T: PartialOrd> BinaryTree<T>
where
    T: Copy,
{
    pub fn new(value: T) -> Self {
        BinaryTree {
            value,
            left: None,
            right: None,
        }
    }

    pub fn from(new_values: &[T]) -> Self {
        let (first, rest) = new_values.split_first().unwrap();
        let mut root: BinaryTree<T> = BinaryTree::new(*first);

        for value in rest {
            root.insert(*value)
        }
        root
    }

    pub fn insert(&mut self, new_value: T) {
        let mut queue: VecDeque<&mut BinaryTree<T>> = VecDeque::new();
        queue.push_front(self);

        loop {
            let BinaryTree {
                ref mut left,
                ref mut right,
                value,
            } = queue.pop_back().unwrap();

            let cmp_value = value.to_owned();

            if cmp_value > new_value {
                match left {
                    Some(node) => {
                        queue.push_front(node);
                    }
                    None => {
                        *left = Some(Box::new(BinaryTree::new(new_value)));
                        return;
                    }
                }
            } else {
                match right {
                    Some(node) => {
                        queue.push_front(node);
                    }
                    None => {
                        *right = Some(Box::new(BinaryTree::new(new_value)));
                        return;
                    }
                }
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use super::BinaryTree;

    #[test]
    fn create_new_tree() {
        let vec_tree = BinaryTree::from(&[7, 9, 2, 10, 6]);
        let mut tree = BinaryTree::new(7);

        tree.insert(9);
        tree.insert(2);
        tree.insert(10);
        tree.insert(6);

        assert_eq!(tree, vec_tree)
    }
}
