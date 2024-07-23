mod composite_data_types;
mod control_structures;
mod data_types;
mod structs;

use structs::{closure, generics, poo_structs, trait_structs};

use composite_data_types::{enums_type, slices_type, structs_type};

use control_structures::{for_structure, if_structure, loops, match_structure};

use data_types::non_primitives;
use data_types::primitives;

use personal_lib::errors::functions;

fn main() {
    primitives::show_primitive_types();
    non_primitives::show_non_primitive_types();

    for_structure::show_for_result();
    if_structure::show_if_result();
    loops::show_loop_result();
    loops::show_while_result();
    match_structure::show_match_result();

    enums_type::show_enums_result();
    structs_type::show_structs_result();
    slices_type::show_slices_result();

    trait_structs::show_trait_result();
    generics::show_generic_result();
    closure::show_closure_result();
    closure::show_closure_ownership_result();
    poo_structs::show_poo_result();

    scope();

    functions::show_result_result();
    functions::show_option_result();
}

fn scope() {
    let mut original_num = 2;
    let local_copy = original_num;
    capture_scope(local_copy);
    borrow_scope(&local_copy);
    borrow_scope_to_modify(&mut original_num);
    println!("Original scope {}", local_copy);
    println!("Original scope of copy{}", original_num);
}

fn capture_scope(num: i32) {
    let mut copy = num;
    println!("I have the copy {} and the original {}", copy, num);
    copy = num * num * num;
    println!("I have the copy {} and the original {}", copy, num);
}

fn borrow_scope(num: &i32) {
    let square = *num * *num;
    println!("I have the square {} and the original {}", square, *num);
}

fn borrow_scope_to_modify(num: &mut i32) {
    *num -= 1;
    println!("I have the value and I modified it {}", *num);
}
