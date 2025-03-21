use std::any::Any;
use std::fs::{File, FileType};
use std::sync::{ Arc, Mutex };
use std::thread;
use std::ops::{self, Add};
use uuid::Uuid;

use crate::error::node_error::NodeError;

pub enum NodeType {
    Math, 
    FileConversion,
    DatabaseAlgorithm,
    Input,
    Output,
    Visualize
}

pub enum MathOperation {
    Add,
    Sub,
    Div,
    Mult,
    Abs
}

struct Node {
    id: Uuid,
    name: String,
    node_type: NodeType,
    node_payload: Box<dyn Any>,
    next: Option<Arc<Mutex<Node>>>
}

impl Node {
    pub fn create_math_node(name: String, payload: &'static dyn Any) -> Node {
        let operand_one: u32 = 0;
        let operand_two: u32 = 0;
        Node {
            id: Uuid::new_v4(),
            name: name,
            node_type: NodeType::Math,
            node_payload: Box::new([operand_one, operand_two]),
            next: None
        }
    }

    pub fn create_input_node(name: String) -> Node {
        let input: Option<File> = None;
        Node {
            id: Uuid::new_v4(),
            name: name,
            node_type: NodeType::FileConversion,
            node_payload: Box::new(input),
            next: None
        }
    }

    // pub fn create_database_node() -> Node {
    //     let input: Option<>
    // }
}
