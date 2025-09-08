fn fizzbuzz (){
    for numero in 1..100 {
        if numero % 3 == 0 && numero % 5 == 0 {
            println!("Fizzbuzz");
        } else if numero % 3 == 0 {
            println!("fizz");
            
        } else if numero % 5 == 0  {
            println!("Buzz");
            
        } else {
            println!("{}", numero);
        }
    }
} 

 fn main (){
    fizzbuzz();
    println!();
}
