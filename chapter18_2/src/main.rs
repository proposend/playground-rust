fn main() {
    {
        // `let` bindings require an "irrefutable pattern"
        // let Some(x) = Option::Some(3);
    }
    {
        if let Some(x) = Option::Some(3) {
            println!("{}", x);
        }

        // this pattern will always match, so the `if let` is useless
        if let x = 5 {
            println!("{}", x);
        }
    }
}
