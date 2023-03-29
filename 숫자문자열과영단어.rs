pub fn solution(s: String) -> usize {
    let mut anwser= 0uisze;
    
    // ascii code unpper case
    // 60~7    0 to 9
    // 141~172 a to z

    // match
//     0 	zero
// 1 one
// 2 two
// 3 three
// 4 four
// 5 five
// 6 six
// 7 seven
// 8 eight
// 9 nine

    anwser= s.parse::<usize>().unwrap();

    return anwser;
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test1() {
        let result = solution(String::from("one4seveneight"));
        assert_eq!(result, 1478);
    }

    #[test]
    fn test1() {
        let result = solution(String::from("23four5six7"));
        assert_eq!(result, 234567);
    }

    #[test]
    fn test1() {
        let result = solution(String::from("2three45sixseven"));
        assert_eq!(result, 234567);
    }

    #[test]
    fn test1() {
        let result = solution(String::from("123"));
        assert_eq!(result, 123);
    }
}
