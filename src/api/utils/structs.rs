use serde::{Deserialize, Serialize};

#[derive(Deserialize, Serialize, Debug)]
pub struct PhotoArg{
    pub photo: String
}

#[derive(Deserialize, Serialize, Debug)]
pub struct AuthReturn {
    pub token: String,
}