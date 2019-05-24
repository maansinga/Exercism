use dot_dsl::graph::Graph;
use dot_dsl::graph::graph_items::node::Node;
use dot_dsl::graph::graph_items::edge::Edge;
use std::collections::HashMap;

fn main(){
    let nodes = vec![
        Node::new("a").with_attrs(&[("color", "green")]),
        Node::new("c"),
        Node::new("b").with_attrs(&[("label", "Beta!")]),
    ];

    let edges = vec![
        Edge::new("b", "c"),
        Edge::new("a", "b").with_attrs(&[("color", "blue")]),
    ];

    let attrs = vec![("foo", "1"), ("title", "Testing Attrs"), ("bar", "true")];

    let mut expected_attrs: HashMap<String, String> = HashMap::new();
    expected_attrs.insert("foo".to_string(), "1".to_string());
    expected_attrs.insert("title".to_string(), "Testing Attrs".to_string());
    expected_attrs.insert("bar".to_string(), "true".to_string());

    let graph = Graph::new()
        .with_nodes(&nodes)
        .with_edges(&edges)
        .with_attrs(&attrs);

    assert_eq!(
        graph.nodes,
        vec![
            Node::new("a").with_attrs(&[("color", "green")]),
            Node::new("c"),
            Node::new("b").with_attrs(&[("label", "Beta!")]),
        ]
    );

    assert_eq!(
        graph.edges,
        vec![
            Edge::new("b", "c"),
            Edge::new("a", "b").with_attrs(&[("color", "blue")]),
        ]
    );

    assert_eq!(graph.attrs, expected_attrs);
    println!("{:#?}", graph);
}