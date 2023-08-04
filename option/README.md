Matching with Option<<T>>

using pattern with the Option enumeration for easy access to the wrapped value

and the if let construct:

    let mut count = 0;
    if let Coin::Quarter(state) = coin {
        println!("State quarter from {:?}!", state);
    } else {
        count += 1;
    }

