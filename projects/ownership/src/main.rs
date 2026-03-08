fn main() {
    {
        let mut s: String = String::from("Hello, world from string s!");
        let slice: &mut str = &mut s[0..5];

        let start_index: usize = 0;
        let end_index: usize = 5;
        let slice2 : &mut str = &mut s[start_index..end_index];

        // println!("Slices: {}", slice);
        println!("slice2: {}", slice2);

        let s1: &str = "Hello world from literal string slice.";
        println!("s1: {}", s1);
    }
}

fn clear_string(s: &str) {
    println!("Slice: {}", s);
}
