use std::env;

fn main() -> std::io::Result<()> {
    let dir = env::current_dir();


    println!("The current directory is {:?}", dir);

    Ok(())
}