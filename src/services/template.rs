use std::fs;

pub fn get_close_this_template() -> Result<String, &'static str> {
    fs::read_to_string("./src/templates/close_this.html")
        .map_err(|_| "Error reading 'close this' template")
}