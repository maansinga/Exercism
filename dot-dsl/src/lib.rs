pub mod graph {
    use std::collections::HashMap;
    use crate::graph::graph_items::node::Node;

    pub trait HasAttributes{
//        type Item;
        fn get_attr(&self, name: &str) -> Option<(&String, &String)>;
    }

    pub mod graph_items{
        pub mod edge{
            use std::collections::HashMap;
            #[derive(Debug, Clone)]
            pub struct Edge{
                end_1: String,
                end_2: String,
                attrs: HashMap<String, String>
            }

            impl Edge{
                pub fn new(end_1: &str, end_2: &str ) -> Self{
                    Edge{end_1: end_1.to_string(), end_2: end_2.to_string(), attrs: HashMap::new()}
                }

                pub fn with_attrs(self, attrs: &[(&str, &str)]) -> Self{
                    self
                }
            }

            impl std::cmp::PartialEq for Edge{
                fn eq(&self, other: &Self) -> bool {
                    self.end_1 == other.end_1 && self.end_2 == other.end_2
                }
            }
        }

        pub mod node{
            use crate::graph::HasAttributes;
            use std::collections::HashMap;

            #[derive(Debug, Clone)]
            pub struct Node{
                pub name: String,
                pub attrs: HashMap<String, String>,
            }

            impl Node{
                pub fn new(s: &str) -> Self{
                    Self {name: s.to_string(), attrs: HashMap::new()}
                }

                pub fn with_attrs(mut self, attrs: &[(&str, &str)]) -> Self{
                    for (k,v) in attrs.iter(){
                        self.attrs.insert(k.to_string(), v.to_string());
                    }
                    self
                }

                pub fn get_attr(&self, name: &str) -> Option<&str> {
                    match self.attrs
                        .iter()
                        .find(|(k, v)| **k == name){
                        Some((_, v)) => Some(v),
                        _ => None
                    }
                }
            }

            impl std::cmp::PartialEq for Node{
                fn eq(&self, other: &Self) -> bool {
                    self.name == other.name
                }
            }
        }
    }
    #[derive(Debug)]
    pub struct Graph{
        pub nodes: Vec<graph_items::node::Node>,
        pub edges: Vec<graph_items::edge::Edge>,
        pub attrs: HashMap<String, String>,
    }

    impl Graph {
        pub fn new() -> Self {
            Self {nodes: Vec::new(), edges: Vec::new(), attrs: HashMap::new()}
        }

        pub fn with_nodes(mut self, nodes: &Vec<graph_items::node::Node>) -> Self {
            for k in nodes{
                self.nodes.push(k.clone())
            }
            self
        }

        pub fn with_edges(mut self, edges: &Vec<graph_items::edge::Edge>) -> Self {
            for k in edges{
                self.edges.push(k.clone())
            }
            self
        }

        pub fn with_attrs(mut self, attrs: &[(&str, &str)]) -> Self {
            for (k, v) in attrs{
                self.attrs.insert(k.to_string(), v.to_string());
            }
            self
        }

        pub fn get_node(&self, name: &str) -> Option<&graph_items::node::Node>{
            self
                .nodes
                .iter()
                .find(|n| (**n).name == name)
        }

        pub fn get_attr(&self, name: &str) -> Option<&str> {
            match self.attrs
                .iter()
                .find(|(k, v)| **k == name){
                Some((_, v)) => Some(v),
                _ => None
            }
        }
    }
}
