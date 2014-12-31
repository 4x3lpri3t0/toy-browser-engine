use std::collections::{HashMap,HashSet};

struct Node {
    // data common to all nodes:
    children: Vec<Node>,

    // data specific to each node type:
    node_type: NodeType,
}

enum NodeType {
    Text(String),
    Element(ElementData),
}

// An element includes a tag name and any number of attributes,
// which can be stored as a map from names to values.

//  Robinson doesn’t support namespaces,
// so it just stores tag and attribute names as simple strings.

struct ElementData {
    tag_name: String,
    attributes: AttrMap,
}

pub type AttrMap = HashMap<String, String>;


// CONSTRUCTOR FUNCTIONS
// Finally, some constructor functions to make it easy to create new nodes:

fn text(data: String) -> Node {
  Node {
    children: Vec::new(),
    node_type: NodeType::Text(data)
  }
}

fn elem(name: String, attrs: AttrMap, children: Vec<Node>) -> Node {
  Node {
    children: children,
    node_type: NodeType::Element(ElementData {
      tag_name: name,
      attributes: attrs,
    })
  }
}