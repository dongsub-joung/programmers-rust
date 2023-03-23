fn solution(n: i32)-> i32 {
    let mut answer= 0;
    for i in 1..n+1{
       if n%i == 0{
           answer= answer+i;
       }
    }
    return answer;
}

#[cfg(test)]
mod tests{
    use super::*;

    #[test]
    fn testing1(){
        let n= 12; 	
        let result= 28;
        assert_eq!(result, solution(n));
    }

    #[test]
    fn testing2(){
        let n= 5;
        let result= 6;
        assert_eq!(result, solution(n));
    }
}

