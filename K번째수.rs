fn main(){
    let v= vec![1usize, 5, 2, 6, 3, 7, 4];
    let vec_list= vec![vec![2usize, 5, 3], vec![4usize, 4, 1], vec![1usize, 7, 3]];
    let result= init(v, vec_list);

}

fn init(vec_list: Vec<usize>, two_dimention: Vec<Vec<usize>>) -> Vec<usize>{
    let mut result_v= Vec::new();

    for mut vec1 in two_dimention{
        let i= vec1[0].clone()-1;
        let j= vec1[1].clone();
        let k= vec1[2].clone()-1;

        let mut v_slice= vec_from_slice(vec_list.clone(), i, j);
        
        v_slice.sort();

        let k_number= v_slice[k];

        result_v.push(k_number);
    }
    return result_v;
}

fn vec_from_slice(vec_list: Vec<usize>, i: usize, j: usize) -> Vec<usize>{
    vec_list[i..j].to_vec()
}

#[cfg(test)]
mod tests{
    use super::*;

    #[test]
    fn test1(){
        let v= vec![1usize, 5, 2, 6, 3, 7, 4];
    
        let expect= vec![2,6,3];
        assert_eq!(expect, vec_from_slice(v, 2, 5))
    }

    #[test]
    fn holeTest(){
        let v= vec![1usize, 5, 2, 6, 3, 7, 4];
        let vec_list= vec![vec![2usize, 5, 3], vec![4usize, 4, 1], vec![1usize, 7, 3]];
        let result= vec![5usize, 6, 3];

        assert_eq!(init(v, vec_list), result); 
    }
}
