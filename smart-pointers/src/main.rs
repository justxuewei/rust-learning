use smart_pointers::{box_pointer, drop_trait};
use smart_pointers::rc_pointer;

fn main() {
    box_pointer::cons_list_test();
    drop_trait::drop_trait_test();
    drop_trait::drop_manually_test();
    rc_pointer::rc_pointer_test();
    rc_pointer::ref_count_test();
}
