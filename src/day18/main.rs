use std::cell::RefCell;
use std::rc::Rc;

type NodePtr = Rc<RefCell<Node>>;
type NullableNodePtr = Option<NodePtr>;

#[derive(Debug)]
struct Node {
    value: i64,
    prev: NullableNodePtr,
    next: NullableNodePtr,
}

#[derive(Debug)]
enum PairValue<T> {
    PairChild(Box<T>),
    Value(NodePtr),
}

#[derive(Debug)]
struct Pair {
    left: PairValue<Pair>,
    right: PairValue<Pair>,
}

struct PairTree {
    root: Option<Pair>,
    value_head: NullableNodePtr,
    value_tail: NullableNodePtr,
}

fn add_pairs(mut first: PairTree, second: PairTree) -> PairTree {
    let new_root = Pair {
        left: PairValue::PairChild(Box::new(first.root.unwrap())),
        right: PairValue::PairChild(Box::new(second.root.unwrap())),
    };

    let pair_tree = PairTree {
        root: Some(new_root),
        value_head: first.value_head,
        value_tail: second.value_tail,
    };

    first.value_tail.as_mut().unwrap().borrow_mut().next =
        Some(Rc::clone(second.value_head.as_ref().unwrap()));
    second.value_head.unwrap().borrow_mut().prev = Some(Rc::clone(&first.value_tail.unwrap()));
    pair_tree
}

fn parse_line<'a>(
    line: std::str::Chars<'a>,
    pair_tree: PairTree,
) -> (Pair, PairTree, std::str::Chars) {
    let parse = |mut line: std::str::Chars<'a>, mut pair_tree: PairTree| {
        let next_character = line.next().unwrap();
        if next_character == '[' {
            let (pair, pair_tree, line) = parse_line(line, pair_tree);
            return (PairValue::PairChild(Box::new(pair)), line, pair_tree);
        } else {
            let value = next_character.to_digit(10).unwrap() as i64;
            let value = Rc::new(RefCell::new(Node {
                value,
                prev: None,
                next: None,
            }));
            match pair_tree.value_tail {
                Some(node) => {
                    value.borrow_mut().prev = Some(Rc::clone(&node));
                    let mut node_mut = node.borrow_mut();
                    node_mut.next = Some(Rc::clone(&value));
                    pair_tree.value_tail = Some(Rc::clone(&value));
                }
                None => {
                    pair_tree.value_head = Some(Rc::clone(&value));
                    pair_tree.value_tail = Some(Rc::clone(&value));
                }
            };

            return (PairValue::Value(value), line, pair_tree);
        };
    };

    let (left, mut line, pair_tree) = parse(line, pair_tree);
    assert_eq!(line.next().unwrap(), ',');
    let (right, mut line, pair_tree) = parse(line, pair_tree);
    assert_eq!(line.next().unwrap(), ']');

    (Pair { left, right }, pair_tree, line)
}

fn get_node(value: &PairValue<Pair>) -> NodePtr {
    match value {
        PairValue::Value(node) => Rc::clone(node),
        PairValue::PairChild(_) => panic!("Expected a value, got a pair"),
    }
}

fn is_node(value: &PairValue<Pair>) -> bool {
    match value {
        PairValue::Value(_) => true,
        PairValue::PairChild(_) => false,
    }
}

fn add_left(node: &NodePtr) {
    if let Some(prev_node) = &node.borrow().prev {
        prev_node.borrow_mut().value += node.borrow().value;
    }
}

fn add_right(node: &NodePtr) {
    if let Some(next_node) = &node.borrow().next {
        next_node.borrow_mut().value += node.borrow().value;
    }
}

fn explode(
    pair: &mut Pair,
    value_head: &mut NullableNodePtr,
    value_tail: &mut NullableNodePtr,
    level: usize,
) -> bool {
    let mut explode_if_pair = |pair_value: &PairValue<Pair>| {
        if let PairValue::PairChild(child) = pair_value {
            if !is_node(&child.left) || !is_node(&child.right) {
                return None;
            }
            let left_node = get_node(&child.left);
            let right_node = get_node(&child.right);
            add_left(&left_node);
            add_right(&right_node);
            let new_node = Rc::new(RefCell::new(Node {
                value: 0,
                prev: left_node.borrow_mut().prev.take(),
                next: right_node.borrow_mut().next.take(),
            }));
            if let Some(prev_node) = new_node.borrow_mut().prev.as_mut() {
                prev_node.borrow_mut().next = Some(Rc::clone(&new_node));
            } else {
                *value_head = Some(Rc::clone(&new_node));
            }
            if let Some(next_node) = new_node.borrow_mut().next.as_mut() {
                next_node.borrow_mut().prev = Some(Rc::clone(&new_node));
            } else {
                *value_tail = Some(Rc::clone(&new_node));
            }
            Some(new_node)
        } else {
            None
        }
    };

    if level >= 3 {
        if let Some(new_node) = explode_if_pair(&pair.left) {
            pair.left = PairValue::Value(new_node);
            return true;
        }
        if let Some(new_node) = explode_if_pair(&pair.right) {
            pair.right = PairValue::Value(new_node);
            return true;
        }

        if let PairValue::PairChild(child) = &mut pair.left {
            return explode(child, value_head, value_tail, level + 1);
        } else if let PairValue::PairChild(child) = &mut pair.right {
            return explode(child, value_head, value_tail, level + 1);
        }
    }

    let mut visit_child = |child: &mut PairValue<Pair>| match child {
        PairValue::PairChild(child) => explode(child, value_head, value_tail, level + 1),
        _ => false,
    };

    visit_child(&mut pair.left) || visit_child(&mut pair.right)
}

fn split(
    pair: &mut Pair,
    value_head: &mut NullableNodePtr,
    value_tail: &mut NullableNodePtr,
) -> bool {
    let mut visit_child = |is_left: bool| {
        let child_ref = if is_left {
            &mut pair.left
        } else {
            &mut pair.right
        };
        if let PairValue::PairChild(child) = child_ref {
            return split(child, value_head, value_tail);
        }

        let split_value = if let PairValue::Value(node) = child_ref {
            if node.borrow().value >= 10 {
                let left_value = node.borrow().value / 2;
                let right_value = node.borrow().value - left_value;
                let left_value = Rc::new(RefCell::new(Node {
                    value: left_value,
                    prev: node.borrow_mut().prev.take(),
                    next: None,
                }));
                let right_value = Rc::new(RefCell::new(Node {
                    value: right_value,
                    prev: Some(Rc::clone(&left_value)),
                    next: node.borrow_mut().next.take(),
                }));

                if let Some(prev) = &left_value.borrow().prev {
                    prev.borrow_mut().next = Some(Rc::clone(&left_value));
                } else {
                    *value_head = Some(Rc::clone(&left_value));
                }

                if let Some(next) = &right_value.borrow().next {
                    next.borrow_mut().prev = Some(Rc::clone(&right_value));
                } else {
                    *value_tail = Some(Rc::clone(&right_value));
                }

                left_value.borrow_mut().next = Some(Rc::clone(&right_value));
                Some(PairValue::PairChild(Box::new(Pair {
                    left: PairValue::Value(left_value),
                    right: PairValue::Value(right_value),
                })))
            } else {
                None
            }
        } else {
            panic!("Expected value");
        };

        match split_value {
            Some(child) => {
                if is_left {
                    pair.left = child;
                } else {
                    pair.right = child;
                }
                true
            }
            None => false,
        }
    };

    visit_child(true) || visit_child(false)
}

fn reduce(mut pair_tree: PairTree) -> PairTree {
    let apply_action =
        |mut pair_tree: PairTree,
         action: fn(&mut Pair, &mut NullableNodePtr, &mut NullableNodePtr) -> bool| {
            let mut root = pair_tree.root.unwrap();
            let mut value_head = pair_tree.value_head;
            let mut value_tail = pair_tree.value_tail;
            let result = action(&mut root, &mut value_head, &mut value_tail);
            pair_tree.root = Some(root);
            pair_tree.value_head = value_head;
            pair_tree.value_tail = value_tail;
            (result, pair_tree)
        };

    loop {
        loop {
            let (result, new_pair_tree) =
                apply_action(pair_tree, |root, value_head, value_tail| {
                    explode(root, value_head, value_tail, 0)
                });
            pair_tree = new_pair_tree;
            if !result {
                break;
            }
        }

        let (result, new_pair_tree) = apply_action(pair_tree, split);
        pair_tree = new_pair_tree;
        if !result {
            break;
        }
    }
    pair_tree
}

fn print_pair_tree(pair: &Pair) {
    let print_pair_value = |value: &PairValue<Pair>| match value {
        PairValue::PairChild(child) => print_pair_tree(child),
        PairValue::Value(node) => {
            print!("{}", node.borrow().value);
        }
    };

    print!("[");
    print_pair_value(&pair.left);
    print!(",");
    print_pair_value(&pair.right);
    print!("]");
}

fn magnitude(pair: &Pair) -> i64 {
    let visit_child = |child: &PairValue<Pair>| match child {
        PairValue::PairChild(child) => magnitude(&child),
        PairValue::Value(node) => node.borrow().value,
    };

    3 * visit_child(&pair.left) + 2 * visit_child(&pair.right)
}

fn main() {
    let file_content = std::fs::read_to_string("res/day18/input.txt").unwrap();
    let pair_tree_from_line = |line: &str| {
        let pair_tree = PairTree {
            root: None,
            value_head: None,
            value_tail: None,
        };
        let mut chars = line.chars();
        assert_eq!(chars.next().unwrap(), '[');
        let (pair, mut pair_tree, _) = parse_line(chars, pair_tree);
        pair_tree.root = Some(pair);
        pair_tree
    };
    let get_pairs = || -> Vec<PairTree> {
        file_content
            .lines()
            .map(|line| pair_tree_from_line(line))
            .collect()
    };

    let result = get_pairs()
        .into_iter()
        .reduce(|a, b| reduce(add_pairs(a, b)))
        .unwrap();

    println!("Part 1 result {}", magnitude(result.root.as_ref().unwrap()));

    let result = file_content
        .lines()
        .enumerate()
        .fold(i64::MIN, |max_magnitude, (i, line1)| {
            file_content
                .lines()
                .enumerate()
                .filter(|(j, _)| i != *j)
                .fold(max_magnitude, |max_magnitude, (_j, line2)| {
                    let first = pair_tree_from_line(line1);
                    let second = pair_tree_from_line(line2);
                    max_magnitude.max(magnitude(
                        reduce(add_pairs(first, second)).root.as_ref().unwrap(),
                    ))
                })
        });

    println!("Part 2 result {}", result);
}
