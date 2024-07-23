// #[]

//Derivation of traits//
#[derive(Debug, Clone, PartialEq)] // Derives the traits Debug, Clone, and PartialEq for the structure
struct Person {
    name: String,
    age: u32,
}

//Module import//
#[path = "other_module.rs"] // Imports a module located in another file
mod other_module;

//Linking C functions//
#[link(name = "mylibrary")] // Links a C library
extern "C" {
    fn c_function();
}

//Suppressing warnings//
#[allow(dead_code)] // Suppresses the warning for unused code
fn unused_function() {
    // Unused code
}

//Marking functions as unreachable//
#[cfg(target_os = "linux")] // Marks the function as available only on Linux operating systems
fn linux_only_function() {
    // Linux-specific code
}

//Specifying stack size//
#[cfg_attr(target_arch = "x86_64", repr(align(16)))] // Specifies the alignment of the structure
struct SomethingWithAlignment {
    // Structure fields
}

//Annotations for unit tests//
#[test] // Marks the function as a unit test
fn test_my_function() {
    // Unit test code
}
