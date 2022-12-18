use std::cell::RefCell;
use std::collections::HashMap;
use std::rc::Rc;

// Phoned it in from https://fasterthanli.me/series/advent-of-code-2022/part-7
pub fn solve(input: String) {
    let root = Rc::new(RefCell::new(Node::default()));
    let mut node = root.clone();
    for line in input.lines() {
        let split_line: Vec<&str> = line.split(" ").collect();
        match split_line[0] {
            "$" => match split_line[1] {
                "cd" => match split_line[2] {
                    "/" => (/* Ignoring root. */),
                    ".." => {
                        let parent = node.borrow().parent.clone().unwrap();
                        node = parent;
                    }
                    dir => {
                        let child = node
                            .borrow_mut()
                            .children
                            .entry(dir.to_owned())
                            .or_default()
                            .clone();
                        node = child;
                    }
                },
                _ => (),
            },
            "dir" => {
                let dir = split_line[1];
                let entry = node
                    .borrow_mut()
                    .children
                    .entry(dir.to_owned())
                    .or_default()
                    .clone();
                entry.borrow_mut().parent = Some(node.clone());
            }
            size => {
                let name = split_line[1];
                let parsed_size = size.parse::<i32>().unwrap();
                let entry = node
                    .borrow_mut()
                    .children
                    .entry(name.to_owned())
                    .or_default()
                    .clone();
                entry.borrow_mut().size = parsed_size;
                entry.borrow_mut().parent = Some(node.clone());
            }
        }
    }
    let total_space = 70_000_000;
    let used_space = root.borrow().total_size();
    let free_space = total_space - used_space;
    let needed_free_space = 30_000_000;
    let minimum_space_to_free = needed_free_space - free_space;
    let removed_dir_size = all_dirs(root)
        .map(|d| d.borrow().total_size())
        .filter(|&s| s >= minimum_space_to_free)
        .min();
    dbg!(removed_dir_size);
}

type NodeHandle = Rc<RefCell<Node>>;

#[derive(Debug, Default)]
struct Node {
    size: i32,
    children: HashMap<String, NodeHandle>,
    parent: Option<NodeHandle>,
}

impl Node {
    fn is_dir(&self) -> bool {
        self.size == 0 && !self.children.is_empty()
    }

    fn total_size(&self) -> i32 {
        self.children
            .values()
            .map(|child| child.borrow().total_size())
            .sum::<i32>()
            + self.size
    }
}

fn all_dirs(n: NodeHandle) -> Box<dyn Iterator<Item = NodeHandle>> {
    #[allow(clippy::needless_collect)]
    let children = n.borrow().children.values().cloned().collect::<Vec<_>>();

    Box::new(
        std::iter::once(n).chain(
            children
                .into_iter()
                .filter_map(|c| {
                    if c.borrow().is_dir() {
                        Some(all_dirs(c))
                    } else {
                        None
                    }
                })
                .flatten(),
        ),
    )
}
