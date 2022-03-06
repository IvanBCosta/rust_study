#[cfg(test)]
mod tests {
    use crate::main;
    use super::*;

    #[test]
    fn it_works() {
        main();
        assert_eq!(2 + 2, 4);
    }
}