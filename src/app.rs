use std::io::Write;

use crate::{command::Command, repl::InputBuffer, statement::Statement, table::TableSchema};

pub struct App {
    input_buffer: InputBuffer,
    tables: Vec<Box<dyn TableSchema>>,
}

impl App {
    pub fn new() -> Self {
        Self {
            input_buffer: InputBuffer::new(),
        }
    }

    pub fn add_table(&mut self, table: dyn TableSchema) {
        self.tables.push(table);
    }

    pub fn run(&mut self) -> ! {
        loop {
            Self::print_prompt();
            if let Err(err) = self.input_buffer.read_input() {
                eprintln!("{}", err);
            }
            if self.input_buffer.get_buffer().chars().nth(0) == Some('.') {
                match Command::new(&self.input_buffer.get_buffer()) {
                    Ok(command) => command.execute(),
                    Err(err) => {
                        eprintln!("{}", err);
                    }
                }
            }
            match Statement::new(&self.input_buffer.get_buffer()) {
                Ok(statement) => {
                    statement.execute();
                    println!("Executed!");
                }
                Err(err) => {
                    eprintln!("{}", err);
                }
            }
        }
    }

    fn print_prompt() {
        print!("db > ");
        let _ = std::io::stdout().flush();
    }
}
