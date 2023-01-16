use std::io;
use std::cmp::Ordering;
use rand::Rng;
use colored::*;

fn main() {
    println!("¡Adivina el numero!");

    //Declara e inicializa la variable de un numero al azar que se intentara adivinar
    let numero_aleatorio: u8 = rand::thread_rng().gen_range(0..101);
    
    //Declara la variable tipo String donde se guardara el intento de adivinanza

    println!("El numero aleatorio es {}", numero_aleatorio);

    loop {

        /*Si declaras la variable fuera del bucle da un error impidiendo que se lea correctamente
        Lo que ocurre es que cuando se lee la linea, lo concadena con la linea previamente guardada en la variable
        Ejemplo: Ingresas 100, la variable toma el valor de "100\n". Luego ingresas 30, la varable se concadena,
        es decir, la variable ahora es "100\n30\n" en lugar de "30\n", esto causa un error al transformarlo a un numero*/
        let mut numero_elegido = String::new();

        println!("Por favor ingrese un numero: ");
        
        //Lee la entrada del intento de adivinanza
        io::stdin().read_line(&mut numero_elegido)
            .expect("No se pudo leer la linea");
        
        //Crea una shadow variable donde almacenara la conversion de la entrada de String a un u8 
        let numero_elegido: u8 = match numero_elegido.trim().parse() {
            Ok(num) => num,
            Err(_) => continue,
        };
        
        println!("Su numero fue: {}", numero_elegido);
        
        //Compara el numero elegido con el numero a adivinar y responde
        match numero_elegido.cmp(&numero_aleatorio){
            Ordering::Greater => println!("{}", "Intenta un numero menor".red()),
            Ordering::Less => println!("{}","Intenta con un numero mayor".red()),
            Ordering::Equal => {
                println!("{}", "Acertaste, que gozu".green());
                break;
            }
        }   
    }

}
