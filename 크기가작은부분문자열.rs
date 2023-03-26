pub fn solution(t: String, p: String) -> usize {
    let mut answer= 0usize;

    let p_size= p.len();
    let chars: Vec<_>= t.chars().collect();

    let mut string_list: Vec<char>= Vec::new();

    for i in 0..chars.len(){
        for j in i+1..p_size{
            string_list.push(chars[j]);
        }

        let line= p.parse::<usize>().unwrap();
        let string_list: String= string_list.to_vec()
            .into_iter()
            .collect();
        let number= string_list.parse::<usize>().unwrap();

        if line < number {
            answer= answer+1;
        }
    }


    return answer;
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test1() {
        let t= String::from("3141592");
        let p= String::from("271");

        let result = solution(t,p);
        assert_eq!(result, 2);
    }

    #[test]
    fn test2() {
        let t= String::from("500220839878");
        let p= String::from("7");

        let result = solution(t,p);
        assert_eq!(result, 8);
    }

    #[test]
    fn test3() {
        let t= String::from("10203");
        let p= String::from("15");

        let result = solution(t,p);
        assert_eq!(result, 3);
    }
}
