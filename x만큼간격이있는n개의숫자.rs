pub fn solution(x: i64, n: i64) -> Vec<i64> {
    let mut anwser: Vec<i64>= Vec::new();
   
    for i in 1..n+1{
        anwser.push(i*x);
    }

    return anwser;
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works() {
        let result = solution(2, 5);
        assert_eq!(result, vec!{2,4,6,8,10});
    }
    
    #[test]
    fn test2(){
        let result= solution(4, 3);
        assert_eq!(result, vec!{4,8,12});
    }

    #[test]
    fn tset3(){
        let result= solution(-4, 2);
        assert_eq!(result, vec!{-4,-8});
    }
}
