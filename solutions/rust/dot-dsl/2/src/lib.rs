pub mod graph_items {
    pub mod edge {
        use std::collections::HashMap;

        use crate::graph::Node;

        #[derive(Debug, Clone, PartialEq)]
        pub struct Edge {
            a: Node,
            b: Node,
            attrs: HashMap<String, String>
        }

        impl Edge {
            pub fn new(a: &str, b: &str) -> Self {
                Self {
                    a: Node::new(a),
                    b: Node::new(b),
                    attrs: HashMap::new()
                }
            }

            pub fn with_attrs(mut self, attrs: &[(&str, &str)]) -> Self {
                self.attrs = {
                    attrs.iter().map(|a| {
                        (a.0.to_string(), a.1.to_string())
                    }).collect()
                };
                self
            }

            pub fn attr(&self, key: &str) -> Option<&str> {
                Some(self.attrs.get(key)?.as_str())
            }
        }
    }

    pub mod node {
        use std::collections::HashMap;

        #[derive(PartialEq, Clone, Debug)]
        pub struct Node {
            pub label: String,
            attrs: HashMap<String, String>
        }

        impl Node {
            pub fn new(label: &str) -> Self {
                Self {
                    label: String::from(label),
                    attrs: HashMap::new()
                }
            }

            pub fn with_attrs(mut self, attrs: &[(&str, &str)]) -> Self {
                self.attrs =   attrs.iter().map(|a| {
                        (a.0.to_string(), a.1.to_string())
                    }).collect();
                self
            }

            pub fn attr(&self, key: &str) -> Option<&str> {
                Some(self.attrs.get(key)?.as_str())
            }
        }

    }
}

pub mod graph {
    use std::collections::HashMap;

    pub use crate::graph_items;
    pub use graph_items::{edge::Edge, node::Node};

    #[derive(Debug,Clone)]
    pub struct Graph {
        pub nodes: Vec<Node>,
        pub edges: Vec<Edge>,
        pub attrs: HashMap<String, String>
    }

    impl Graph {
        pub fn new() -> Self {
            Self {
                nodes: vec!(),
                edges: vec!(),
                attrs: HashMap::new()
            }
        }

        pub fn with_nodes(mut self, nodes: &[Node]) -> Self {
            self.nodes = nodes.to_owned();
            self
        }

        pub fn with_edges(mut self, edges: &[Edge]) -> Self {
            self.edges = edges.to_owned();
            self
        }

        pub fn with_attrs(mut self, attrs: &[(&str, &str)]) -> Self {
            self.attrs = {
                attrs.iter().map(|a| {
                    (a.0.to_string(), a.1.to_string())
                }).collect()
            };
            self
        }

        pub fn node(&self, key: &str) -> Option<&Node> {
            self.nodes.iter().filter(|a| {
                a.label == key
            }).next()
        }

    }
}
