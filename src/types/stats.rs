use serde::{Deserialize, Serialize};

#[derive(Deserialize, Serialize, Debug)]
pub struct Stats {
    pub post_number: usize,
    pub followers_number: usize,
    pub following_number: usize,
    pub name: String,
}
