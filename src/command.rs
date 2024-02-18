use comma::parse_command;

use crate::App;

pub fn command_parse(command: &str) -> Option<Vec<String>> {
    parse_command(command)
}
