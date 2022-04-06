use procedural_macros::Builder;

#[allow(dead_code)]
#[derive(Builder)]
pub struct Command {
    executable: String,
    args: Vec<String>,
    current_dir: Option<String>,
}

fn main() {

}
