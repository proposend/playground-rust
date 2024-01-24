fn main() {
    //Storing UTF-8 Encoded Text with Strings
    // Creating a New String
    // {
    //     let mut s = String::new();
    //     let data = "initial contents";
    //     let s = data.to_string();
    //     let s = "initial contents".to_string();

    //     let s = String::from("initial contents");
    // }
    // strings are UTF-8 encoded
    // {
    //     let hello = String::from("السلام عليكم");
    //     let hello = String::from("Dobrý den");
    //     let hello = String::from("Hello");
    //     let hello = String::from("שָׁלוֹם");
    //     let hello = String::from("नमस्ते");
    //     let hello = String::from("こんにちは");
    //     let hello = String::from("안녕하세요");
    //     let hello = String::from("你好");
    //     let hello = String::from("Olá");
    //     let hello = String::from("Здравствуйте");
    //     let hello = String::from("Hola");
    // }
    // Updating a String
    // {
    //     let mut s = String::from("foo");
    //     s.push_str("bar");

    //     println!("{s}");

    //     let mut s1 = String::from("foo");
    //     let s2 = "bar";
    //     s1.push_str(s2);
    //     println!("s2 is {s2}");

    //     let mut s = String::from("lo");
    //     s.push('l');
    //     println!("{s}");

    //     let s1 = String::from("Hello, ");
    //     let s2 = String::from("World!");
    //     let s3 = s1 + &s2;
    //     println!("s3: {s3}");

    //     let s1 = String::from("tic");
    //     let s2 = String::from("tac");
    //     let s3 = String::from("toe");

    //     let s = s1 + "-" + &s2 + "-" + &s3;
    //     println!("s: {s}");

    //     let s1 = String::from("tic");
    //     let s2 = String::from("tac");
    //     let s3 = String::from("toe");

    //     let s = format!("{s1}-{s2}-{s3}");
    //     println!("s: {s}");
    // }
    // Indexing into Strings
    // {
    //     let s1 = String::from("hello");
    //     let h = s1[0];
    //     // Internal Representation
    //     let hello = String::from("hello");
    // }
    // Slicing Strings
    // {
    //     let hello = "Здравствуйте";
    //     let s = &hello[0..4];
    //     println!("s: {s}");
    // }
    // Methods for Iterating Over Strings
    {
        for c in "Зд".chars() {
            println!("{c}");
        }

        for b in "Зд".bytes() {
            println!("{b}");
        }
    }
}
