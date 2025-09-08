fn calcular_factorial (numero : u128) -> u128 {
    if numero  == 0 || numero == 1 {
        return 1;
    } else {
        let mut resultado = numero ;
        for i in (1..numero).rev() {
            resultado = resultado*i 
        } resultado
    }
} 

fn es_primo (numero : u128)-> bool {
    let mut es_primo=true;
    let num : f64 = numero as f64;
    if numero > 1 { 
        for i in 2..  ((num.sqrt() as i128)+1) {
            if (numero as i128) % i == 0 {
                es_primo = false ;
                break;
            }
        }

    }es_primo
} 
let resultado_primo= es_primo(17);
    println!("Es primo {} ",resultado_primo);

    let resultado_factorial = calcular_factorial(6);
    println!("El resultado factorial es {}",resultado_factorial); 
}
