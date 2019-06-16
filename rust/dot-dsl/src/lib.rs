pub mod graph {

    use graph_items::edge::Edge;
    use graph_items::node::Node;
    use std::collections::HashMap;

    pub mod graph_items {
        pub mod edge {
            use std::collections::HashMap;

            #[derive(PartialEq, Debug)]
            pub struct Edge {
                from: String,
                to: String,
                attrs: HashMap<String, String>,
            }

            impl Edge {
                pub fn new(from: &str, to: &str) -> Self {
                    Edge {
                        from: String::from(from),
                        to: String::from(to),
                        attrs: HashMap::new(),
                    }
                }

                pub fn with_attrs(mut self, v: &[(&str, &str)]) -> Self {
                    self.attrs = v
                        .iter()
                        .map(|(key, value)| (String::from(*key), String::from(*value)))
                        .collect();
                    self
                }
            }
        }

        pub mod node {
            use std::collections::HashMap;

            #[derive(Clone, PartialEq, Debug)]
            pub struct Node {
                name: String,
                attrs: HashMap<String, String>,
            }

            impl Node {
                pub fn new(name: &str) -> Self {
                    Node {
                        name: String::from(name),
                        attrs: HashMap::new(),
                    }
                }

                pub fn with_attrs(mut self, v: &[(&str, &str)]) -> Self {
                    self.attrs = v
                        .iter()
                        .map(|(key, value)| (String::from(*key), String::from(*value)))
                        .collect();
                    self
                }

                pub fn get_attr(&self, attr: &str) -> Option<&str> {
                    self.attrs.get(attr).map(|s| s.as_str())
                }

                pub fn name(&self) -> &str {
                    &self.name
                }
            }
        }
    }

    #[derive(Default)]
    pub struct Graph {
        pub nodes: Vec<graph_items::node::Node>,
        pub edges: Vec<graph_items::edge::Edge>,
        pub attrs: HashMap<String, String>,
    }

    impl Graph {
        pub fn new() -> Self {
            Graph {
                nodes: vec![],
                edges: vec![],
                attrs: HashMap::new(),
            }
        }

        pub fn with_nodes(self, nodes: &[Node]) -> Self {
            Graph {
                nodes: Vec::from(nodes),
                ..self
            }
        }

        pub fn with_edges(self, edges: &[Edge]) -> Self {
            Graph {
                edges: Vec::from(edges),
                ..self
            }
        }

        pub fn with_attrs(self, attrs: &[(&str, &str)]) -> Self {
            Graph {
                attrs: attrs
                    .iter()
                    .map(|(key, value)| (String::from(*key), String::from(*value)))
                    .collect(),

                ..self
            }
        }

        pub fn get_node(self, name: &str) -> Option<Node> {
            self.nodes.iter().find(|n| n.name() == name).cloned()
        }
    }
}
