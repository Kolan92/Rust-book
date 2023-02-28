use hello_macro::HelloMacro;
use hello_macro_derive::HelloMacro;

#[derive(HelloMacro)]
struct Pancakes;

#[derive(HelloMacro)]
struct DefinitelyNotPancakes;

fn main() {
    Pancakes::hello_macro();

    DefinitelyNotPancakes::hello_macro();
}
