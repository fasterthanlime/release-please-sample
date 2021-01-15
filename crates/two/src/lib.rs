#[cfg(test)]
mod tests {
    #[test]
    fn it_works() {
        assert_eq!(2 + 2, 4);
    }

    #[test]
    fn do_greet() {
        super::greet();
    }
}

pub fn greet() {
    println!("hello from two!");
    one::greet();
}
