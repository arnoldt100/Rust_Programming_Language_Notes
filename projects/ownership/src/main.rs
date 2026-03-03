fn main() {
    {
        let mut s1 = String::from("Hello, world from string s1!");
	    let length = calculate_lengths(&s1);
	    println!("The length of s1 is: {}", length);

        change(&mut s1);
        let length = calculate_lengths(&s1);
        println!("The length of s1 is: {}", length);
    }
}

fn calculate_lengths(s: &String) -> usize {
	s.len()
}

fn change(some_string: &mut String) {
    some_string.push_str(", world");
}