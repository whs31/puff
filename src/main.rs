mod consts;
mod poppy;

fn main() {
    println!("Hello, world!");

    poppy::run_poppy()
        .map_err(|e| println!("Error: {}", e))
        .unwrap();
}
