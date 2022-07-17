use std::{collections::HashMap};
use std::time::{Instant};

fn collatz(n: u64) -> u64{
    if n == 1{
        return 0;
    }
    
    match n%2 {
        0 => 1 + collatz(n/2),
        _ => 1 + collatz(3*n + 1),
    }
}

fn collatz2(n: u64, scores: &mut HashMap<u64, u64>) -> u64 {
    if n == 1{
        return 0;
    }

    let mut collatzval;
    let val = scores.get(&n);
        match val {
            Some(v) =>{*v},
            None => {
                match n%2 {
                    0 => {collatzval = 1 + collatz2(n/2,scores);scores.insert(n, collatzval); collatzval},
                    _ => {collatzval = 1+ collatz2(3*n + 1,scores); scores.insert(n, collatzval);collatzval},
                }
            }
        }
}

fn main2(x:u64) {
    println!("start - main2");
    let mut scores: HashMap<u64, u64> = HashMap::new();
    //let x: u64 = 260;
    let mut collatzval: u64;
    let mut counter = 1;
    loop {
        let val = collatz2(counter, &mut scores);
        if x == val{
            println!("{}",counter);
            break;
        }
        counter+=1;
    }
    println!("end - main2");
}

fn main1(x:u64){
    println!("start - main1");
    let mut collatzvalue;
    let mut counter = 1;
    loop{
        collatzvalue = collatz(counter);
        if collatzvalue == x {
            println!("{}",counter);
            break;
        }
        counter += 1;
    }
    println!("end - main1");
}

fn main(){
    let input:u64 = 260;//input representing a number of Collatz steps (steps required to reach 1 by following the Collatz procedure
    let mut start = Instant::now();
    main1(input);
    println!("{}milli seconds", start.elapsed().as_millis());
    start = Instant::now();
    main2(input);
    println!("{}milli seconds", start.elapsed().as_millis());
}