fn main() {

    //empty vector type annotation needed as the compiler can't infer the type
    let mut v: Vec<i32> = Vec::new(); 
    
    //adding elements
    v.push(5); 
    v.push(6);
    v.push(7);
    v.push(8);
    
    // interesting enough the compiler assigns the option type
    let third = v.get(2); 
    
    // Being a maybe result I would need to strip the value
    println!("{:?}", third); 
    
    // using unwrap is unsafe the compiler will throw an error at run time if a None type is met
    println!("{}", third.unwrap());  
    
    //the get method with the index passed as an argument, we get an Option<&T> that we can use with match.
    let fifth = v.get(5);
 
    match fifth { //using match makes the print operation safe
        Some(fifth) => println!("The fifth element is {fifth}"),
        None => println!("This vector has no fifth element")
    }

    // creating a vector and filling it at the same time
    let u = vec![1, 2, 3, 4, 5];

    // A double reference,  &u gives us a reference to the element at the index value which is of type i32
    // This &u reference will mean an index out of bound error will be thrown if a too large index is called
    let fifth: &i32 = &u[4];

    println!("The fifth element of u is {}", fifth); 

    let w = vec![100, 32, 57];
    for i in &w {
        println!("{i}");
    }

    // adding to every element in a list 
    // To change the value that the mutable reference refers to, 
    // we have to use the * dereference operator to get to the value in i before we can use the += operator
    let mut x = vec![100, 32, 57];
    for i in &mut x {
        *i += 50;
    }
    for i in &x {
        println!("{i}");
    }

    // vectors can only have elements of the same type 
    // using enums allows the use of multiple types 
    #[derive(Debug)]
    enum SpreadsheetCell {
        Int(i32),
        Float(f64),
        Text(String),
    }

    let row = vec![
        SpreadsheetCell::Int(3),
        SpreadsheetCell::Text(String::from("blue")),
        SpreadsheetCell::Float(10.12),
    ];

    println!("The first element is {:?} the last element is {:?}",&row[0], &row[2])



}
