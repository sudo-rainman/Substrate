use std::io::stdin;
fn main() {

    let to_slice = String::from("This is a regular paragraph with the default style of Normal. This is a regular paragraph with the default style of Normal. This is a regular paragraph with the default style of Normal. This is a regular paragraph with the default style of Normal. This is a regular paragraph with the default style of Normal.");


    let mut substr = String::new();
    stdin().read_line(&mut substr).ok().expect("Failed to read line");
    
    // pub fn contains<'a, P>(&'a self, pat: P) -> bool
    substr.truncate(substr.len()-1);
    let mut i = 0;
    let mut total = 0;

    while i < to_slice.len()- substr.len() {
        let s = &to_slice[i..(i+substr.len())];
        println!("{}",s);
        if s.contains(&substr) {
            total += 1;
            i+=1;
        }
        i += 1;
        
    }
    println!("{}",total);

   
}
