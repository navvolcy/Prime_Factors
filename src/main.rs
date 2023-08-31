fn primeFactors(mut n:u32) -> i32{
    let mut count = 0;

    while n % 2 == 0 {
        //prints the number or 2's
        println!("{}", 2);
        count+=1;
        n = n / 2;
    }
    
    let mut i = 3u32;
    
    while i*i <=  n {
       
        if n % i == 0 {
            println!("{}", i);
            count+=1;
            n = n / i;
        }
        else{
            i = i + 2;
        }
    }

    if n > 2 {
        println!("{}", n);
        count+=1;

    }

    return count;

}

fn main() {

 let n = 360;
println!("{n} has {} prime factors",primeFactors(n)); 
  
}

