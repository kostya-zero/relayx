use tabled::{Table, Tabled};
use tabled::settings::object::{Columns, Rows};
use tabled::settings::{Format, Modify, Remove, Style};

#[derive(Tabled)]
pub struct TableEntry {
    pub name: String,
    pub description: String,
}

pub fn print_table(entries: Vec<TableEntry>) -> String {
    let mut table = Table::new(entries);
    table.with(Remove::row(Rows::first()));
    table.with(
        Modify::new(Columns::first())
            .with(Format::content(|s| format!("\x1b[1m{s}\x1b[0m"))),
    );
    table.with(
        Modify::new(Columns::one(1))
            .with(Format::content(|s| format!("\x1b[38;5;250m{s}\x1b[0m"))),
    );
    table.with(Style::blank());
    table.to_string()
}