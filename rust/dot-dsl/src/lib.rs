pub mod graph {
    use std::collections::HashMap;

    use crate::graph::graph_items::{edge::Edge, node::Node};

    pub struct Graph {
        pub attrs: HashMap<String, String>,
        pub edges: Vec<Edge>,
        pub nodes: Vec<Node>,
    }

    impl Graph {
        pub fn new() -> Self {
            Graph {
                attrs: HashMap::new(),
                edges: Vec::new(),
                nodes: Vec::new(),
            }
        }

        pub fn with_attrs(mut self, attrs: &[(&str, &str)]) -> Self {
            self.attrs
                .extend(attrs.iter().map(|(k, v)| (k.to_string(), v.to_string())));
            self
        }

        pub fn with_edges(mut self, edges: &Vec<Edge>) -> Self {
            self.edges.extend(edges.iter().cloned());
            self
        }

        pub fn with_nodes(mut self, nodes: &Vec<Node>) -> Self {
            self.nodes.extend(nodes.iter().cloned());
            self
        }

        pub fn node(&self, node: &str) -> Option<&Node> {
            self.nodes.iter().find(|n| n.name == node)
        }
    }

    pub mod graph_items {
        pub mod edge {
            use std::collections::HashMap;

            #[derive(Clone, PartialEq, Debug)]
            pub struct Edge {
                start: String,
                end: String,
                pub attrs: HashMap<String, String>,
            }

            impl Edge {
                pub fn new(start: &str, end: &str) -> Self {
                    Edge {
                        start: start.to_string(),
                        end: end.to_string(),
                        attrs: HashMap::new(),
                    }
                }

                pub fn with_attrs(mut self, attrs: &[(&str, &str)]) -> Self {
                    self.attrs
                        .extend(attrs.iter().map(|(k, v)| (k.to_string(), v.to_string())));
                    self
                }

                pub fn attr(&self, attr: &str) -> Option<&str> {
                    self.attrs.get(attr).map(|s| s.as_str())
                }
            }
        }

        pub mod node {
            use std::collections::HashMap;

            #[derive(Clone, PartialEq, Debug)]
            pub struct Node {
                pub name: String,
                pub attrs: HashMap<String, String>,
            }

            impl Node {
                pub fn new(name: &str) -> Self {
                    Node {
                        name: name.to_string(),
                        attrs: HashMap::new(),
                    }
                }

                pub fn with_attrs(mut self, attrs: &[(&str, &str)]) -> Self {
                    self.attrs
                        .extend(attrs.iter().map(|(k, v)| (k.to_string(), v.to_string())));
                    self
                }

                pub fn attr(&self, attr: &str) -> Option<&str> {
                    self.attrs.get(attr).map(|s| s.as_str())
                }
            }
        }
    }
}
