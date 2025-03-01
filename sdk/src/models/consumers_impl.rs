use crate::Consumer;

use super::consumers::Company;

impl Consumer for Company {
    fn consumes(&self, good_list: &super::Bill) {
        // TODO: just send message or calculate with a bill?
        todo!();
    }
}