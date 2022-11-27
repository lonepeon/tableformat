use crate::table;

pub fn parse(content: &str) -> Result<table::Table, table::Error> {
    let mut t = table::Table::default();
    let mut content = parse_column_names(&mut t, content)?;
    let new_content = parse_delimiter_line(&mut t, content)?;
    if let Some(new_content) = new_content {
        content = new_content
    }
    parse_rows(&mut t, content)?;

    Ok(t)
}

fn next(content: &str) -> (Option<char>, &str) {
    let mut chars = content.chars();
    (chars.next(), chars.as_str())
}

fn peek(content: &str) -> Option<char> {
    content.chars().next()
}

fn skip_spaces(content: &str) -> &str {
    content.trim()
}

fn read_until(content: &str, c: char) -> (Option<&str>, &str) {
    for (index, current) in content.chars().enumerate() {
        if current == c {
            return (Some(&content[0..index]), &content[index..]);
        }
    }

    (None, content)
}

fn parse_column_names<'a>(
    t: &mut table::Table<'a>,
    content: &'a str,
) -> Result<&'a str, table::Error> {
    let mut content = skip_spaces(content);
    while let (Some(c), new_content) = next(content) {
        content = new_content;
        match c {
            '|' => {
                if let Some(c) = peek(content) {
                    if c == '\n' {
                        break;
                    }
                }
                let (column, new_content) = parse_column(content)?;
                content = new_content;

                t.add_column(column, None)
            }
            c => Err(table::Error::from(format!(
                "expecting header definition surrounded by | but found {}",
                c
            )))?,
        }
    }

    Ok(content)
}

fn parse_delimiter_line<'a>(
    t: &mut table::Table<'a>,
    content: &'a str,
) -> Result<Option<&'a str>, table::Error> {
    let mut abort = false;
    let mut index = 0;
    let mut content = skip_spaces(content);
    while let (Some(c), new_content) = next(content) {
        content = new_content;
        match c {
            '|' => {
                if let Some(c) = peek(content) {
                    if c == '\n' {
                        break;
                    }
                }
                let (column, new_content) = parse_column(content)?;
                content = new_content;

                if let Some(column) = column {
                    if column.starts_with(':') && column.ends_with(':') {
                        t.columns[index].alignment = Some(table::Alignment::Centered);
                    } else if column.starts_with(':') {
                        t.columns[index].alignment = Some(table::Alignment::Left);
                    } else if column.ends_with(':') {
                        t.columns[index].alignment = Some(table::Alignment::Right);
                    } else if column.starts_with('-') {
                        t.columns[index].alignment = None;
                    } else {
                        abort = true;
                        break;
                    }
                } else {
                    abort = true;
                    break;
                }
                index += 1;
            }
            c => Err(table::Error::from(format!(
                "expecting header delimiter definition surrounded by | but found {}",
                c
            )))?,
        }
    }

    if abort {
        t.no_headers = true;
        for i in 0..t.columns.len() {
            t.columns[i].alignment = None;
            t.push(t.columns[i].name);
            t.columns[i].name = None;
        }
        t.next_row();
        return Ok(None);
    }

    Ok(Some(content))
}

fn parse_rows<'a>(t: &mut table::Table<'a>, content: &'a str) -> Result<(), table::Error> {
    let mut content = skip_spaces(content);
    while let (Some(c), new_content) = next(content) {
        content = new_content;
        if content.is_empty() {
            break;
        }
        match c {
            '|' => {
                if let Some(c) = peek(content) {
                    if c == '\n' {
                        let (_, new_content) = next(content);
                        content = new_content;
                        t.next_row();
                        continue;
                    }
                }
                let (column, new_content) = parse_column(content)?;
                content = new_content;

                t.push(column)
            }
            c => Err(table::Error::from(format!(
                "expecting body definition surrounded by | but found {}",
                c
            )))?,
        }
    }

    Ok(())
}

fn parse_column(content: &str) -> Result<(Option<&str>, &str), table::Error> {
    let (column, new_content) = read_until(content, '|');
    let column_name =
        column.ok_or_else(|| table::Error::from("failed to find closing |".to_string()))?;
    let column_name = column_name.trim();
    if column_name.is_empty() {
        Ok((None, new_content))
    } else {
        Ok((Some(column_name), new_content))
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn parse_valid_table_with_header() {
        let content = "| column 1  | column 2    |                  |              column 4 |
|-----------|:------------|:----------------:|----------------------:|
| value 1.1 | a value 1.2 | a long value 1.3 | a long long value 1.4 |
| value 2.1 | value 2.2   |                  |                       |
| value 3.1 | value 3.2   |    value 3.3     |             value 3.4 |
";

        let t = parse(content).unwrap();
        assert_eq!(4, t.columns.len(), "number of columns");

        assert_eq!(Some("column 1"), t.columns[0].name);
        assert_eq!(Some("column 2"), t.columns[1].name);
        assert_eq!(None, t.columns[2].name);
        assert_eq!(Some("column 4"), t.columns[3].name);

        assert_eq!(None, t.columns[0].alignment);
        assert_eq!(Some(table::Alignment::Left), t.columns[1].alignment);
        assert_eq!(Some(table::Alignment::Centered), t.columns[2].alignment);
        assert_eq!(Some(table::Alignment::Right), t.columns[3].alignment);

        assert_eq!(3, t.columns[0].values.len());
        assert_eq!(Some("value 1.1"), t.columns[0].values[0]);
        assert_eq!(Some("value 2.1"), t.columns[0].values[1]);
        assert_eq!(Some("value 3.1"), t.columns[0].values[2]);

        assert_eq!(3, t.columns[1].values.len());
        assert_eq!(Some("a value 1.2"), t.columns[1].values[0]);
        assert_eq!(Some("value 2.2"), t.columns[1].values[1]);
        assert_eq!(Some("value 3.2"), t.columns[1].values[2]);

        assert_eq!(3, t.columns[2].values.len());
        assert_eq!(Some("a long value 1.3"), t.columns[2].values[0]);
        assert_eq!(None, t.columns[2].values[1]);
        assert_eq!(Some("value 3.3"), t.columns[2].values[2]);

        assert_eq!(3, t.columns[3].values.len());
        assert_eq!(Some("a long long value 1.4"), t.columns[3].values[0]);
        assert_eq!(None, t.columns[3].values[1]);
        assert_eq!(Some("value 3.4"), t.columns[3].values[2]);
    }

    #[test]
    fn parse_valid_table_no_header() {
        let content = "| value 1.1 | a value 1.2 | a long value 1.3 | a long long value 1.4 |
| value 2.1 | value 2.2   |                  |                       |
| value 3.1 | value 3.2   |    value 3.3     |             value 3.4 |
";

        let t = parse(content).unwrap();
        assert_eq!(4, t.columns.len(), "number of columns");

        assert_eq!(None, t.columns[0].name);
        assert_eq!(None, t.columns[1].name);
        assert_eq!(None, t.columns[2].name);
        assert_eq!(None, t.columns[3].name);

        assert_eq!(None, t.columns[0].alignment);
        assert_eq!(None, t.columns[1].alignment);
        assert_eq!(None, t.columns[2].alignment);
        assert_eq!(None, t.columns[3].alignment);

        assert_eq!(3, t.columns[0].values.len());
        assert_eq!(Some("value 1.1"), t.columns[0].values[0]);
        assert_eq!(Some("value 2.1"), t.columns[0].values[1]);
        assert_eq!(Some("value 3.1"), t.columns[0].values[2]);

        assert_eq!(3, t.columns[1].values.len());
        assert_eq!(Some("a value 1.2"), t.columns[1].values[0]);
        assert_eq!(Some("value 2.2"), t.columns[1].values[1]);
        assert_eq!(Some("value 3.2"), t.columns[1].values[2]);

        assert_eq!(3, t.columns[2].values.len());
        assert_eq!(Some("a long value 1.3"), t.columns[2].values[0]);
        assert_eq!(None, t.columns[2].values[1]);
        assert_eq!(Some("value 3.3"), t.columns[2].values[2]);

        assert_eq!(3, t.columns[3].values.len());
        assert_eq!(Some("a long long value 1.4"), t.columns[3].values[0]);
        assert_eq!(None, t.columns[3].values[1]);
        assert_eq!(Some("value 3.4"), t.columns[3].values[2]);
    }

    #[test]
    fn parse_column_with_text() {
        assert_eq!(
            (Some("column 1"), "| column 2 |"),
            parse_column(" column 1 | column 2 |").unwrap()
        )
    }

    #[test]
    fn parse_column_no_text() {
        assert_eq!(
            (None, "| column 2 |"),
            parse_column("| column 2 |").unwrap()
        )
    }

    #[test]
    fn skip_spaces_when_no_spaces() {
        assert_eq!("no space", skip_spaces("no space"))
    }

    #[test]
    fn skip_spaces_when_spaces_in_front() {
        assert_eq!("with space", skip_spaces("   with space"))
    }

    #[test]
    fn skip_spaces_when_spaces_in_back() {
        assert_eq!("with space", skip_spaces("   with space   "))
    }

    #[test]
    fn read_until_empty_string() {
        assert_eq!((None, ""), read_until("", 'x'))
    }

    #[test]
    fn read_until_no_match() {
        assert_eq!((None, "this is a test"), read_until("this is a test", 'x'))
    }

    #[test]
    fn read_until_fullmatch() {
        assert_eq!(
            (Some("this is a test"), "x"),
            read_until("this is a testx", 'x')
        )
    }

    #[test]
    fn next_empty_string() {
        assert_eq!((None, ""), next(""))
    }

    #[test]
    fn next_non_empty_string() {
        assert_eq!((Some('t'), "his is a test"), next("this is a test"))
    }
}
