fn main() {
    let logical: bool = true;

    let a_float: f64 = 1.0;

    let an_integer = 5i32; // Suffix annotation

    let default_float = 3.0; // f64
    let default_integer = 7; // i32

    let mut inferred_type = 12; // Type i164 is inferred from another line
    inferred_type = 4294978697i64;

    let mut mutable = 12; // Mutable i32
    mutable = 21;

    // mutable = true; // Error! The type of a variable can't be changed.

    // Variables can be overwritten with shadowing.
    let mutable = true;
}