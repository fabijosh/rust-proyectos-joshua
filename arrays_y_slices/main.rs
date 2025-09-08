struct Kathia {
    nombre:String,
    edad:u8,
    peso:u8,
    nacionalidad:String,
    genero:String,
}
impl Kathia {
    fn presentacion(&self) { 
        println!("nombre {} \nedad : {} \nnacionalidad : {} \ngenero : {}",self.nombre,self.edad,self.nacionalidad,self.genero);
        }

        fn peso (&self,menor_peso:u8) { 
            if self.peso > menor_peso {
                println!("{} pasada de peso , peso ideal {}",self.peso,menor_peso);
            } else {
                println!("{} peso ideal , baja de peso {}",self.peso,menor_peso);
            }

        } 

        fn mayor_edad (&self) -> bool {
            self.edad > 18 
        } 
    
}

struct Caliel { 
    nombre:String,
    edad:u8,
    energia:u8

} 

impl Caliel { 
    fn mostrar (&self) {
        println!("nombre {} \nedad {} años \nenergia {}% ",self.nombre,self.edad,self.energia);
    } 
    fn recargando (&mut self,mas_energia:u8) {
        self.energia += mas_energia

    } 

    fn cumpleaños (&mut self,mas_años:u8) {
        self.edad += mas_años
    }
    
}

fn main(){
let mut frutas = ["manzana","fresa","frambuesa","uva"]; 
frutas[1]="fresa";
frutas[2]="piña";
for fruta in frutas {
    println!("fruta : {:?}",fruta)
}
println!("Fruta que fue cambiada del inventario :{} y fruta favorita del del cliente {}",frutas[1],frutas[2]);
println!();

let mut temperaturas:[f32;5] = [15.0,20.5,17.7,16.0,11.0];
temperaturas[1]=17.0;
for temperatura in temperaturas {
    println!("temperatura por dia {}",temperatura);
}
println!("Temperatura mas alta de la semana {} temperatura mas baja {}",temperaturas[2],temperaturas[4]);
println!();
let mut calificaciones:[u32;4] = [8,7,4,9];
calificaciones [1] = 9;
for calificacion in calificaciones {
    println!("Calificacion por materia {}",calificacion);
}
println!("El alumno subio de calificacion a : {} y el alumno saco : {} en matematicas", calificaciones [1], calificaciones [2]);
println!();
let mut alumnos :[[u32;4];2] =[
    [10,8,6,7],
     [9,7,10,9]
];
alumnos[0][2]=8;
alumnos[1][1]=10;

println!("Alumno 1 tuve un calificacion final en ciencias de {} \nAlumno2 tuvo calificacion final en ciencias de {}",alumnos[0][2],alumnos[1][1]);

let mut inventario:[[[u32;3];2];3] = [
    [
        [150,120,110],
        [130,101,115],
    ],

    [
        [102,103,104],
        [105,106,107],
    ],
    [
        [121,122,123],
        [124,425,126],
    ],
]; 

inventario[0][0][0]=190;
inventario[2][1][1]=500;
inventario[1][1][2]=100; 

println!("producto mas vendido : {} \nProducto mas caro  : {} 
\nProducto mas barato : {} \nProductos menos vendido : {} "
,inventario[1][1][0],inventario[2][1][1],inventario[1][1][2],inventario[0][0][0]); 

let mut  ventanas :[bool;4] = [true,false,false,true];
ventanas[2]=true;
ventanas[3]= false;

if !ventanas [3]{
    println!("ventana equivocada cerrada");
} else {
    println!("Ventanas en orden");
} 

println!("Estado actual de las ventanas {:#?}",ventanas); 
let mut produtos : [u32;4]=[100,102,103,104];
    produtos[3]=200;
    let mut total= 0;
    let ultimos = &produtos[1..4];
    for produto in ultimos {
        total += produto;
        println!("Elementos a sumar {:#?}",ultimos);
 
  

} 
println!("Resulto de la suma {}",total);

let  suma:u32 =produtos.iter().sum();
println!("Elementos a sumar {:#?}",produtos);
println!("Resultado de la suma {}",suma);
println!();

let mut pasos : [i32;15] = [
    1000,1500,900,500,
    2000,11000,3000,300,
    100,15000,1000,500,
    15000,2000,4000,
]; 
pasos.sort();
println!("Orden de los dias de manerda ascendente {:#?}",pasos);
println!();

let mut pasos_agregados = 0;

pasos_agregados += pasos [0];
println!("Dia 1 : {} ",pasos_agregados);
pasos_agregados += pasos [2];
println!("Dia 2 : {} ",pasos_agregados);
pasos_agregados += pasos [3];
println!("Dia 3: {} ",pasos_agregados);
pasos_agregados += pasos [4];
println!("Dia 4: {} ",pasos_agregados);
pasos_agregados += pasos [5];
println!("Dia 5: {} ",pasos_agregados);
pasos_agregados += pasos [6];
println!("Dia 6: {} ",pasos_agregados);
pasos_agregados += pasos [7];
println!("Dia 7: {} ",pasos_agregados);
pasos_agregados += pasos [8];
println!("Dia 8: {} ",pasos_agregados);
pasos_agregados += pasos [9];
println!("Dia 9: {} ",pasos_agregados);
pasos_agregados += pasos [10];
println!("Dia 10: {} ",pasos_agregados);
pasos_agregados += pasos [11];
println!("Dia 11: {} ",pasos_agregados);
pasos_agregados += pasos [12];
println!("Dia 12: {} ",pasos_agregados);
pasos_agregados += pasos [12];
println!("Dia 13: {} ",pasos_agregados);
pasos_agregados += pasos [13];
println!("Dia 14: {} ",pasos_agregados);
pasos_agregados += pasos [14];
println!("Dia 15: {} ",pasos_agregados);
println!("Numeros de pasos generados en total {:#?}",pasos_agregados); 
println!();

let  primer_semana = &pasos [0..7];
let suma :i32 = primer_semana.iter().sum();
println!("Numero de pasos dados en la primer semana {:?}",suma);
println!();
let ultima_semana = &pasos[7..15];
let mut total = 0;
for paso in ultima_semana {
    total += paso;
    println!("Cual es la suma de la ultima semana {}",total);
} let persona1 = Kathia {
        nombre:String::from("Kathia"),
        edad:21,
        peso:60,
        nacionalidad:String::from("Mexicana"),
        genero:String::from("Femenina"),
    };
persona1.presentacion();
persona1.peso(60);
if persona1.mayor_edad(){
    println!("Eres mayor de edad");
} else {
    println!("No eres mayor de edad");
} 

println!();

let mut persona1 = Caliel{
    nombre:String::from("Caliel"),
    edad:5,
    energia:80,
}; 
persona1.mostrar();
persona1.recargando(20);
persona1.cumpleaños(1);
println!("Comiendo......");
println!("Cumpleaños....");
persona1.mostrar(); 
}


