use crate::parser;

#[derive(Debug)]
pub struct Error(String);

impl std::fmt::Display for Error {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        writeln!(f, "failed to parse table: {}", self.0)
    }
}

impl std::error::Error for Error {}

impl std::convert::From<String> for Error {
    fn from(msg: String) -> Self {
        Self(msg)
    }
}

impl std::convert::From<std::io::Error> for Error {
    fn from(err: std::io::Error) -> Self {
        Self(err.to_string())
    }
}

impl std::convert::From<Error> for std::io::Error {
    fn from(err: Error) -> Self {
        std::io::Error::new(std::io::ErrorKind::InvalidInput, err)
    }
}

#[derive(Debug, Copy, Clone, Eq, PartialEq)]
pub enum Alignment {
    Left,
    Right,
    Centered,
}

#[derive(Debug, Eq, PartialEq)]
pub struct Column<'a> {
    pub name: Option<&'a str>,
    pub alignment: Option<Alignment>,
    pub largest_value: usize,
    pub values: Vec<Option<&'a str>>,
}

#[derive(Debug, Eq, PartialEq, Default)]
pub struct Table<'a> {
    current_row: usize,
    current_column: usize,
    pub no_headers: bool,
    pub columns: Vec<Column<'a>>,
}

impl<'a> Table<'a> {
    pub fn len(&self) -> usize {
        self.current_row + 1
    }

    pub fn is_empty(&self) -> bool {
        self.len() == 0
    }

    pub fn push(&mut self, value: Option<&'a str>) {
        if self.current_column == self.columns.len() {
            self.columns.push(Column {
                largest_value: 0,
                name: None,
                alignment: None,
                values: vec![None; self.current_row],
            });
        }
        let largest = self.columns[self.current_column]
            .largest_value
            .max(value.unwrap_or_default().len());
        self.columns[self.current_column].largest_value = largest;
        self.columns[self.current_column].values.push(value);
        self.current_column += 1;
    }

    pub fn next_row(&mut self) {
        while self.current_column != self.columns.len() {
            self.columns[self.current_column].values.push(None);
            self.current_column += 1;
        }
        self.current_row += 1;
        self.current_column = 0;
    }

    pub fn add_column(&mut self, name: Option<&'a str>, alignment: Option<Alignment>) {
        self.columns.push(Column {
            largest_value: name.map(|n| n.len()).unwrap_or_default(),
            name,
            alignment,
            values: Vec::new(),
        })
    }
}

impl<'a> std::convert::TryFrom<&'a str> for Table<'a> {
    type Error = Error;

    fn try_from(content: &'a str) -> Result<Self, Self::Error> {
        parser::parse(content)
    }
}

#[cfg(test)]
mod tests {

    use super::*;

    #[test]
    fn add_column_existing_lines() {
        let mut actual = Table::default();
        actual.add_column(Some("column 1"), None);
        actual.add_column(Some("column 2"), Some(Alignment::Left));
        actual.push(Some("value 1.1"));
        actual.push(Some("value 1.2"));
        actual.next_row();
        actual.push(Some("value 2.1"));
        actual.push(Some("value 2.2"));

        let expected = vec![
            Column {
                largest_value: 9,
                name: Some("column 1"),
                alignment: None,
                values: vec![Some("value 1.1"), Some("value 2.1")],
            },
            Column {
                largest_value: 9,
                name: Some("column 2"),
                alignment: Some(Alignment::Left),
                values: vec![Some("value 1.2"), Some("value 2.2")],
            },
        ];

        assert_eq!(expected, actual.columns)
    }

    #[test]
    fn add_column_missing_columns() {
        let mut actual = Table::default();
        actual.add_column(Some("column 1"), None);
        actual.add_column(Some("column 2"), Some(Alignment::Left));
        actual.push(Some("value 1.1"));
        actual.push(Some("value 1.2"));
        actual.next_row();
        actual.push(Some("value 2.1"));
        actual.push(Some("value 2.2"));
        actual.push(Some("value 2.3"));
        actual.next_row();
        actual.push(Some("value 3.1"));
        actual.next_row();
        actual.push(Some("value 4.1"));
        actual.push(Some("value 4.2"));
        actual.push(Some("value 4.3"));

        let expected = vec![
            Column {
                largest_value: 9,
                name: Some("column 1"),
                alignment: None,
                values: vec![
                    Some("value 1.1"),
                    Some("value 2.1"),
                    Some("value 3.1"),
                    Some("value 4.1"),
                ],
            },
            Column {
                largest_value: 9,
                name: Some("column 2"),
                alignment: Some(Alignment::Left),
                values: vec![
                    Some("value 1.2"),
                    Some("value 2.2"),
                    None,
                    Some("value 4.2"),
                ],
            },
            Column {
                largest_value: 9,
                name: None,
                alignment: None,
                values: vec![None, Some("value 2.3"), None, Some("value 4.3")],
            },
        ];

        assert_eq!(expected, actual.columns)
    }
}
