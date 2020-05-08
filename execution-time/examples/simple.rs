use execution_time::execution_time;

#[execution_time]
fn main() {
    let mut item = create(2).unwrap();
    item.req("adas", 2u32);
    item.finish();
}

pub struct Item;

impl Item {
    #[execution_time]
    pub fn req<T>(&mut self, bar1: impl Into<String>, bar2: T) -> i32
    where
        T: Into<u32>,
    {
        2 + 2
    }

    #[execution_time]
    pub fn new() -> Result<Self, String> {
        let _a = 2 + 2;
        Ok(Item)
    }

    #[execution_time]
    fn finish(self) {}
}

#[execution_time("create_new_item")]
fn create<T>(_a: T) -> Result<Item, String> {
    Ok(Item::new()?)
}
