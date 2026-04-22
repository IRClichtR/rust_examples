mod foo;
mod bar;
mod utils;

fn main() {
    println!("Hello, world!");
    
    foo::fait_ceci();
    let coucou = bar::Bar::coucou();
    let lower_bar = utils::lower_bar();
}
