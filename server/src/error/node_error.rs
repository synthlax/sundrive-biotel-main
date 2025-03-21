pub enum NodeError {
    MathOperationError,
    FileConversionError,
    DatabaseAlgorithmError,
    VisualizeError
}

#[allow(unused)]
impl NodeError {
    pub fn to_string(&self) -> String {
        match self {
            NodeError::MathOperationError => "Failed to conduct mathmatical operation. Please revise your inputs.".to_string(),
            NodeError::FileConversionError => "Failed to convert file. Please ensure file contents are properly formatted".to_string(),
            NodeError::DatabaseAlgorithmError => "Failed to initiate database search. Please revise your inputs.".to_string(),
            NodeError::VisualizeError => "Unable to cast information into graphical interpretation. Please revise your inputs".to_string()
        }
    }

}


impl ToString for NodeError {
    fn to_string(&self) -> String {
        self.to_string().to_owned()
    }
}