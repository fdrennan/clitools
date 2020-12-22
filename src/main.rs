/*#[warn(unused_imports)]
use std::{env, fs, io};*/
use postgres::{Client, NoTls};

fn main() {
    // Get File Path
    /*    let dir = env::current_dir();

        println!("The current directory is {:?}", dir);

        let mut entries = fs::read_dir(".")?
            .map(|res| res.map(|e| e.path()))
            .collect::<Result<Vec<_>, io::Error>>()?;

        entries.sort();

        let mut index = 0;
        for x in entries.iter() {
            index = index + 1;
            println!("{} {:?}", index, x);
        }
    */

    // POSTGRES
    let url = "postgresql://fdrennan:thirdday1@localhost:5432/postgres";
    let mut client = Client::connect(url, NoTls).unwrap();

    client
        .execute(
            "
      create table blog (
        id serial primary key,
        title varchar(255),
        body text
        )",
            &[],
        )
        .ok()
        .expect("Table creation failed");

    client.batch_execute(
        "
    CREATE TABLE person (
        id      SERIAL PRIMARY KEY,
        name    TEXT NOT NULL,
        data    BYTEA
    )
    ",
    ).unwrap();

    let name = "Ferris";
    let data = None::<&[u8]>;
    client.execute(
        "INSERT INTO person (name, data) VALUES ($1, $2)",
        &[&name, &data],
    ).unwrap();

    for row in client.query("SELECT id, name, data FROM person", &[]).unwrap() {
        let id: i32 = row.get(0);
        let name: &str = row.get(1);
        let data: Option<&[u8]> = row.get(2);

        println!("found person: {} {} {:?}", id, name, data);
    }
}
