fn main () {
    let s1 = "hello dolly".to_string();
    // rust does not see strings obj as copyable. 
    let s2 = s1;
    println!("s1 {}", s1);
}