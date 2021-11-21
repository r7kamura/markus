#[derive(Debug)]
pub struct Tree<T> {
    pub ancestors: Vec<usize>,
    pub current: Option<usize>,
    pub nodes: Vec<Node<T>>,
}

impl<T> Default for Tree<T> {
    fn default() -> Self {
        Self {
            ancestors: Vec::new(),
            current: None,
            nodes: Vec::new(),
        }
    }
}

impl<T> Tree<T> {
    pub fn new() -> Self {
        Self::default()
    }

    /// Append an item to the current position.
    pub fn append(&mut self, item: T) {
        let index = self.create_node(item);
        let next_current = Some(index);

        if let Some(index) = self.current {
            // When some sibling already exists.
            self.nodes[index].next = next_current;
        } else if let Some(&parent) = self.ancestors.last() {
            // When no sibling exists yet, but parent does.
            self.nodes[parent].child = next_current;
        }
        self.current = next_current;
    }

    pub fn go_to_next_sibling(&mut self) {
        let index = self.current.unwrap();
        self.current = self.nodes[index].next;
    }

    pub fn go_to_parent(&mut self) {
        self.current = self.ancestors.pop();
    }

    pub fn go_to_child(&mut self) {
        let index = self.current.unwrap();
        self.ancestors.push(index);
        self.current = self.nodes[index].child;
    }

    pub fn go_to_first(&mut self) {
        self.current = if self.nodes.is_empty() { None } else { Some(0) };
    }

    /// Create a new Node, and return its index.
    fn create_node(&mut self, item: T) -> usize {
        let index = self.nodes.len();
        self.nodes.push(Node {
            child: None,
            item,
            next: None,
        });
        index
    }
}

#[derive(Clone, Copy, Debug)]
pub struct Node<T> {
    pub child: Option<usize>,
    pub next: Option<usize>,
    pub item: T,
}
