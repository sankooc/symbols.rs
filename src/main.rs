pub use sym::sym;
fn main() {
    let mut argv = std::env::args().peekable();
    argv.next();
    if let Some(arg) = argv.next(){
        let size = arg.chars().count();
        if size > 1 {
            sym::load(&arg)
        } else {
            println!("need more than 1 charector");
        }
        
    }
}
