use std::string::ToString;

fn get_id<T: ToString>(id: T) {
    println!("ID: {}", id.to_string());
}

fn main() {
    get_id("123");
    get_id("123".to_string());
    get_id(uuid::Uuid::new_v4());
}
