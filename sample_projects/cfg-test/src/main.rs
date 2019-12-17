fn main() {
    println!("Hello, cfg-test!");
}

#[cfg(test)]
#[test]
fn cfg_test() {
    println!("cfg-tet test executed");
}
