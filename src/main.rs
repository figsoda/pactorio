mod util;

fn main() {
    let _ = util::app().map_err(|e|
        println!("{}", e)
    );
}
