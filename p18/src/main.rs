// This solution parses snailfish numbers using an LL(1) parser into expression
// trees. These expression trees are then manipulated by explosions, splits,
// adds and reduces.

// I strongly suspect that another solution using just a cursor over snailfish
// number strings that would do string manipulation over them would have been
// much much simpler. But I wanted to practice complex data structures in Rust,
// like expression trees.
use std::cell::RefCell;
use std::fmt::{Display, Formatter};
use std::rc::Rc;

type RcNode = Rc<RefCell<Node>>;

macro_rules! rcnode {
    ($x : expr) => {
        Rc::new(RefCell::<Node>::new($x))
    };
}

mod llparser {
    use crate::{Node, RcNode};
    use std::cell::RefCell;
    use std::iter::Peekable;
    use std::rc::Rc;

    // Grammar:
    // S -> N
    // N -> [X,X]
    // X -> num
    // X -> N

    pub fn parse(mut stream: Peekable<impl Iterator<Item = char>>) -> RcNode {
        rule_s(&mut stream)
    }

    fn rule_s(stream: &mut Peekable<impl Iterator<Item = char>>) -> RcNode {
        let c = stream.peek().unwrap();
        let node = match c {
            '[' => rule_n(stream),
            _ => panic!(),
        };
        if stream.next().is_some() {
            panic!();
        }
        node
    }

    fn rule_n(stream: &mut Peekable<impl Iterator<Item = char>>) -> RcNode {
        let mut c = stream.next().unwrap();
        let x1 = match c {
            '[' => rule_x(stream),
            _ => panic!(),
        };

        c = stream.next().unwrap();
        let x2 = match c {
            ',' => rule_x(stream),
            _ => panic!(),
        };

        c = stream.next().unwrap();
        if c != ']' {
            panic!();
        }

        let result = rcnode!(Node::new_pair(x1.clone(), x2.clone()));
        x1.borrow_mut().parent = Some(result.clone());
        x2.borrow_mut().parent = Some(result.clone());
        result
    }

    fn rule_x(stream: &mut Peekable<impl Iterator<Item = char>>) -> RcNode {
        let c = stream.peek().unwrap();
        let node = match c {
            '[' => rule_n(stream),
            '0'..='9' | '-' => {
                let mut s = String::new();
                while let Some(c @ ('0'..='9' | '-')) = stream.peek() {
                    s.push(*c);
                    stream.next();
                }
                let num = s.parse::<i32>().unwrap();
                rcnode!(Node::new_regular(num))
            }
            _ => panic!(),
        };
        node
    }
}

fn parse_input(handle: impl std::io::BufRead) -> Vec<RcNode> {
    handle
        .lines()
        .map(|line_result| {
            let line = line_result.unwrap();
            let node = llparser::parse(line.chars().peekable());
            node
        })
        .collect()
}

#[derive(Clone)]
enum NodePayload {
    Regular(i32),
    Pair { left: RcNode, right: RcNode },
}

#[derive(Clone)]
pub struct Node {
    payload: NodePayload,
    parent: Option<RcNode>,
}

impl Node {
    fn new_regular(v: i32) -> Self {
        let payload = NodePayload::Regular(v);
        Node {
            payload,
            parent: None,
        }
    }

    fn new_pair(left: RcNode, right: RcNode) -> Self {
        let payload = NodePayload::Pair { left, right };
        Node {
            payload,
            parent: None,
        }
    }

    fn is_same(n1: &Node, n2: &Node) -> bool {
        std::ptr::eq(n1, n2)
    }

    fn get_parent(&self) -> Option<RcNode> {
        self.parent.clone()
    }

    fn get_node_to_the_left(node: RcNode) -> Option<RcNode> {
        let mut current = node;
        let mut parent;

        // Go up
        loop {
            parent = match current.borrow().get_parent() {
                None => return None,
                Some(p) => p,
            };
            let parent_payload = &parent.borrow().payload;

            if let NodePayload::Pair { right, .. } = parent_payload {
                if Node::is_same(&current.borrow(), &right.borrow()) {
                    break;
                } else {
                    current = parent.clone();
                }
            } else {
                panic!();
            }
        }

        // Go down and right
        let left = match &parent.borrow().payload {
            NodePayload::Pair { left, .. } => left.clone(),
            _ => panic!(),
        };
        current = left;

        loop {
            let right = match &current.borrow().payload {
                NodePayload::Regular(_) => return Some(current.clone()),
                NodePayload::Pair { right, .. } => right.clone(),
            };
            current = right;
        }
    }

    fn get_node_to_the_right(node: RcNode) -> Option<RcNode> {
        let mut current = node;
        let mut parent;

        // Go up
        loop {
            parent = match current.borrow().get_parent() {
                None => return None,
                Some(p) => p,
            };
            let parent_payload = &parent.borrow().payload;

            if let NodePayload::Pair { left, .. } = parent_payload {
                if Node::is_same(&current.borrow(), &left.borrow()) {
                    break;
                } else {
                    current = parent.clone();
                }
            } else {
                panic!();
            }
        }

        // Go down and left
        let right = match &parent.borrow().payload {
            NodePayload::Pair { right, .. } => right.clone(),
            _ => panic!(),
        };
        current = right;

        loop {
            let left = match &current.borrow().payload {
                NodePayload::Regular(_) => return Some(current.clone()),
                NodePayload::Pair { left, .. } => left.clone(),
            };
            current = left;
        }
    }

    fn explode(node: RcNode) {
        let node_to_the_left_opt = Node::get_node_to_the_left(node.clone());
        let node_to_the_right_opt = Node::get_node_to_the_right(node.clone());

        let (left, right) = match &node.borrow().payload {
            NodePayload::Pair { left, right } => (left.clone(), right.clone()),
            _ => panic!(),
        };

        let left_val = match left.borrow().payload {
            NodePayload::Regular(v) => v,
            _ => panic!(),
        };
        let right_val = match right.borrow().payload {
            NodePayload::Regular(v) => v,
            _ => panic!(),
        };

        if let Some(node_to_the_left) = node_to_the_left_opt {
            match node_to_the_left.borrow_mut().payload {
                NodePayload::Regular(ref mut v) => *v += left_val,
                _ => panic!(),
            }
        }

        if let Some(node_to_the_right) = node_to_the_right_opt {
            match node_to_the_right.borrow_mut().payload {
                NodePayload::Regular(ref mut v) => *v += right_val,
                _ => panic!(),
            }
        }

        node.borrow_mut().payload = NodePayload::Regular(0);
    }

    fn split(node: RcNode) {
        let val = match node.borrow().payload {
            NodePayload::Regular(v) => v,
            _ => panic!(),
        };
        let left_val = val / 2;
        let right_val = val - left_val;
        let left_node = rcnode!(Node::new_regular(left_val));
        let right_node = rcnode!(Node::new_regular(right_val));
        node.borrow_mut().payload = NodePayload::Pair {
            left: left_node.clone(),
            right: right_node.clone(),
        };
        left_node.borrow_mut().parent = Some(node.clone());
        right_node.borrow_mut().parent = Some(node);
    }

    fn add(n1: RcNode, n2: RcNode) -> RcNode {
        let node = rcnode!(Node::new_pair(n1.clone(), n2.clone()));
        n1.borrow_mut().parent = Some(node.clone());
        n2.borrow_mut().parent = Some(node.clone());
        node
    }

    fn reduce(root: RcNode) {
        let mut changed = true;
        while changed {
            changed = Node::in_order_traversal_explode(root.clone(), 0);
            if changed {
                continue;
            }
            changed = Node::in_order_traversal_split(root.clone());
        }
    }

    fn in_order_traversal_explode(node: RcNode, depth: usize) -> bool {
        let (left, right) = match &node.borrow().payload {
            NodePayload::Pair { left, right } => (left.clone(), right.clone()),
            _ => return false,
        };

        if Node::in_order_traversal_explode(left.clone(), depth + 1) {
            return true;
        }

        if depth > 3
            && matches!(left.borrow().payload, NodePayload::Regular(_))
            && matches!(right.borrow().payload, NodePayload::Regular(_))
        {
            Node::explode(node);
            return true;
        }

        if Node::in_order_traversal_explode(right, depth + 1) {
            return true;
        }
        false
    }

    fn in_order_traversal_split(node: RcNode) -> bool {
        let should_split = matches!(node.borrow().payload, NodePayload::Regular(v) if v >= 10);

        if should_split {
            Node::split(node);
            return true;
        }

        let (left, right) = match &node.borrow().payload {
            NodePayload::Pair { left, right } => (left.clone(), right.clone()),
            _ => return false,
        };

        if Node::in_order_traversal_split(left) {
            return true;
        }
        if Node::in_order_traversal_split(right) {
            return true;
        }
        false
    }

    fn magnitude(node: RcNode) -> i32 {
        match &node.borrow().payload {
            NodePayload::Regular(v) => *v,
            NodePayload::Pair { left, right } => {
                3 * Node::magnitude(left.clone()) + 2 * Node::magnitude(right.clone())
            }
        }
    }

    fn deep_clone(node: RcNode) -> RcNode {
        match &node.borrow().payload {
            NodePayload::Regular(v) => {
                rcnode!(Node::new_regular(*v))
            }
            NodePayload::Pair { left, right } => {
                let new_left = Node::deep_clone(left.clone());
                let new_right = Node::deep_clone(right.clone());
                let result = rcnode!(Node::new_pair(new_left.clone(), new_right.clone()));
                new_left.borrow_mut().parent = Some(result.clone());
                new_right.borrow_mut().parent = Some(result.clone());
                result
            }
        }
    }
}

impl Display for Node {
    fn fmt(&self, f: &mut Formatter<'_>) -> Result<(), std::fmt::Error> {
        match &self.payload {
            NodePayload::Regular(v) => write!(f, "{}", v),
            NodePayload::Pair { left, right } => {
                write!(f, "[")?;
                write!(f, "{}", left.borrow())?;
                write!(f, ",")?;
                write!(f, "{}", right.borrow())?;
                write!(f, "]")
            }
        }
    }
}

fn part1(snailfish_numbers: &[RcNode]) {
    let base = snailfish_numbers[0].clone();
    let result = snailfish_numbers.iter().skip(1).fold(base, |acc, node| {
        let add_node = Node::add(acc, node.clone());
        Node::reduce(add_node.clone());
        add_node
    });
    println!("Part 1 - Result: {}", Node::magnitude(result));
}

fn part2(snailfish_numbers: &[RcNode]) {
    let mut best = 0;
    for (i, node_i) in snailfish_numbers.iter().enumerate() {
        for (j, node_j) in snailfish_numbers.iter().enumerate() {
            if i == j {
                continue;
            }
            let node_i_tmp = Node::deep_clone(node_i.clone());
            let node_j_tmp = Node::deep_clone(node_j.clone());
            let sum = Node::add(node_i_tmp, node_j_tmp);
            Node::reduce(sum.clone());
            let magnitude = Node::magnitude(sum);
            if magnitude > best {
                best = magnitude;
            }
        }
    }
    println!("Part 2 - Result: {}", best);
}

fn main() {
    let snailfish_numbers = parse_input(std::io::stdin().lock());
    let snailfish_numbers_part1 = snailfish_numbers;
    let snailfish_numbers_part2 = snailfish_numbers_part1
        .iter()
        .map(|n| Node::deep_clone(n.clone()))
        .collect::<Vec<RcNode>>();
    part1(&snailfish_numbers_part1);
    part2(&snailfish_numbers_part2);
}
