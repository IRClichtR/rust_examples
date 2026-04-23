use super::foo;
// use crate::foo;

pub struct Bar;

impl Bar {
    pub fn coucou() {
        foo::fait_ceci();
    }
}