use util::HasChildren;
use markup::NodeType;
use markup::Node;

pub struct TaggedNode {
    pub is_acceptor: bool,
    pub has_children_acceptors: bool,
    pub kids: Vec<TaggedNode>,
}

impl HasChildren for TaggedNode {

    fn children<'a>(&'a self) -> &'a [TaggedNode] {
        &self.kids
    }
}

impl TaggedNode {

    pub fn new(node: &Node) -> TaggedNode {

        let mut children = Vec::with_capacity(node.children.len());
        let mut has_children_acceptors = false;
        // For now, the only node focus acceptor is `button`.
        let is_acceptor = if let NodeType::Button(_) = node.node_type {
            true
        } else {
            false
        };

        for kid in node.children.iter() {
            let child = TaggedNode::new(kid);
            has_children_acceptors |= child.is_acceptor | child.has_children_acceptors;
            children.push(child);
        }

        TaggedNode {
            is_acceptor: is_acceptor,
            has_children_acceptors: has_children_acceptors,
            kids: children,
        }
    }
}
