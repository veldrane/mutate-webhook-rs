use crate::prelude::*;

#[handler]
pub fn up() -> String {
    ("{{ status: \"UP\" }}\n").to_string()
}
