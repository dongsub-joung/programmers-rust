
// not work
fn solution(t: String, p: String) -> usize {
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


fn solution2(t: String, p: String) -> usize {
    let mut answer= 0usize;

    let p_size= p.len();
    let p= p.parse::<usize>().unwrap();

    for i in 0..t.len()-p_size+1{
        let sliced= substring(t.clone(), i, i+p_size);
        if sliced <= p {
            answer= answer+1;
        }
    }
    return answer;
}

fn substring(t: String, frist_idx: usize, second_idx: usize) -> usize{
    let mut number= 0usize; 

    // let chars: Vec<_>= t.chars().collect();
    // let mut string_list: Vec<char>= Vec::new();

    let slice= &t[frist_idx..second_idx];

    number= slice.parse::<usize>().unwrap();

    return number;
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test1() {
        let t= String::from("3141592");
        let p= String::from("271");

        let result = solution2(t,p);
        assert_eq!(result, 2);
    }

    #[test]
    fn test2() {
        let t= String::from("500220839878");
        let p= String::from("7");

        let result = solution2(t,p);
        assert_eq!(result, 8);
    }

    #[test]
    fn test3() {
        let t= String::from("10203");
        let p= String::from("15");

        let result = solution2(t,p);
        assert_eq!(result, 3);
    }
}
