pub fn get_newline_style(input: &'static str) -> &str {
    let carriage_return = input.find('\r');
    let line_field = input.find('\n');

    match (carriage_return, line_field) {
        (Some(_), None) => "\r",
        (None, Some(_)) => "\n",
        (Some(_), Some(_)) => "\r\n",
        (None, None) => panic!("invalid input : no line return"),
    }
}