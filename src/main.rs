use std::fs::File;
use std::io::{BufRead, BufReader, Lines};

/// Intermediary structure for parsed tsv data
#[derive(Debug)]
struct Table {
    name: String,
    header: Vec<String>,
    rows: Vec<Vec<String>>,
}

/// ## Source file structure
/// - first line is info
/// - each table starts with `%T`
/// - each header starts with `%F`
/// - each row starts with `%R`
/// - file ends with `%E`
///
/// ## Example
/// |ERMHDR|19.12       |2024-03-15  |Project     |user        |user_name   |dbxDatabaseNoName|Project Management|EUR|
/// |------|------------|------------|------------|------------|------------|-----------------|------------------|---|
/// |%T    |`TABLE1`    |            |            |            |            |                 |                  |   |
/// |%F    |`column_1`  |`column_2`  |`column_3`  |            |            |                 |                  |   |
/// |%R    |1           |2           |€           |            |            |                 |                  |   |
/// |%R    |10          |2           |$           |            |            |                 |                  |   |
/// |%R    |11          |2           |A$          |            |            |                 |                  |   |
/// |%R    |13          |2           |R$          |            |            |                 |                  |   |
/// |%T    |`TABLE2`    |            |            |            |            |                 |                  |   |
/// |%F    |`column_1`  |`column_2`  |`column_3`  |`column_4`  |            |                 |                  |   |
/// |%R    |11          |20005       |VAC         |Vacation    |            |                 |                  |   |
/// |%R    |12          |4           |JURY        |Jury Duty   |            |                 |                  |   |
/// |%R    |13          |3           |HOL         |Holiday     |            |                 |                  |   |
/// |%T    |`TABLE3`    |            |            |            |            |                 |                  |   |
/// |%F    |`column_1`  |`column_2`  |`column_3`  |`column_4`  |`column_5`  |                 |                  |   |
/// |%R    |565         |            |            |0           |Enterprise  |                 |                  |   |
/// |%E    |            |            |            |            |            |                 |                  |   |
struct TableIterator {
    iter: Lines<BufReader<File>>,
}

impl Iterator for TableIterator {
    type Item = Table;

    fn next(&mut self) -> Option<Self::Item> {
        let mut line = self
            .iter
            .by_ref()
            .map(Result::unwrap)
            .skip_while(|line| !line.starts_with("%T")); // skip lines until first table starts

        // Prepare data to construct Table
        #[rustfmt::skip]
        let table_name: String = line
            .nth(0)?
            .split('\t')
            .nth(1)
            .unwrap()
            .into();

        let table_header: Vec<String> = line
            .next()?
            .split('\t')
            .skip(1)
            .map(|col| col.to_string())
            .collect();

        let table_rows: Vec<Vec<String>> = line
            .take_while(|line| line.starts_with("%R"))
            .map(|line| {
                line.split('\t')
                    .skip(1)
                    .map(|col| col.to_string())
                    .collect()
            })
            .collect();

        Some(Table {
            name: table_name,
            header: table_header,
            rows: table_rows,
        })
    }
}

fn main() {
    // WARN: assume file encoding is UTF-8, in case of non-UTF-8 file use `iconv` in advance
    //                               > iconv -f cp1251 -t utf-8 `input.xer` -o `schedule.xer`
    // TODO: is it possible to use `iconv` here?
    let filepath = "./data/schedule.xer";
    let file = File::open(filepath).unwrap();
    let reader = BufReader::new(file);
    let iter = TableIterator {
        iter: reader.lines(),
    };

    //Demo printing
    for table in iter {
        println!(
            "{:>15} {:>3} columns {:>6} rows",
            table.name,
            table.header.len(),
            table.rows.len() // TODO: .size_hint 0-120_000
        );
    }
}
