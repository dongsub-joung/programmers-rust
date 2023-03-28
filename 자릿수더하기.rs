fn add(n: usize) -> usize {
    let list: Vec<usize>= divi(n);

    let mut anwser= 0usize;
    for n in list {
        anwser= anwser+n;
    }
    return anwser;
}

fn divi(n: usize) -> Vec<usize>{
    let mut list: Vec<usize>= Vec::new();
    let mut n= n;
    
    loop{
        let number= n % 10;
        list.push(number);
        n= n/10;
        if  n < 10 {
            list.push(n);
            break;
        }
    }

    return list;
}

mod tests {
    use super::*;

    #[test]
    fn unit_test() {
        let result = divi(123);
        assert_eq!(result, vec!{3,2,1});
    }

    #[test]
    fn it_works() {
        let result = add(123);
        assert_eq!(result, 6);
    }

    #[test]
    fn it_works2() {
        let result = add(987);
        assert_eq!(result, 24);
    }
}
