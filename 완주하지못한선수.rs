use std::{collections::HashMap, ptr::eq};

fn solution(parti: Vec<String>, competi: Vec<String>) -> String{
    let mut parti_hash: HashMap<String, i32>= HashMap::new();
    let mut competi_hash: HashMap<String, i32>= HashMap::new();
    let mut anwser= String::new();

    for value in parti.clone(){
        parti_hash.insert(value, 1); 
    }

    for value in competi.clone(){
        competi_hash.insert(value, 1);
    }
   
    for value in parti.clone(){
        let (ck, cv)= competi_hash.get_key_value(&value).unwrap_or_else(
            || (&value, &1)
            );

        if eq(ck, &value) {
            anwser= ck.clone();
        }
    }

    return anwser;
}


//fn solution2(parti: Vec<String>, competi: Vec<String>) -> String{
//}

#[cfg(test)]
mod tests{
    use super::*;

    #[test]
    fn test_all(){
        let participant= vec!{String::from("leo"), String::from("kiki"), String::from("eden")};
        let completion= vec!{String::from("eden"), String::from("kiki")};
        let anwser= String::from("leo");

        assert_eq!(anwser, solution(participant, completion));
    }
}
