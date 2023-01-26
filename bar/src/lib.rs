pub fn add(left: usize, right: usize) -> usize {
    left + right
}

pub fn compiles() {
   					 panic!("Eu nao compilo, me recuso.");
    println!("I compile.");
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works() {
        let result = add(2, 2);
        assert_eq!(result, 4);
        panic!("Eu nao compilo, me recuso.")
    }
}
