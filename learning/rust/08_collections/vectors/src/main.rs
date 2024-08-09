fn main() {
    let mut v_mut = Vec::new();
    
    v_mut.push(1);
    v_mut.push(2);
    v_mut.push(3);
    
    let v = vec![1, 2, 3, 4, 5];
    let third: &i32 = &v[2];
    println!("The third element is {third}");
    
    let third: Option<&i32> = v.get(2);
    match third {
        Some(third) => println!("The third element is {third}"),
        None => println!("There are no third element.")
    }
    
    let new_v = vec![1,2,3,4,5];
    // let does_not_exist = &v[100];
    let does_not_exist = v.get(100);

    let mut v = vec![1, 2, 3, 4, 5];
    let first = &v[0];
    // v.push(6);  impossible due to possible reallocation of memory
    // println!("The first element is: {first}");
    
    let v = vec![100, 32, 57];
    for i in &v {
        println!("{i}")
    }
    
    let mut v = vec![100, 32, 57];
    for i in &mut v {
        *i += 50;
        println!("{i}")
    }
}

