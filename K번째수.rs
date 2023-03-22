fn main(){
    
}

fn init(vec_list: Vec<usize>, two_dimention: Vec<Vec<usize>>) -> Vec<usize>{
    let mut result_v= Vec::new();

    for vec1 in two_dimention{
        let i= vec1[0].clone();
        let j= vec1[1].clone();
        let k= vec1[2].clone();

        let v_slice= &vec1[i..j]; 
        
        //v_slice.sort();

        let k_number= v_slice[k];

        result_v.push(k_number);
    }
    return result_v;
}


#[cfg(test)]
mod tests{
    use super::*;

    #[test]
    fn test1(){
        let v= vec![1usize, 5, 2, 6, 3, 7, 4];
        let vec_list= vec![vec![2usize, 5, 3], vec![4usize, 4, 1], vec![1usize, 7, 3]];
        let result= vec![5usize, 6, 3];

        assert_eq!(init(v, vec_list), result); 
    }
}
