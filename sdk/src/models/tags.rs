use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug, Clone, Copy, PartialEq, Eq, Hash, PartialOrd, Ord)]
pub enum Tags {

}

#[test]
fn test() {
    // assert!(Tags::Common.cmp(&Tags::Discount).is_lt());
}