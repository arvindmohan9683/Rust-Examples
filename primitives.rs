fn main()
{
    let boolvar: bool:true;
    let floatvar: f64 = 4.2; //64 bit
    let intvar = 42i32; //32 bit (suffix notation)

    let default_float   = 3.0; // `f64` by default
    let default_integer = 7;   // `i32` by default


    let mut inferred_type = 12; // A type can also be inferred from context
    inferred_type = 4294967296i64; //  => i64 inferred form here

    let mut mutable = 12; // Mutable `i32`
    mutable = 21; //can change value but not type


    let mutable = true; // Variables can be overwritten with shadowing.    
}
