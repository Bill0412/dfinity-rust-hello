#[ic_cdk_macros::query]
fn print() {
    let body = reqwest::blocking::get("https://www.rust-lang.org")
    .unwrap()
    .text();

    ic_cdk::print(format!("body = {:?}", body));
}