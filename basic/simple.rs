fn main() {
    let const_var = 1;
    //const_var = 2;   // err
    
    let mut mut_var = 1;
    mut_var = 2;

    println!("CONST: {}, MUTABLE: {}", const_var, mut_var);
}
