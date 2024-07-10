use std::cell::RefCell;
use std::collections::VecDeque;
use std::sync::{Arc, Weak};

#[derive(Debug)]
struct Node {
    id: usize,
    parent: RefCell<Weak<Node>>,
    children: RefCell<Vec<Arc<Node>>>,
}

impl Node {
    fn new(id: usize) -> Arc<Node> {
        Arc::new(Node {
            id,
            children: RefCell::new(Vec::new()),
            parent: RefCell::new(Weak::new()),
        })
    }
}

#[derive(Debug)]
struct Tree {
    root: Arc<Node>,
}

impl Tree {
    fn new() -> Tree {
        Tree { root: Node::new(1) }
    }

    fn generate(&mut self, depth: usize) {
        let mut current_level = vec![self.root.clone()];
        let mut next_id = 2;

        for i in pi_digits().take(depth).skip(1) {
            let mut next_level = Vec::new();

            let num_parents = current_level.len();
            let num_children = i;
            let avg_children_per_parent = num_children / num_parents;
            let mut extra_children = num_children % num_parents;

            let mut parent_index = 0;

            for _ in 0..num_children {
                if parent_index >= num_parents {
                    parent_index = 0;
                }

                let child = Node::new(next_id);
                {
                    let mut children = current_level[parent_index].children.borrow_mut();
                    children.push(child.clone());
                }
                *child.parent.borrow_mut() = Arc::downgrade(&current_level[parent_index]);
                next_level.push(child);
                next_id += 1;

                if next_level.len()
                    >= (avg_children_per_parent + if extra_children > 0 { 1 } else { 0 })
                {
                    parent_index += 1;
                    extra_children = extra_children.saturating_sub(1)
                }
            }

            current_level = next_level;
        }
    }

    fn traverse(&self) -> Vec<usize> {
        let mut result = Vec::new();
        let mut queue = VecDeque::new();
        queue.push_back(self.root.clone());

        while let Some(node) = queue.pop_front() {
            result.push(node.id);
            for child in node.children.borrow().iter() {
                queue.push_back(child.clone());
            }
        }

        result
    }
}

fn pi_digits() -> impl Iterator<Item = usize> {
    vec![
        1, 4, 1, 5, 9, 2, 6, 5, 3, 5, 8, 9, 7, 9, 3, 2, 3, 8, 4, 6, 2, 6, 4, 3,
    ]
    .into_iter()
}
#[test]
fn tests() {
    let mut tree = Tree::new();
    tree.generate(6);
    println!("{:?}", tree.traverse());
}
