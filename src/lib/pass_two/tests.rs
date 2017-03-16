#[cfg(test)]
mod tests {
    //    use super::*;

    #[test]
    #[should_panic]
    fn it_panics() {
        println!("I'll die now");
        panic!("aaahh");
    }
}
