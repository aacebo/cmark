fn main() {
    for path in std::env::args().skip(1) {
        println!("path: {}", path);
    }
}
