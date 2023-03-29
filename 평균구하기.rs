pub fn solution(list: Vec<usize>) -> f64 {
    let mut answer= 0f64;
    let len= list.len() as f64;
    let sum: usize= list.iter().sum();
    answer= (sum as f64) / len;

    return answer;
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works() {
        let result = solution(vec!{1,2,3,4});
        assert_eq!(result, 2.5);
    }


    #[test]
    fn it_works2() {
        let result = solution(vec!{5, 5});
        assert_eq!(result, 5f64);
    }
}
