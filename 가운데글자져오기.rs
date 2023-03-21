fn main(){
    solution(String::from("abcde"));
}

fn solution(strs: String) -> String{
    let char_vec: Vec<char>= strs.chars().collect();
    let len= &char_vec.len() / 2;
    let not_odd= &char_vec.len() % 2;

    println!("{}", len);
    if not_odd != 0 {
        return format!("{}", char_vec[len]);
    }else{
        return format!("{}{}",char_vec[len-1], char_vec[len]);
    }
}


#[cfg(test)]
mod tests{
    use super::*;

    #[test]
    fn test1(){
        let EX1= String::from("abcde");
        let result= solution(EX1);
        assert_eq!(result, "c");
    }

    #[test]
    fn test2(){
        let EX2= String::from("qwer");
        let result= solution(EX2);
        assert_eq!(result, "we");
    }
}
