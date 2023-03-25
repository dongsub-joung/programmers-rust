fn solution(n: usize) -> String{
    let mut anwser= String::new();
    if n % 2 == 0 {
        anwser= String::from("Even")
    }else {
        anwser= String::from("Odd");
    }

    return anwser;
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn odd() {
        let expect = String::from("Odd");
        let result= solution(3);
        assert_eq!(result, expect);
    }

    #[test]
    fn even(){
        let expect = String::from("Even");
        let result= solution(4);
        assert_eq!(result, expect);
    }
}
