pub mod alg {
    use crate::graph::{Graph, Node, NodeRcWrapper};

    use std::cell::RefCell;
    use std::cmp::Reverse;
    use std::rc::Rc;

    use priority_queue::PriorityQueue;

    #[derive(Debug)]
    pub enum AlgorithmError {
        CannotFindClosestNode,
        CannotFindPath(String),
    }

    pub fn find_path<T: std::cmp::PartialEq + std::fmt::Display + std::hash::Hash>(
        graph: &mut Graph<T>,
        start: &RefCell<Node<T>>,
        end: &RefCell<Node<T>>,
    ) -> Result<Vec<NodeRcWrapper<T>>, AlgorithmError> {
        // If no nodes exist, return error
        if graph.get_nodes().len() == 0 {
            return Err(AlgorithmError::CannotFindPath(
                "No nodes exist in graph".to_string(),
            ));
        }

        // If start and end nodes are the same, return error
        if *start.borrow().get_value() == *end.borrow().get_value() {
            return Err(AlgorithmError::CannotFindPath(
                "Start and end nodes are the same".to_string(),
            ));
        }

        // Validate that start and end nodes exist
        if !graph.exists(end) {
            return Err(AlgorithmError::CannotFindPath(
                "End node does not exist in graph".to_string(),
            ));
        }

        if !graph.exists(start) {
            return Err(AlgorithmError::CannotFindPath(
                "Start node does not exist in graph".to_string(),
            ));
        }

        // Set start node distance to 0
        start.borrow_mut().set_distance(0);

        // Copy all nodes besides
        let mut unvisited_nodes = PriorityQueue::<NodeRcWrapper<T>, Reverse<u32>>::new();

        for n in graph.get_nodes() {
            unvisited_nodes.push(
                NodeRcWrapper(Rc::clone(&n.0)),
                Reverse(n.0.borrow().get_distance()),
            );
        }

        // Loop until all nodes have been visited
        while !unvisited_nodes.is_empty() {
            // Pop node with smallest distance
            let closest_node = unvisited_nodes.pop();

            if closest_node.is_none() {
                return Err(AlgorithmError::CannotFindClosestNode);
            }

            let closest_node = closest_node.unwrap();
            println!("Checking node {}", closest_node.0 .0.borrow().get_value());

            // Update distance of all edges
            for e in closest_node.0 .0.borrow().get_edges() {
                println!("Checking edge: {}", e.get_node().0.borrow().get_value());

                let node = e.get_node();

                if *node.0.borrow().get_value() == *end.borrow().get_value() {
                    println!("Found path");

                    // Found path, copy into vector and return
                    let mut path = Vec::<NodeRcWrapper<T>>::new();

                    // Add start node to path
                    let start_node = graph.get_node(start);
                    if start_node.is_none() {
                        return Err(AlgorithmError::CannotFindPath(
                            "Start node does not exist in graph".to_string(),
                        ));
                    }

                    path.push(start_node.unwrap());

                    for n in closest_node.0 .0.borrow().get_path() {
                        path.push(NodeRcWrapper(Rc::clone(&n.0)));
                    }
                    path.push(NodeRcWrapper(Rc::clone(&node.0)));

                    return Ok(path);
                }

                let node_found = unvisited_nodes.get(&node);
                if node_found.is_some() {
                    let distance = closest_node.0 .0.borrow().get_distance() + e.get_weight();

                    if distance < node.0.borrow().get_distance() {
                        node.0.borrow_mut().set_distance(distance);

                        // Update priority queue
                        unvisited_nodes.change_priority(&node, Reverse(distance));

                        // Construct path
                        let mut path = Vec::<NodeRcWrapper<T>>::new();

                        // Copy path from closest node to current node
                        for n in closest_node.0 .0.borrow().get_path() {
                            path.push(NodeRcWrapper(Rc::clone(&n.0)));
                        }
                        path.push(NodeRcWrapper(Rc::clone(&node.0)));

                        print!("Node Path: ");
                        for n in path.iter() {
                            print!("{} ", n.0.borrow().get_value());
                        }
                        println!();

                        node.0.borrow_mut().set_path(path);
                    }
                }
            }
        }

        // Only reachable if no path was found
        Err(AlgorithmError::CannotFindPath("No path found".to_string()))
    }

    pub fn calculate_path_cost<T: std::cmp::PartialEq + std::fmt::Display + std::hash::Hash>(
        path: &Vec<NodeRcWrapper<T>>,
    ) -> u32 {
        let mut cost = 0;

        for i in 0..path.len() - 1 {
            let node = path[i].0.borrow();
            let next_node = path[i + 1].0.borrow();

            for e in node.get_edges() {
                if e.get_node().0.borrow().get_value() == next_node.get_value() {
                    cost += e.get_weight();
                }
            }
        }

        cost
    }
}

#[cfg(test)]
mod tests {
    use crate::graph::{Graph, Node};

    use super::alg::{calculate_path_cost, find_path};

    #[test]
    fn simple_directed() {
        let mut graph = Graph::<u32>::new(true);
        // Add nodes (data is moved into node)
        let node_1 = Node::new(1);
        let node_2 = Node::new(2);
        let node_3 = Node::new(3);
        let node_4 = Node::new(4);
        let node_5 = Node::new(5);
        let node_6 = Node::new(6);

        // Add nodes to graph (graph takes ownership of nodes)
        let node_ptr1 = graph.add_node(node_1).expect("Failed to add node");
        let node_ptr2 = graph.add_node(node_2).expect("Failed to add node");
        let node_ptr3 = graph.add_node(node_3).expect("Failed to add node");
        let node_ptr4 = graph.add_node(node_4).expect("Failed to add node");
        let node_ptr5 = graph.add_node(node_5).expect("Failed to add node");
        let node_ptr6 = graph.add_node(node_6).expect("Failed to add node");

        // Add edges

        // 1 -> 2, 1
        graph.add_edge(node_ptr1.0.as_ref(), node_ptr2.0.as_ref(), 1);
        // 1 -> 3, 3
        graph.add_edge(node_ptr1.0.as_ref(), node_ptr3.0.as_ref(), 3);
        // 2 -> 5, 2
        graph.add_edge(node_ptr2.0.as_ref(), node_ptr5.0.as_ref(), 2);
        // 3 -> 4, 2
        graph.add_edge(node_ptr3.0.as_ref(), node_ptr4.0.as_ref(), 3);
        // 5 -> 4, 1
        graph.add_edge(node_ptr5.0.as_ref(), node_ptr4.0.as_ref(), 1);
        // 4 -> 6, 2
        graph.add_edge(node_ptr4.0.as_ref(), node_ptr6.0.as_ref(), 2);

        // Run algorithm
        let solution_path = find_path(&mut graph, node_ptr1.0.as_ref(), node_ptr6.0.as_ref());

        assert!(solution_path.is_ok());

        let solution_path = solution_path.unwrap();

        // Print path
        print!("Path: ");
        for (i, node) in solution_path.iter().enumerate() {
            if i == 0 {
                print!("{} ", node.0.borrow().get_value());
            } else {
                print!("-> {} ", node.0.borrow().get_value());
            }
        }
        println!("");

        // Expected path: 1 -> 2 -> 5 -> 4 -> 6, cost: 6
        assert_eq!(solution_path.len(), 5, "Path length is not 5");

        assert_eq!(
            *solution_path[0].0.borrow().get_value(),
            1,
            "Path incorrect"
        );
        assert_eq!(
            *solution_path[1].0.borrow().get_value(),
            2,
            "Path incorrect"
        );
        assert_eq!(
            *solution_path[2].0.borrow().get_value(),
            5,
            "Path incorrect"
        );
        assert_eq!(
            *solution_path[3].0.borrow().get_value(),
            4,
            "Path incorrect"
        );
        assert_eq!(
            *solution_path[4].0.borrow().get_value(),
            6,
            "Path incorrect"
        );

        let cost = calculate_path_cost(&solution_path);
        assert_eq!(cost, 6, "Path cost is not 6");
    }

    #[test]
    fn complex_directed() {
        let mut graph = Graph::<u32>::new(true);
        // Add nodes (data is moved into node)
        let node_1 = Node::new(1);
        let node_2 = Node::new(2);
        let node_3 = Node::new(3);
        let node_4 = Node::new(4);
        let node_5 = Node::new(5);
        let node_6 = Node::new(6);
        let node_7 = Node::new(7);
        let node_8 = Node::new(8);
        let node_9 = Node::new(9);
        let node_10 = Node::new(10);
        let node_11 = Node::new(11);

        // Add nodes to graph (graph takes ownership of nodes)
        let node_ptr1 = graph.add_node(node_1).expect("Failed to add node");
        let node_ptr2 = graph.add_node(node_2).expect("Failed to add node");
        let node_ptr3 = graph.add_node(node_3).expect("Failed to add node");
        let node_ptr4 = graph.add_node(node_4).expect("Failed to add node");
        let node_ptr5 = graph.add_node(node_5).expect("Failed to add node");
        let node_ptr6 = graph.add_node(node_6).expect("Failed to add node");
        let node_ptr7 = graph.add_node(node_7).expect("Failed to add node");
        let node_ptr8 = graph.add_node(node_8).expect("Failed to add node");
        let node_ptr9 = graph.add_node(node_9).expect("Failed to add node");
        let node_ptr10 = graph.add_node(node_10).expect("Failed to add node");
        let node_ptr11 = graph.add_node(node_11).expect("Failed to add node");

        // Add edges

        // 1 -> 2, 1
        graph.add_edge(node_ptr1.0.as_ref(), node_ptr2.0.as_ref(), 1);
        // 1 -> 4, 1
        graph.add_edge(node_ptr1.0.as_ref(), node_ptr4.0.as_ref(), 1);
        // 1 -> 3, 3
        graph.add_edge(node_ptr1.0.as_ref(), node_ptr3.0.as_ref(), 3);
        // 2 -> 7, 5
        graph.add_edge(node_ptr2.0.as_ref(), node_ptr7.0.as_ref(), 5);
        // 3 -> 4, 2
        graph.add_edge(node_ptr3.0.as_ref(), node_ptr4.0.as_ref(), 2);
        // 3 -> 6, 2
        graph.add_edge(node_ptr3.0.as_ref(), node_ptr6.0.as_ref(), 2);
        // 3 -> 6, 4
        graph.add_edge(node_ptr3.0.as_ref(), node_ptr6.0.as_ref(), 4);
        // 4 -> 5, 6
        graph.add_edge(node_ptr4.0.as_ref(), node_ptr5.0.as_ref(), 6);
        // 4 -> 6, 1
        graph.add_edge(node_ptr4.0.as_ref(), node_ptr6.0.as_ref(), 1);
        // 5 -> 8, 2
        graph.add_edge(node_ptr5.0.as_ref(), node_ptr8.0.as_ref(), 2);
        // 5 -> 9, 2
        graph.add_edge(node_ptr5.0.as_ref(), node_ptr9.0.as_ref(), 2);
        // 6 -> 8, 3
        graph.add_edge(node_ptr6.0.as_ref(), node_ptr8.0.as_ref(), 3);
        // 7 -> 8, 6
        graph.add_edge(node_ptr7.0.as_ref(), node_ptr8.0.as_ref(), 6);
        // 8 -> 10, 2
        graph.add_edge(node_ptr8.0.as_ref(), node_ptr10.0.as_ref(), 2);
        // 9 -> 10, 2
        graph.add_edge(node_ptr9.0.as_ref(), node_ptr10.0.as_ref(), 2);
        // 9 -> 11, 4
        graph.add_edge(node_ptr9.0.as_ref(), node_ptr11.0.as_ref(), 4);
        // 10 -> 11, 3
        graph.add_edge(node_ptr10.0.as_ref(), node_ptr11.0.as_ref(), 3);

        // Run algorithm
        let solution_path = find_path(&mut graph, node_ptr1.0.as_ref(), node_ptr11.0.as_ref());

        assert!(solution_path.is_ok());

        let solution_path = solution_path.unwrap();

        // Print path
        print!("Path: ");
        for (i, node) in solution_path.iter().enumerate() {
            if i == 0 {
                print!("{} ", node.0.borrow().get_value());
            } else {
                print!("-> {} ", node.0.borrow().get_value());
            }
        }
        println!("");

        // Expected path: 1 -> 4 -> 6 -> 8 -> 10 -> 11, cost: 10
        assert_eq!(solution_path.len(), 6, "Path length is not 6");

        assert_eq!(
            *solution_path[0].0.borrow().get_value(),
            1,
            "Path incorrect"
        );
        assert_eq!(
            *solution_path[1].0.borrow().get_value(),
            4,
            "Path incorrect"
        );
        assert_eq!(
            *solution_path[2].0.borrow().get_value(),
            6,
            "Path incorrect"
        );
        assert_eq!(
            *solution_path[3].0.borrow().get_value(),
            8,
            "Path incorrect"
        );
        assert_eq!(
            *solution_path[4].0.borrow().get_value(),
            10,
            "Path incorrect"
        );
        assert_eq!(
            *solution_path[5].0.borrow().get_value(),
            11,
            "Path incorrect"
        );

        let cost = calculate_path_cost(&solution_path);
        assert_eq!(cost, 10, "Path cost is not 10");
    }

    #[test]
    fn directed_vs_undirected() {
        let directed = true;

        for _ in 0..2 {
            // Construct graph
            let mut graph = Graph::<u32>::new(directed);

            // Add nodes (data is moved into node)
            let node_1 = Node::<u32>::new(1);
            let node_2 = Node::<u32>::new(2);
            let node_3 = Node::<u32>::new(3);
            let node_4 = Node::<u32>::new(4);
            let node_5 = Node::<u32>::new(5);
            let node_6 = Node::<u32>::new(6);
            let node_7 = Node::<u32>::new(7);
            let node_8 = Node::<u32>::new(8);
            let node_9 = Node::<u32>::new(9);

            // Add nodes to graph (graph takes ownership of nodes)
            let node_ptr1 = graph.add_node(node_1).expect("Failed to add node");
            let node_ptr2 = graph.add_node(node_2).expect("Failed to add node");
            let node_ptr3 = graph.add_node(node_3).expect("Failed to add node");
            let node_ptr4 = graph.add_node(node_4).expect("Failed to add node");
            let node_ptr5 = graph.add_node(node_5).expect("Failed to add node");
            let node_ptr6 = graph.add_node(node_6).expect("Failed to add node");
            let node_ptr7 = graph.add_node(node_7).expect("Failed to add node");
            let node_ptr8 = graph.add_node(node_8).expect("Failed to add node");
            let node_ptr9 = graph.add_node(node_9).expect("Failed to add node");

            // Add edges
            // 1 -> 2
            graph.add_edge(node_ptr1.0.as_ref(), node_ptr2.0.as_ref(), 2);
            // 1 -> 4
            graph.add_edge(node_ptr1.0.as_ref(), node_ptr4.0.as_ref(), 2);
            // If directed, add 4 -> 1
            if directed {
                graph.add_edge(node_ptr4.0.as_ref(), node_ptr1.0.as_ref(), 2);
            }

            // 2 -> 5
            graph.add_edge(node_ptr2.0.as_ref(), node_ptr5.0.as_ref(), 2);

            // 3 -> 2
            graph.add_edge(node_ptr3.0.as_ref(), node_ptr2.0.as_ref(), 1);

            // 4 -> 5
            graph.add_edge(node_ptr4.0.as_ref(), node_ptr5.0.as_ref(), 2);
            // If directed, add 5 -> 4
            if directed {
                graph.add_edge(node_ptr5.0.as_ref(), node_ptr4.0.as_ref(), 2);
            }
            // 4 -> 7
            graph.add_edge(node_ptr4.0.as_ref(), node_ptr7.0.as_ref(), 2);

            // 5 -> 8
            graph.add_edge(node_ptr5.0.as_ref(), node_ptr8.0.as_ref(), 2);

            // 6 -> 3
            graph.add_edge(node_ptr6.0.as_ref(), node_ptr3.0.as_ref(), 1);
            // 6 -> 5
            graph.add_edge(node_ptr6.0.as_ref(), node_ptr5.0.as_ref(), 1);

            // 7 -> 8
            graph.add_edge(node_ptr7.0.as_ref(), node_ptr8.0.as_ref(), 2);

            // 8 -> 9
            graph.add_edge(node_ptr8.0.as_ref(), node_ptr9.0.as_ref(), 1);

            // 9 -> 6
            graph.add_edge(node_ptr9.0.as_ref(), node_ptr6.0.as_ref(), 1);

            // Run algorithm
            let now = std::time::Instant::now();
            let mut solution_path =
                find_path(&mut graph, node_ptr1.0.as_ref(), node_ptr6.0.as_ref());
            for _ in 0..999 {
                solution_path = find_path(&mut graph, node_ptr1.0.as_ref(), node_ptr6.0.as_ref());
            }
            let elapsed = now.elapsed();
            println!("Time elapsed: {:?}", elapsed);

            assert!(solution_path.is_ok());

            let solution_path = solution_path.unwrap();

            // Print path
            print!("directed: {}, path found: ", directed);
            for (i, node) in solution_path.iter().enumerate() {
                if i == 0 {
                    print!("{} ", node.0.borrow().get_value());
                } else {
                    print!("-> {} ", node.0.borrow().get_value());
                }
            }
            println!("");

            // Expected directed path: 1 -> 2 -> 5 -> 8 -> 9 -> 6, cost: 8
            // Note: there are multiple cost 8 paths, just picks this due to order nodes are inserted (impacting how they are searched)
            if directed {
                assert_eq!(solution_path.len(), 6, "Directed path length is not 6");
                assert_eq!(
                    *solution_path[0].0.borrow().get_value(),
                    1,
                    "Directed path incorrect"
                );
                assert_eq!(
                    *solution_path[1].0.borrow().get_value(),
                    2,
                    "Directed path incorrect"
                );
                assert_eq!(
                    *solution_path[2].0.borrow().get_value(),
                    5,
                    "Directed path incorrect"
                );
                assert_eq!(
                    *solution_path[3].0.borrow().get_value(),
                    8,
                    "Directed path incorrect"
                );
                assert_eq!(
                    *solution_path[4].0.borrow().get_value(),
                    9,
                    "Directed path incorrect"
                );
                assert_eq!(
                    *solution_path[5].0.borrow().get_value(),
                    6,
                    "Directed path incorrect"
                );

                let cost = calculate_path_cost(&solution_path);
                assert_eq!(cost, 8, "Directed path cost is not 8");
            }
            // Expected undirected path: 1 -> 2 -> 3 -> 6, cost: 4
            else {
                assert_eq!(solution_path.len(), 4, "Undirected path is not 4");
                assert_eq!(
                    *solution_path[0].0.borrow().get_value(),
                    1,
                    "Undirected path incorrect"
                );
                assert_eq!(
                    *solution_path[1].0.borrow().get_value(),
                    2,
                    "Undirected path incorrect"
                );
                assert_eq!(
                    *solution_path[2].0.borrow().get_value(),
                    3,
                    "Undirected path incorrect"
                );
                assert_eq!(
                    *solution_path[3].0.borrow().get_value(),
                    6,
                    "Undirected path incorrect"
                );

                let cost = calculate_path_cost(&solution_path);
                assert_eq!(cost, 4, "Undirected path cost is not 4");
            }
        }
    }
}
