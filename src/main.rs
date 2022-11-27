use tableformat::{markdown, table};

fn main() -> Result<(), table::Error> {
    let content = std::io::read_to_string(std::io::stdin())?;
    let table: table::Table = content.as_str().try_into()?;
    let mkdown: markdown::Table = table.into();

    println!("{}", mkdown);

    Ok(())
}
