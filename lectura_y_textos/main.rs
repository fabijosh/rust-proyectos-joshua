fn agregar_texto (s: &mut String) {
    s.push_str("Soy un robot"); 
}
fn leer_texto (s: &String) -> usize {
    s.trim().len()
} 
fn eres_robot (letras : usize) -> bool {
    letras > 10
} 

fn contar_vocales (texto: &String) -> usize {
    let mut contador = 0;
    for letras in texto.to_lowercase().chars() { 
    if letras == 'a'||letras == 'e'||letras == 'i'||letras == 'o'||letras == 'u' {
        contador += 1;

    }
} contador 

} 

fn invertir_texto (s:&String) ->String {
    s.chars().rev().collect()
}

fn contar_consonante (s:&String) -> usize {
s.to_lowercase().chars().filter(|c| c.is_ascii_alphabetic() && !"aeiou".contains(*c)).count()
} 

  let mut contador = String::new();
    println!("Ingresa tu nombre robot");
    std::io::stdin().read_line(&mut contador).unwrap();
    let letras = leer_texto(&contador);
     let total_vocales = contar_vocales(&contador); 
     if eres_robot(letras){
        println!("Eres robot");
     }else {
        println!("No eres robot");
     }
    let invertido=invertir_texto(&contador);
    let total_consonante = contar_vocales(&contador);
    agregar_texto(&mut contador);
    println!("tu nombre es {}  tiene {} letras y tiene {} vocales {} consonantes y tu nombre al reves es {} ",contador,letras,total_vocales,total_consonante,invertido);
}
