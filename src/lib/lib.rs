// Public modules
pub mod alg;

/// Contains all the structures for creating a graph with nodes and edges
pub mod graph {
    use std::cell::RefCell;
    use std::rc::Rc;

    /// Wrapper for Node to allow for Rc<RefCell<Node<T>>> to implement Hash
    pub struct NodeRcWrapper<T: std::cmp::PartialEq + std::hash::Hash>(pub Rc<RefCell<Node<T>>>);

    impl<T: std::cmp::PartialEq + std::hash::Hash> std::cmp::PartialEq for NodeRcWrapper<T> {
        fn eq(&self, other: &Self) -> bool {
            *self.0.borrow().get_value() == *other.0.borrow().get_value()
        }
    }

    impl<T: std::cmp::PartialEq + std::hash::Hash> std::cmp::Eq for NodeRcWrapper<T> {}

    /// Main struct for creating a graph, containing a list of nodes
    /// and a flag for whether the graph is directed or not
    ///
    /// If directed, edges are added in one direction (from -> to)
    /// If undirected, edges are added in both directions (from -> to and to -> from)
    ///
    /// T must implement PartialEq, and Hash
    /// T must be a unique for each node
    pub struct Graph<T>
    where
        T: std::cmp::PartialEq + std::hash::Hash,
    {
        directed: bool,
        nodes: Vec<NodeRcWrapper<T>>,
    }

    impl<T: std::cmp::PartialEq + std::hash::Hash> Graph<T> {
        pub fn new(directed: bool) -> Graph<T> {
            Graph {
                directed,
                nodes: Vec::new(),
            }
        }

        pub fn add_node(&mut self, node: Node<T>) -> Option<NodeRcWrapper<T>> {
            // Check if node with value already exists
            for n in &self.nodes {
                if *n.0.borrow().get_value() == *node.get_value() {
                    return None;
                }
            }

            // Add node
            self.nodes.push(NodeRcWrapper(Rc::new(RefCell::new(node))));

            // Return pointer to node
            Some(NodeRcWrapper(Rc::clone(&self.nodes.last().unwrap().0)))
        }

        pub fn get_nodes(&self) -> &Vec<NodeRcWrapper<T>> {
            &self.nodes
        }

        pub fn add_edge(
            &mut self,
            from: &RefCell<Node<T>>,
            to: &RefCell<Node<T>>,
            weight: u32,
        ) -> bool {
            // Find nodes
            let from_idx = self
                .nodes
                .iter()
                .position(|n| *n.0.borrow().get_value() == *from.borrow().get_value());
            let to_idx = self
                .nodes
                .iter()
                .position(|n| *n.0.borrow().get_value() == *to.borrow().get_value());

            // Check if nodes exist
            if from_idx.is_none() || to_idx.is_none() {
                return false;
            }

            // Add edge from -> to
            self.nodes[from_idx.unwrap()].0.borrow_mut().add_edge(
                weight,
                NodeRcWrapper(Rc::clone(&self.nodes[to_idx.unwrap()].0)),
            );

            // If graph is undirected, add edge to -> from
            if !self.directed {
                self.nodes[to_idx.unwrap()].0.borrow_mut().add_edge(
                    weight,
                    NodeRcWrapper(Rc::clone(&self.nodes[from_idx.unwrap()].0)),
                );
            }

            false
        }

        pub fn exists(&self, node: &RefCell<Node<T>>) -> bool {
            self.nodes
                .iter()
                .find(|n| *n.0.borrow().get_value() == *node.borrow().get_value())
                .is_some()
        }

        pub fn get_node(&self, node: &RefCell<Node<T>>) -> Option<NodeRcWrapper<T>> {
            self.nodes
                .iter()
                .find(|n| *n.0.borrow().get_value() == *node.borrow().get_value())
                .map(|n| NodeRcWrapper(Rc::clone(&n.0)))
        }
    }

    impl<T: std::cmp::PartialEq + std::hash::Hash> std::hash::Hash for NodeRcWrapper<T> {
        fn hash<H: std::hash::Hasher>(&self, state: &mut H) {
            self.0.borrow().get_value().hash(state);
        }
    }

    ///
    /// Node
    ///
    /// T must implement PartialEq, and Hash
    pub struct Node<T>
    where
        T: std::cmp::PartialEq + std::hash::Hash,
    {
        value: Box<T>,
        edges: Vec<Edge<T>>,
        distance: u32,               // distance from source node
        path: Vec<NodeRcWrapper<T>>, // path from source node
    }

    impl<T: std::cmp::PartialEq + std::hash::Hash> Node<T> {
        pub fn new(value: T) -> Node<T> {
            Node {
                value: Box::new(value),
                edges: Vec::new(),
                distance: u32::MAX,
                path: Vec::new(),
            }
        }

        pub fn get_value(&self) -> &T {
            &self.value
        }

        pub fn add_edge(&mut self, weight: u32, node: NodeRcWrapper<T>) {
            // Check if edge already exists
            for e in &self.edges {
                if *e.get_node().0.borrow().get_value() == *node.0.borrow().get_value() {
                    return;
                }
            }

            self.edges.push(Edge::new(weight, node));
        }

        pub fn get_edges(&self) -> &Vec<Edge<T>> {
            &self.edges
        }

        pub fn get_distance(&self) -> u32 {
            self.distance
        }

        pub fn set_distance(&mut self, distance: u32) {
            self.distance = distance;
        }

        pub fn get_path(&self) -> &Vec<NodeRcWrapper<T>> {
            &self.path
        }

        pub fn set_path(&mut self, path: Vec<NodeRcWrapper<T>>) {
            self.path = path;
        }
    }

    impl<T: std::cmp::PartialEq + std::hash::Hash> std::cmp::PartialEq for Node<T> {
        fn eq(&self, other: &Self) -> bool {
            self.value == other.value
        }
    }

    impl<T: std::cmp::PartialEq + std::hash::Hash> std::cmp::Eq for Node<T> {
        // Empty
    }

    impl<T: std::cmp::PartialEq + std::hash::Hash> std::ops::Deref for Node<T> {
        type Target = T;

        fn deref(&self) -> &Self::Target {
            &self.value
        }
    }

    impl<T: std::cmp::PartialEq + std::hash::Hash> std::ops::DerefMut for Node<T> {
        fn deref_mut(&mut self) -> &mut Self::Target {
            &mut self.value
        }
    }

    ///
    /// Edge
    ///
    pub struct Edge<T: std::cmp::PartialEq + std::hash::Hash> {
        weight: u32,
        node: NodeRcWrapper<T>,
    }

    impl<T: std::cmp::PartialEq + std::hash::Hash> Edge<T> {
        pub fn new(weight: u32, node: NodeRcWrapper<T>) -> Edge<T> {
            Edge { weight, node }
        }

        pub fn get_weight(&self) -> u32 {
            self.weight
        }

        pub fn get_node(&self) -> &NodeRcWrapper<T> {
            &self.node
        }
    }
}

///
/// Tests
///
#[cfg(test)]
mod test {
    use super::graph::{Graph, Node};

    #[test]
    fn directed_test() {
        let mut graph = Graph::new(true);

        let node1 = Node::new(1);
        let node2 = Node::new(2);

        let node_ptr1 = graph.add_node(node1).expect("Failed to add node");
        let node_ptr2 = graph.add_node(node2).expect("Failed to add node");

        graph.add_edge(node_ptr1.0.as_ref(), node_ptr2.0.as_ref(), 1);

        // Check that nodes exist
        assert_eq!(
            graph.exists(node_ptr1.0.as_ref()),
            true,
            "Node 1 does not exist in graph"
        );
        assert_eq!(
            graph.exists(node_ptr2.0.as_ref()),
            true,
            "Node 2 does not exist in graph"
        );

        // Check that we can get nodes
        assert_eq!(
            graph.get_node(node_ptr1.0.as_ref()).is_some(),
            true,
            "Failed to get node 1"
        );
        assert_eq!(
            graph.get_node(node_ptr2.0.as_ref()).is_some(),
            true,
            "Failed to get node 2"
        );

        // Check that edge exists from node 1 -> node 2, but not the other way around
        assert_eq!(
            graph
                .get_node(node_ptr1.0.as_ref())
                .unwrap()
                .0
                .borrow()
                .get_edges()
                .len(),
            1,
            "Node 1 does not have an edge to node 2"
        );
        assert_eq!(
            graph
                .get_node(node_ptr2.0.as_ref())
                .unwrap()
                .0
                .borrow()
                .get_edges()
                .len(),
            0,
            "Node 2 has an edge to node 1"
        );
    }

    #[test]
    fn undirected_test() {
        let mut graph = Graph::new(false);

        let node1 = Node::new(1);
        let node2 = Node::new(2);

        let node_ptr1 = graph.add_node(node1).expect("Failed to add node");
        let node_ptr2 = graph.add_node(node2).expect("Failed to add node");

        // Check that duplicate nodes are not added
        let nod_dup = Node::new(1);
        let node_dup_ptr = graph.add_node(nod_dup);
        assert!(node_dup_ptr.is_none(), "Duplicate node was added");

        graph.add_edge(node_ptr1.0.as_ref(), node_ptr2.0.as_ref(), 1);

        // Check that nodes exist
        assert_eq!(
            graph.exists(node_ptr1.0.as_ref()),
            true,
            "Node 1 does not exist in graph"
        );
        assert_eq!(
            graph.exists(node_ptr2.0.as_ref()),
            true,
            "Node 2 does not exist in graph"
        );

        assert_eq!(
            graph.get_node(node_ptr1.0.as_ref()).is_some(),
            true,
            "Failed to get node 1"
        );
        assert_eq!(
            graph.get_node(node_ptr2.0.as_ref()).is_some(),
            true,
            "Failed to get node 2"
        );

        // Check that edge exists from node 1 -> node 2, and the other way around
        assert_eq!(
            graph
                .get_node(node_ptr1.0.as_ref())
                .unwrap()
                .0
                .borrow()
                .get_edges()
                .len(),
            1,
            "Node 1 does not have an edge to node 2"
        );
        assert_eq!(
            graph
                .get_node(node_ptr2.0.as_ref())
                .unwrap()
                .0
                .borrow()
                .get_edges()
                .len(),
            1,
            "Node 2 does not have an edge to node 1"
        );
    }
}
