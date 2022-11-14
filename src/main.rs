use tableformat::parser;

fn main() -> Result<(), parser::Error> {
    let content = std::io::read_to_string(std::io::stdin())?;
    let _table: parser::Table = content.try_into()?;

    Ok(())
}
