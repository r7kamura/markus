#[derive(Debug)]
pub struct Tree<T> {
    pub current: Option<usize>,
    pub nodes: Vec<Node<T>>,
}

impl<T> Default for Tree<T> {
    fn default() -> Self {
        Self {
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
            self.nodes[index].next = next_current;
        }
        self.current = next_current;
    }

    pub fn go_to_next_sibling_of(&mut self, index: usize) {
        self.current = self.nodes[index].next;
    }

    /// Set current with the 1st Node's index, or None.
    pub fn rewind(&mut self) {
        self.current = if self.nodes.is_empty() { None } else { Some(0) };
    }

    /// Create a new Node and return its index.
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
