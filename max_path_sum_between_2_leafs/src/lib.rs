/// A node in the binary tree.
pub struct Node{
    val:i32,
    left:Option<Box<Node>>,
    right:Option<Box<Node>>

}
impl Node {
    fn new(value: i32, left: Option<Box<Node>>, right: Option<Box<Node>> ) -> Node {
        Node {
            val : value,
            left : left,
            right : right
        }
    }
}

pub fn max_path_solution(tree: Node) -> (i32,i32){
    if tree.left.is_none() && tree.right.is_none() { return (tree.val,0); }
    let (left_to_leaf, left_path) = tree.left.map_or(
        (i32::min_value(), i32::min_value()), 
        |left| max_path_solution(*left)
    );
    let (right_to_leaf, right_path) = tree.right.map_or(
        (i32::min_value(), i32::min_value()), 
        |right| max_path_solution(*right)
    );
    return (tree.val + left_to_leaf.max(right_to_leaf), left_path.max(right_path).max(tree.val + left_to_leaf + right_to_leaf));
}

pub fn solution(tree: Node) -> i32 {
    max_path_solution(tree).1
}
#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test1() {
        let tree = Node::new(
            -15,
            Some(Box::new(Node::new(5, 
                Some(Box::new(Node::new(-8, 
                    Some(Box::new(Node::new(2, None, None))), 
                    Some(Box::new(Node::new(-3, None, None)))
                ))), 
                Some(Box::new(Node::new(1, None, None)))
            ))),
            Some(Box::new(Node::new(6, 
                Some(Box::new(Node::new(3, 
                    None,
                    None
                ))), 
                Some(Box::new(Node::new(9, 
                    None, 
                    Some(Box::new(Node::new(0, 
                        Some(Box::new(Node::new(4, None, None))), 
                        Some(Box::new(Node::new(-1, 
                            Some(Box::new(Node::new(10, None, None))),
                            None
                            )))
                    )))
                ))) 
            )))
        );
        let res = solution(tree);
        assert_eq!(res, 27)
    }
    #[test]
    fn test2(){
        let tree = Node::new(
            3, 
            Some(Box::new(Node::new(
                4, 
                Some(Box::new(Node::new(-10, None, None))), 
                Some(Box::new(Node::new(4, None, None)))
            ))), 
            Some(Box::new(Node::new(5, None, None)))
        );
        let res = solution(tree);
        assert_eq!(res, 16)
    }
}
