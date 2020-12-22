/*use postgres::{Client, NoTls};*/
use dotenv;
use std::{env, fs, io};

fn print_dir() -> io::Result<()> {
    // Get File Path
    let dir = env::current_dir();

    println!("The current directory is {:?}", dir);
    let mut entries = fs::read_dir(".")?
        .map(|res| res.map(|e| e.path()))
        .collect::<Result<Vec<_>, io::Error>>()?;
    entries.sort();

    let mut index = 0;
    for x in entries.iter() {
        index = index + 1;
        println!("{} {:?}", index, &x);
    }

    /*    let mut input_text = String::new();
    io::stdin()
        .read_line(&mut input_text)
        .expect("failed to read from stdin");

    let mut trimmed: i8 = input_text.parse().unwrap();
    trimmed = trimmed - 1;
    println!("asdfasdf {}", trimmed);*/
    /*println!("You selected {:?} ", entries.get(trimmed));*/
    Ok(())
}

/*fn build_table() {

    // POSTGRES
    let url = "postgresql://fdrennan:thirdday1@localhost:5432/postgres";
    let mut client = Client::connect(url, NoTls).unwrap();


    client
        .batch_execute(
            "
     CREATE TABLE person (
         id      SERIAL PRIMARY KEY,
         name    TEXT NOT NULL,
         data    BYTEA
     )
     ",
        )
        .unwrap();

    let name = "Ferris";
    let data = None::<&[u8]>;
    client
        .execute(
            "INSERT INTO person (name, data) VALUES ($1, $2)",
            &[&name, &data],
        )
        .unwrap();

    for row in client
        .query("SELECT id, name, data FROM person", &[])
        .unwrap()
    {
        let id: i32 = row.get(0);
        let name: &str = row.get(1);
        let data: Option<&[u8]> = row.get(2);

        println!("found person: {} {} {:?}", id, name, data);
    }
}


*/

fn read_dot_env() {
    dotenv::dotenv().expect("Failed to read .env file");
    let username = env::var("username").expect("username not found");
    let password = env::var("password").expect("password not found");
    println!("\nThe password is {}", username);
    println!("The username is {}", password);
}
fn main() {
    let _blah = print_dir();
    /*build_table();*/
    read_dot_env();
}
