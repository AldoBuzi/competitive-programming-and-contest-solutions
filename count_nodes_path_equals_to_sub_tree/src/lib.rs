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

pub fn max_count_solution(node_traversed: i32, current_sum: i32, tree: Node) -> (i32,i32){
    
    if tree.left.is_none() && tree.right.is_none() { return (-1,tree.val); }

    let right_result =  tree.right.map_or((-1,0), |right| max_count_solution(node_traversed + 1, current_sum + tree.val, *right));
    let left_result = tree.left.map_or((-1,0), |left| max_count_solution(node_traversed + 1, current_sum  + tree.val, *left));
    if (left_result.0) != -1 || (right_result.0) != -1{
        return (left_result.0.max(right_result.0), 0);
    }
    if current_sum + tree.val == right_result.1 + left_result.1 {
        return (node_traversed, 0);
    }
    (-1, tree.val + right_result.1 + left_result.1)
}

pub fn solution(tree: Node) -> i32 {
    max_count_solution(0,0,tree).0
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
                Some(Box::new(Node::new(-9, 
                    None,
                    None
                ))), 
                Some(Box::new(Node::new(9, 
                    None, 
                    Some(Box::new(Node::new(-9, 
                        Some(Box::new(Node::new(4, None, None))), 
                        Some(Box::new(Node::new(6, 
                            Some(Box::new(Node::new(-1, None, None))),
                            None
                            )))
                    )))
                ))) 
            )))
        );
        let res = solution(tree);
        assert_eq!(res, 2)
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
        assert_eq!(res, 0)
    }
}
