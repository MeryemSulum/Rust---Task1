fn main() {
    let string1 : &str = "Hello";
    let string2 : &str = " World";
    concatenate_strings(string1 , string2);
    
}
fn concatenate_strings( s1 : &str, s2 : &str ) -> String {
    let mut result = String:: from(s1) ;
    result.push_str(s2);
    println!("Result is {}", result);
    result
}
