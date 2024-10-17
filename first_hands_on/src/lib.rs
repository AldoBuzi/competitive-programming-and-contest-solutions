pub struct Node {
    key: u32,
    id_left: Option<usize>,
    id_right: Option<usize>,
}

impl Node {
    fn new(key: u32) -> Self {
        Self {
            key,
            id_left: None,
            id_right: None,
        }
    }
}

pub struct Tree {
    nodes: Vec<Node>,
}

impl Tree {
    pub fn with_root(key: u32) -> Self {
        Self {
            nodes: vec![Node::new(key)],
        }
    }

    /// Adds a child to the node with `parent_id` and returns the id of the new node.
    /// The new node has the specified `key`. The new node is the left  child of the  
    /// node `parent_id` iff `is_left` is `true`, the right child otherwise.
    ///
    /// # Panics
    /// Panics if the `parent_id` does not exist, or if the node `parent_id ` has  
    /// the child already set.
    pub fn add_node(&mut self, parent_id: usize, key: u32, is_left: bool) -> usize {
        assert!(
            parent_id < self.nodes.len(),
            "Parent node id does not exist"
        );
        if is_left {
            assert!(
                self.nodes[parent_id].id_left == None,
                "Parent node has the left child already set"
            );
        } else {
            assert!(
                self.nodes[parent_id].id_right == None,
                "Parent node has the right child already set"
            );
        }

        let child_id = self.nodes.len();
        self.nodes.push(Node::new(key));

        let child = if is_left {
            &mut self.nodes[parent_id].id_left
        } else {
            &mut self.nodes[parent_id].id_right
        };

        *child = Some(child_id);

        child_id
    }

    /// Returns the sum of all the keys in the tree
    pub fn sum(&self) -> u32 {
        self.rec_sum(Some(0))
    }

    /// A private recursive function that computes the sum of
    /// nodes in the subtree rooted at `node_id`.
    fn rec_sum(&self, node_id: Option<usize>) -> u32 {
        if let Some(id) = node_id {
            assert!(id < self.nodes.len(), "Node id is out of range");
            let node = &self.nodes[id];

            let sum_left = self.rec_sum(node.id_left);
            let sum_right = self.rec_sum(node.id_right);

            return sum_left + sum_right + node.key;
        }

        0
    }
    pub fn is_bst(&self) -> bool {
        self.check_bst(0).0
    }
    fn check_bst(&self, node_id: usize) -> (bool, u32, u32) {
        let current_node = &self.nodes[node_id];
        let (right_comparison, right_min, right_max) = match current_node.id_right {
            Some(id) => self.check_bst(id),
            None => (true, u32::MAX, u32::MIN),
        };
        let (left_comparison, left_min, left_max) = match current_node.id_left {
            Some(id) => self.check_bst(id),
            None => (true, u32::MAX, u32::MIN),
        };
        (
            left_comparison
                && right_comparison
                && (left_max <= current_node.key && current_node.key <= right_min),
            right_min.min(left_min).min(current_node.key),
            left_max.max(right_max).max(current_node.key),
        )
    }
    pub fn max_path_solution(&self) -> u32 {
        self.calculate_max_path_sum(0).1
    }
    fn calculate_max_path_sum(&self, node_id: usize) -> (u32, u32) {
        let current_node = &self.nodes[node_id];
        if current_node.id_left.is_none() && current_node.id_right.is_none() {
            return (current_node.key, 0);
        }
        let (left_to_leaf, left_path) = current_node.id_left
            .map_or((0, 0), |left| self.calculate_max_path_sum(left));
        let (right_to_leaf, right_path) = current_node.id_right
            .map_or((0, 0), |right| self.calculate_max_path_sum(right));
        (
            current_node.key + left_to_leaf.max(right_to_leaf),
            left_path
                .max(right_path)
                .max(current_node.key + left_to_leaf + right_to_leaf),
        )
    }
}

#[cfg(test)]
mod tests {
    use super::*;


    /* For both exercises */
    #[test]
    fn only_root() {
        let tree = Tree::with_root(10);
        assert_eq!(tree.is_bst(), true);
        assert_eq!(tree.max_path_solution(),0);
    }

    /* Test cases for "Max path sum" exercise */
    #[test]
    fn four_leaves_two_are_zero() {
        let mut tree = Tree::with_root(10);

        tree.add_node(0, 5, true); // id 1
        tree.add_node(0, 22, false); // id 2


        tree.add_node(1, 7, true); // id 3
        tree.add_node(1, 0, false); // id 4
        
        tree.add_node(2, 0, true); // id 3
        tree.add_node(2, 24, false); // id 4

        assert_eq!(tree.max_path_solution(),68)
    }

    #[test]
    fn only_left_children() {
        let mut tree = Tree::with_root(10);

        tree.add_node(0, 5, true); // id 1
        tree.add_node(1, 22, true); // id 2


        tree.add_node(2, 7, true); // id 3
        tree.add_node(3, 0, true); // id 4

        
        tree.add_node(4, 0, true); // id 3
        tree.add_node(5, 24, true); // id 4

        assert_eq!(tree.max_path_solution(),68)
    }
    #[test]
    fn sum_is_on_the_the_most_far_leaves() {
        let mut tree = Tree::with_root(10);

        tree.add_node(0, 5, true); // id 1
        tree.add_node(0, 22, false); // id 2


        tree.add_node(1, 7, true); // id 3
        tree.add_node(1, 5, false); // id 4

        
        tree.add_node(2, 10, true); // id 3
        tree.add_node(2, 24, false); // id 4

        assert_eq!(tree.max_path_solution(),68)
    }
    #[test]
    fn solution_is_on_left_sub_tree() {
        let mut tree = Tree::with_root(10);

        tree.add_node(0, 5, true); // id 1
        tree.add_node(0, 22, false); // id 2


        tree.add_node(1, 7, true); // id 3
        tree.add_node(1, 0, false); // id 4

        tree.add_node(3, 30, true); // id 5
        tree.add_node(3, 0, false); // id 6

        
        tree.add_node(2, 10, true); // id 7
        tree.add_node(2, 0, false); // id 8

        tree.add_node(7, 7, true); // id 3
        tree.add_node(7, 0, false); // id 4

        assert_eq!(tree.max_path_solution(),91)
    }
    
    /* END of Test cases for "Max path sum" exercise */
    
    /* Test cases for "Check if a tree is a binary search tree */
    #[test]
    fn leaf_in_right_subtree_is_invalid() {
        let mut tree = Tree::with_root(10);

        tree.add_node(0, 5, true); // id 1
        tree.add_node(0, 22, false); // id 2

        tree.add_node(1, 7, false); // id 3
        tree.add_node(2, 20, true); // id 4

        //Invalid node
        tree.add_node(4, 19, false); // id 4

        assert_eq!(tree.is_bst(), false);
    }
    #[test]
    fn both_subtree_are_valid_but_not_the_whole_tree() {
        let mut tree = Tree::with_root(10);

        tree.add_node(0, 5, true); // id 1
        tree.add_node(0, 22, false); // id 2

        tree.add_node(1, 7, false); // id 3
        tree.add_node(2, 20, true); // id 4

        tree.add_node(4, 1, true); // id 5

        assert_eq!(tree.is_bst(), false);
    }
    #[test]
    fn complex_tree_is_not_bst() {
        let mut tree = Tree::with_root(10);

        tree.add_node(0, 5, true); // id 1
        tree.add_node(0, 22, false); // id 2

        tree.add_node(1, 8, false); // id 3
        tree.add_node(2, 20, true); // id 4

        tree.add_node(1, 3, true); // id 5

        tree.add_node(5, 2, true); // id 6
                                   //not a bst for this entry
        tree.add_node(5, 6, false); // id 7

        tree.add_node(3, 7, true); // id 8
        tree.add_node(3, 9, false); // id 9

        tree.add_node(2, 30, false); // id 10

        tree.add_node(4, 18, true); // id 11
        tree.add_node(4, 21, false); // id 12

        tree.add_node(10, 28, true); // id 13
        tree.add_node(10, 33, false); // id 14
        assert_eq!(tree.is_bst(), false);
    }
    #[test]
    fn complex_tree_is_bst() {
        let mut tree = Tree::with_root(10);

        tree.add_node(0, 5, true); // id 1
        tree.add_node(0, 22, false); // id 2

        tree.add_node(1, 8, false); // id 3
        tree.add_node(2, 20, true); // id 4

        tree.add_node(1, 3, true); // id 5

        tree.add_node(5, 2, true); // id 6
        tree.add_node(5, 4, false); // id 7

        tree.add_node(3, 6, true); // id 8
        tree.add_node(3, 9, false); // id 9

        tree.add_node(2, 30, false); // id 10

        tree.add_node(4, 18, true); // id 11
        tree.add_node(4, 21, false); // id 12

        tree.add_node(10, 28, true); // id 13
        tree.add_node(10, 33, false); // id 14
        assert_eq!(tree.is_bst(), true);
    }

    /* END of Test cases for "Check if a tree is a binary search tree */
}
