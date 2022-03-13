use common_collections::vector;
use common_collections::string;
use common_collections::hash_map;

fn main() {
    vector::vector_test();
    vector::vector_borrowing_test();
    vector::vector_iteration_test();
    vector::enum_store_multiple_types_test();

    string::new_string_test();
    string::push_pop_test();
    string::concatenate_test();
    string::string_index_test();
    string::iterate_string_test();

    hash_map::new_hash_map_test();
    hash_map::iterate_hash_map_test();
    hash_map::update_test();
}
