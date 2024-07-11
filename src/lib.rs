use serde::{Deserialize, Serialize};

use std::cell::RefCell;
use std::collections::VecDeque;
use std::fs::File;
use std::io::{Read, Write};
use std::sync::{Arc, Weak};

#[derive(Debug, Serialize, Deserialize)]
struct Node {
    id: usize,
    #[serde(skip)]
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

#[derive(Debug, Serialize, Deserialize)]
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
            let mut new_children = Vec::new();

            let num_parents = current_level.len();
            let num_children = i;
            let avg_children_per_parent = num_children / num_parents;
            let mut extra_children = num_children % num_parents;

            for _ in 0..num_children {
                let child = Node::new(next_id);
                new_children.push(child);
                next_id += 1;
            }

            let mut child_index = 0;
            for parent in current_level.iter() {
                let num_assign_children =
                    avg_children_per_parent + if extra_children > 0 { 1 } else { 0 };

                extra_children = extra_children.saturating_sub(1);

                for _ in 0..num_assign_children {
                    let child = new_children[child_index].clone();

                    let mut children = parent.children.borrow_mut();
                    children.push(child.clone());

                    *child.parent.borrow_mut() = Arc::downgrade(parent);
                    next_level.push(child);
                    child_index += 1;
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

    fn save_to_file(&self, filename: &str) -> std::io::Result<()> {
        let serialized = bincode::serialize(&self).expect("Serialization failed");
        let mut file = File::create(filename)?;
        file.write_all(&serialized)?;
        Ok(())
    }

    fn load_from_file(filename: &str) -> std::io::Result<Self> {
        let mut file = File::open(filename)?;
        let mut buffer = Vec::new();
        file.read_to_end(&mut buffer)?;
        let tree: Tree = bincode::deserialize(&buffer).expect("Deserialization failed");
        tree.rebuild_parent_links();
        Ok(tree)
    }

    fn rebuild_parent_links(&self) {
        let mut queue = VecDeque::new();
        queue.push_back(self.root.clone());

        while let Some(node) = queue.pop_front() {
            for child in node.children.borrow().iter() {
                *child.parent.borrow_mut() = Arc::downgrade(&node);
                queue.push_back(child.clone());
            }
        }
    }
}

fn pi_digits() -> impl Iterator<Item = usize> {
    vec![
        1, 4, 1, 5, 9, 2, 6, 5, 3, 5, 8, 9, 7, 9, 3, 2, 3, 8, 4, 6, 2, 6, 4, 3,
    ]
    .into_iter()
}

#[cfg(test)]
mod tests {
    use std::fs;

    use super::*;

    #[test]
    fn tree_generation() {
        let mut tree1 = Tree::new();
        tree1.generate(5);
        assert_eq!(tree1.traverse().len(), 20);

        let mut tree2 = Tree::new();
        tree2.generate(3);
        assert_eq!(tree2.traverse().len(), 6);
    }

    #[test]
    fn tree_traversal() {
        let mut tree = Tree::new();
        tree.generate(5);

        assert_eq!(
            tree.traverse(),
            vec![1, 2, 3, 4, 5, 6, 7, 8, 9, 10, 11, 12, 13, 14, 15, 16, 17, 18, 19, 20]
        );
    }

    #[test]
    fn tree_save_load() {
        let mut tree = Tree::new();
        tree.generate(3);
        tree.save_to_file("test_tree.bin")
            .expect("Failed to save tree to file");

        let loaded_tree =
            Tree::load_from_file("test_tree.bin").expect("Failed to load tree from file");

        assert_eq!(loaded_tree.traverse(), vec![1, 2, 3, 4, 5, 6]);

        fs::remove_file("test_tree.bin").expect("Failed to delete test file");
    }

    #[test]
    fn tree_rebuild_parent() {
        let mut tree = Tree::new();
        tree.generate(3);

        let serialized = bincode::serialize(&tree).expect("Serialization failed");
        let deserialized_tree: Tree =
            bincode::deserialize(&serialized).expect("Deserialization failed");

        assert!(deserialized_tree.root.children.borrow()[0]
            .parent
            .borrow()
            .upgrade()
            .is_none());

        deserialized_tree.rebuild_parent_links();

        assert!(deserialized_tree.root.children.borrow()[0]
            .parent
            .borrow()
            .upgrade()
            .is_some());
    }
}
