use crate::table;

pub struct Table<'a>(table::Table<'a>);

impl<'a> Table<'a> {
    fn display_header(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        if self.0.no_headers {
            return Ok(());
        }

        for column in self.0.columns.as_slice() {
            Table::display_column(
                f,
                column.name.unwrap_or_default(),
                column.alignment.unwrap_or(table::Alignment::Left),
                column.largest_value,
            )?;
        }
        writeln!(f, "|")?;
        for column in self.0.columns.as_slice() {
            Table::display_header_seperator_column(f, column.alignment, column.largest_value)?;
        }
        writeln!(f, "|")
    }

    fn display_rows(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        for row in 0..self.0.len() {
            for column in self.0.columns.as_slice() {
                Table::display_column(
                    f,
                    column.values[row].unwrap_or_default(),
                    column.alignment.unwrap_or(table::Alignment::Left),
                    column.largest_value,
                )?;
            }
            writeln!(f, "|")?;
        }

        Ok(())
    }

    fn display_column(
        f: &mut std::fmt::Formatter<'_>,
        value: &str,
        alignment: table::Alignment,
        width: usize,
    ) -> std::fmt::Result {
        match alignment {
            table::Alignment::Left => write!(f, "| {: <width$} ", value, width = width),
            table::Alignment::Right => write!(f, "| {: >width$} ", value, width = width),
            table::Alignment::Centered => {
                write!(f, "| {: ^width$} ", value, width = width)
            }
        }
    }

    fn display_header_seperator_column(
        f: &mut std::fmt::Formatter<'_>,
        alignment: Option<table::Alignment>,
        width: usize,
    ) -> std::fmt::Result {
        match alignment {
            None => write!(f, "|-{:-<width$}-", "-", width = width),
            Some(table::Alignment::Left) => write!(f, "|:{:-<width$}-", "-", width = width),
            Some(table::Alignment::Right) => write!(f, "|-{:-<width$}:", "-", width = width),
            Some(table::Alignment::Centered) => {
                write!(f, "|:{:-<width$}:", "-", width = width)
            }
        }
    }
}

impl<'a> std::fmt::Display for Table<'a> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        self.display_header(f)?;
        self.display_rows(f)
    }
}

impl<'a> std::convert::From<table::Table<'a>> for Table<'a> {
    fn from(t: table::Table<'a>) -> Self {
        Self(t)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::table;
    use std::io::Write;

    #[test]
    fn format_table_with_headers() {
        let mut parsed_table = table::Table::default();
        parsed_table.add_column(Some("column 1"), None);
        parsed_table.add_column(Some("column 2"), Some(table::Alignment::Left));
        parsed_table.add_column(Some("column 3"), Some(table::Alignment::Centered));
        parsed_table.add_column(Some("column 4"), Some(table::Alignment::Right));
        parsed_table.push(Some("value 1.1"));
        parsed_table.push(Some("a value 1.2"));
        parsed_table.push(Some("a long value 1.3"));
        parsed_table.push(Some("a long long value 1.4"));
        parsed_table.next_row();
        parsed_table.push(Some("value 2.1"));
        parsed_table.push(Some("value 2.2"));
        parsed_table.next_row();
        parsed_table.push(Some("value 3.1"));
        parsed_table.push(Some("value 3.2"));
        parsed_table.push(Some("value 3.3"));
        parsed_table.push(Some("value 3.4"));

        let t: Table = parsed_table.into();

        let mut output = std::io::BufWriter::new(Vec::new());
        write!(&mut output, "{}", t).expect("failed to generate markdown table");
        let content = String::from_utf8(output.into_inner().unwrap()).unwrap();

        assert_eq!(
            "| column 1  | column 2    |     column 3     |              column 4 |
|-----------|:------------|:----------------:|----------------------:|
| value 1.1 | a value 1.2 | a long value 1.3 | a long long value 1.4 |
| value 2.1 | value 2.2   |                  |                       |
| value 3.1 | value 3.2   |    value 3.3     |             value 3.4 |
",
            content
        );
    }

    #[test]
    fn format_table_no_headers() {
        let mut parsed_table = table::Table::default();
        parsed_table.no_headers = true;
        parsed_table.push(Some("value 1.1"));
        parsed_table.push(Some("a value 1.2"));
        parsed_table.push(Some("a long value 1.3"));
        parsed_table.push(Some("a long long value 1.4"));
        parsed_table.next_row();
        parsed_table.push(Some("value 2.1"));
        parsed_table.push(Some("value 2.2"));
        parsed_table.next_row();
        parsed_table.push(Some("value 3.1"));
        parsed_table.push(Some("value 3.2"));
        parsed_table.push(Some("value 3.3"));
        parsed_table.push(Some("value 3.4"));

        let t: Table = parsed_table.into();

        let mut output = std::io::BufWriter::new(Vec::new());
        write!(&mut output, "{}", t).expect("failed to generate markdown table");
        let content = String::from_utf8(output.into_inner().unwrap()).unwrap();

        assert_eq!(
            "| value 1.1 | a value 1.2 | a long value 1.3 | a long long value 1.4 |
| value 2.1 | value 2.2   |                  |                       |
| value 3.1 | value 3.2   | value 3.3        | value 3.4             |
",
            content
        );
    }
}
