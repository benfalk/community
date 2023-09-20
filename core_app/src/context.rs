#[derive(Debug, Clone)]
pub struct Context {
    pub(crate) user_id: i64,
}

impl Context {
    pub fn new(user_id: i64) -> Self {
        if user_id == 0 {
            panic!("Cannot create a context of 0; reserved for root");
        }

        Context { user_id }
    }

    pub fn as_root() -> Self {
        Context { user_id: 0 }
    }
}
