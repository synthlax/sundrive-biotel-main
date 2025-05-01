// use std::any::Any;
// use std::sync::{ Arc, Mutex };
// use uuid::Uuid;

// // Use Type State & Factory Builder pattern
// // Optimize for performance and scalability



// pub struct MathNode;
// pub struct FileNode;
// pub struct DatabaseNode;
// pub struct InputNode;
// pub struct OutputNode;

// struct Inactive;
// struct Running;
// struct Completed;
// struct Failed;

// trait NodeId {
//     fn get_id(&self) -> &Uuid;
// }

// trait NodeName {
//     fn get_name(&self) -> &str;
// }

// trait NextNode {
//     fn next(&self) -> &Arc<Mutex<Node>>;
// }

// trait NodeState {
//     fn get_state(&self) -> &str;
// }

// struct Node {
//     id: Uuid,
//     name: String,
//     state: NodeState,
//     next_node: Arc<Mutex<Node>>,
// }

// impl NodeBuilder<NodeType> {
//     pub fn new() -> Node<NodeType> {
//         Node { node_type: NodeType }
//     }
// }


// pub enum MathOperation {
//     Add,
//     Sub,
//     Div,
//     Mult,
//     Abs
// }


