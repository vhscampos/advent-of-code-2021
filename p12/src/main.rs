use std::cell::RefCell;
use std::collections::{HashMap, HashSet};
use std::rc::Rc;

struct Node {
    key: String,
    edges: Vec<Rc<RefCell<Node>>>,
}

pub struct Graph {
    nodes_map: HashMap<String, Rc<RefCell<Node>>>,
}

impl Node {
    fn new(k: &str) -> Self {
        Node {
            key: k.to_string(),
            edges: Vec::new()
        }
    }

    fn get_key(&self) -> &str {
        &self.key
    }

    fn is_start(&self) -> bool {
        self.key == "start"
    }

    fn is_end(&self) -> bool {
        self.key == "end"
    }

    fn is_small(&self) -> bool {
        self.key.chars().next().unwrap().is_lowercase()
    }

    fn add_edge(&mut self, to: Rc<RefCell<Node>>) {
        self.edges.push(to);
    }

    fn get_edges(&self) -> &Vec<Rc<RefCell<Node>>> {
        &self.edges
    }
}

impl Graph {
    fn new() -> Self {
        Graph {
            nodes_map: HashMap::new(),
        }
    }

    fn add_node(&mut self, key: &str) -> Rc<RefCell<Node>> {
        let node = Rc::new(RefCell::new(Node::new(key)));
        self.nodes_map.insert(key.to_string(), node.clone());
        node
    }

    fn get_node(&self, key: &str) -> Option<Rc<RefCell<Node>>> {
        self.nodes_map.get(key).cloned()
    }

    fn get_or_add_node(&mut self, key: &str) -> Rc<RefCell<Node>> {
        if let Some(node) = self.nodes_map.get(key) {
            return node.clone();
        }
        self.add_node(key)
    }

    fn get_start(&self) -> Option<Rc<RefCell<Node>>> {
        self.get_node("start")
    }
}

fn parse_input(handle: impl std::io::BufRead) -> Graph {
    let mut g = Graph::new();
    for line_result in handle.lines() {
        let line = line_result.unwrap();
        let mut tokenizer = line.split('-');
        let from = tokenizer.next().unwrap();
        let to = tokenizer.next().unwrap();
        let from_node = g.get_or_add_node(from);
        let to_node = g.get_or_add_node(to);

        from_node.borrow_mut().add_edge(to_node.clone());
        to_node.borrow_mut().add_edge(from_node.clone());
    }
    g
}

mod part1 {
    use crate::*;

    fn visit(node: Rc<RefCell<Node>>, g: &Graph, visited: &mut HashSet<String>, counter: &mut u32) {
        let node_borrowed = node.borrow();
        let key = node_borrowed.get_key();
        if node_borrowed.is_small() && visited.contains(key) {
            return;
        }
        visited.insert(key.to_string());
        if node_borrowed.is_end() {
            *counter += 1;
        } else if !node_borrowed.is_start() {
            let edges = node_borrowed.get_edges();
            for to in edges {
                visit(to.clone(), g, visited, counter);
            }
        }
        visited.remove(key);
    }

    pub fn solve(g: &Graph) {
        let start_node = g.get_start().unwrap();
        let mut visited = HashSet::<String>::new();
        let mut counter = 0;
        visited.insert(start_node.borrow().get_key().to_string());
        for to in start_node.borrow().get_edges() {
            visit(to.clone(), g, &mut visited, &mut counter);
        }
        visited.remove(start_node.borrow().get_key());

        println!("Part 1 - Result: {}", counter);
    }
}

mod part2 {
    use crate::*;

    fn visit(node: Rc<RefCell<Node>>, g: &Graph, visited: &mut HashSet<String>, mut spare: bool, counter: &mut u32) {
        let node_borrowed = node.borrow();
        let key = node_borrowed.get_key();
        let mut spare_used = false;
        if node_borrowed.is_small() && visited.contains(key) {
            if spare {
                spare = false;
                spare_used = true;
            } else {
                return;
            }
        }
        visited.insert(key.to_string());
        if node_borrowed.is_end() {
            *counter += 1;
        } else if !node_borrowed.is_start() {
            let edges = node_borrowed.get_edges();
            for to in edges {
                visit(to.clone(), g, visited, spare, counter);
            }
        }
        if !spare_used {
            visited.remove(key);
        }
    }

    pub fn solve(g: &Graph) {
        let start_node = g.get_start().unwrap();
        let mut visited = HashSet::<String>::new();
        let mut counter = 0;
        visited.insert(start_node.borrow().get_key().to_string());
        for to in start_node.borrow().get_edges() {
            visit(to.clone(), g, &mut visited, true, &mut counter);
        }
        visited.remove(start_node.borrow().get_key());

        println!("Part 2 - Result: {}", counter);
    }
}

fn main() {
    let g = parse_input(std::io::stdin().lock());
    part1::solve(&g);
    part2::solve(&g);
}
