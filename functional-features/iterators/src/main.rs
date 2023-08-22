fn main() {
    let v1 = vec![1, 2, 3];

    //Note that we needed to make v1_iter mutable: 
    //calling the next method on an iterator changes internal state that the iterator uses to keep track of where it is in the sequence.
    let mut v1_iter = v1.iter();

    let first = v1_iter.next().unwrap_or_else( || &0);

    println!("{}", first);

    //assert_eq!(iterator_sum(v1),6);

    // collect transforms an iterator into a collection. This is necessary for the iteratot does not get evaluated until called up on
    let v2: Vec<_> = v1_iter.map(|x| x * 2).collect();

    // vi_iter is mutable, at this moment next has moved the iterator on to 2
    println!("{:?}", v2); 

    let v = vec![1, 2, 3, 4];

    let a: Vec<_> = v.iter().filter(|x: &&i32| *x % 2 == 0).map(|x: &i32| x * 2).collect();

    let b: Vec<_> = v.iter().map(|x: &i32| x * 2).filter(|x: &i32| x % 2 == 0).collect();

    assert_eq!(vec![4,8], a);
    assert_eq!(vec![2,4,6,8],b);

}

fn iterator_sum(vec:Vec<i32>) -> i32{

    let v1_iter = vec.iter();

    let total: i32 = v1_iter.sum(); //sum takes ownership over the iterator v1_iter

    total
}