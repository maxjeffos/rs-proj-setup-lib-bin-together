pub mod thing;
use thing::inner_get_num;

pub fn get_message() -> String {
    "hello".to_string()
}

pub fn outer_get_num() -> u8 {
    inner_get_num()
}
