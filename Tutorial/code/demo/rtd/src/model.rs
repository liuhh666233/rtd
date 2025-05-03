

#[allow(unused)]
pub struct Item {
    pub name: String,
}

#[allow(unused)]
impl Item {
    pub fn new(name: String) -> Self {
        Self { name }
    }

    pub fn to_prettier_string(&self) {
        todo!();
    }
}

