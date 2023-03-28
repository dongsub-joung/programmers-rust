pub fn add(a: usize, b: usize, mut n: usize) -> usize {
    let mut answer= 0usize;
    
    while a <= n {
        answer= answer+n /a * b;
        n= (n/a*b) + (n % a);
    }
    return answer;
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works() {
        let result = add(2, 1, 20);
        assert_eq!(result, 19);
    }


    #[test]
    fn it_works2() {
        let result = add(3, 1, 20);
        assert_eq!(result, 9);
    }
}
