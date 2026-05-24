mod cell;
mod vectors;

fn main() {
    vectors::create_vector();
    vectors::create_vector_with_macro();
    vectors::update_vector();
    vectors::read_vector_value();
    vectors::read_vector_value_index_out_of_bounds();
    vectors::borrowing_error();
    vectors::iterate_vector_values();
    vectors::use_vector_to_store_different_types_of_data();

    // Comment implémenter Vec: https://doc.rust-lang.org/nomicon/vec/vec.html
}
