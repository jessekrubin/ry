use serde::{Deserialize, Serialize};

#[derive(Debug, Deserialize, Serialize)]
pub struct Done {
    pub args: Vec<String>,
    pub returncode: i32,

    #[serde(with = "serde_bytes")]
    pub stdout: Vec<u8>,

    #[serde(with = "serde_bytes")]
    pub stderr: Vec<u8>,
}

impl Done {
    pub fn new(args: Vec<String>, returncode: i32, stdout: Vec<u8>, stderr: Vec<u8>) -> Self {
        Self {
            args,
            returncode,
            stdout,
            stderr,
        }
    }
}
