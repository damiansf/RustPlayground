mod print;
mod vars;
mod types;
mod strings;
mod tuples;
mod arrays;
mod vectors;
mod conditionals;
mod loops;
mod functions;
mod pointer_ref;
mod structs;
mod enums;
mod cli;

fn main() {
    println!("\nPrint Testing:\n");
    print::run();

    println!("\nVars Testing:\n");
    vars::run();

    println!("\nTypes Testing:\n");
    types::run();

    println!("\nStrings Testing:\n");
    strings::run();

    println!("\nTuples Testing:\n");
    tuples::run();

    println!("\nArrays Testing:\n");
    arrays::run();

    println!("\nVectors Testing:\n");
    vectors::run();

    println!("\nConditionals Testing:\n");
    conditionals::run();

    println!("\nLoops Testing:\n");
    loops::run();

    println!("\nFunctions Testing:\n");
    functions::run();

    println!("\nPointers Testing:\n");
    pointer_ref::run();

    println!("\nStructs Testing:\n");
    structs::run();

    println!("\nEnums Testing:\n");
    enums::run();

    println!("\nCli Testing:\n");
    cli::run();
}
