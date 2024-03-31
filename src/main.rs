mod data_types;

use data_types::non_primitives;
use data_types::primitives;

fn main() {
    primitives::show_primitive_types();
    non_primitives::show_non_primitive_types();
}
