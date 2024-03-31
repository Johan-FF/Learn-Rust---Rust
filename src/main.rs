mod composite_data_types;
mod control_structures;
mod data_types;

use composite_data_types::{enums_type, slices_type, structs_type};

// use control_structures::{for_structure, if_structure, loops, match_structure};

// use data_types::non_primitives;
// use data_types::primitives;

fn main() {
    // primitives::show_primitive_types();
    // non_primitives::show_non_primitive_types();

    // for_structure::show_for_result();
    // if_structure::show_if_result();
    // loops::show_loop_result();
    // loops::show_while_result();
    // match_structure::show_match_result();

    enums_type::show_enums_result();
    structs_type::show_structs_result();
    slices_type::show_slices_result();
}
