use std::error::Error;
use error_handling::{panic, result};

fn main() -> Result<(), Box<dyn Error>> {
    // panic::do_panic();

    // result::open_file();
    // result::unwrap_expect_test();
    result::propagating_error_test();
    result::option_question_test();

    Ok(())
}
