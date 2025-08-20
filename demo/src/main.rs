use http_test_core::{Json, Status, Text};
use http_test_derive::GET;

fn main() {}

#[GET("https://crates.io/crates/reqwest")]
fn demo1(status: Status, text: Text) {
    println!("{}", text);
    println!("========================================");
    println!("{}", status);
}

#[GET("https://docs.rs/derive_more/2.0.1/derive_more/")]
fn demo2(status: Status, text: Text) {
    println!("{}", text);
    println!("========================================");
    println!("{}", status);
}

#[GET("https://api.kuleu.com/api/getGreetingMessage?type=json")]
fn dem3(status: Status, json: Json) {
    println!("{}", json.pretty());
    println!("========================================");
    println!("{}", status);
}
