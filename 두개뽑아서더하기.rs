pub fn solution(numbers: Vec<usize>) -> Vec<usize> {
    let mut result_numbers: Vec<usize>= Vec::new();
    let numbers_copy= numbers.clone();

    for number in numbers{
        for j in 1..numbers_copy.len()-1{
            let added= number + numbers_copy[j];
            result_numbers.push(added); 
        }
    }

    // result_numbers.dedup();
    result_numbers.sort();

    return result_numbers;
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn testA() {
        let result = solution(vec!{2,1,3,4,1});
        assert_eq!(result, vec!{2,3,4,5,6,7});
    }

    fn testB() {
        let result = solution(vec!{5,0,2,7});
        assert_eq!(result, vec!{2,5,7,9,12});
    }
}
