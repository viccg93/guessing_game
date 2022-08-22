//Cuando la libreria no esta en el preludio, se debe de importar explicitamente
//io pertenece a la libreria estandar pero no pertenece al preludio
use std::io;

//Rng es un trait de rand que define metodos que implementan los generadores de numeros aleatorios.
use rand::Rng;

//Ordering es un enum
use std::cmp::Ordering;

fn main() {
    println!("Adivina el numero secreto");
    
    //rand::thread_rng() nos permite obtener un generador en el mismo thread y con semilla generada por el SO.
    //gen_range() es una funcion que podemos utilizar gracias al trait Rng.
    //Recibe una expresion de rango, en este caso inclusiva en ambos limites.

    let secret_number = rand::thread_rng().gen_range(1..=100);

    // loop, salida se marca con break;
    loop {
        println!("Por favor introduce tu conjetura (numero)");
        
        //En rust las variables se declaran con let
        //las variables son inmutables por defecto, mediante mut indicamos que es mutable (puede cambiar su valor)
        //String es un tipo definido en la libreria estandar correspondiente a cadena UTF-8
        //mediante la funcion asociada new obtenemos una instancia del tipo String.
        let mut guess = String::new();

        //Podemos usar io, por que fue importado mediante use, alternativamente se puede usar std::io::stdin()
        //en caso de evitar importar con use.
        //La funcion stdin() nos devuelve una instancia que entre sus funciones tiene el metodo read_line
        //que permite leer texto que se introduce en el terminal, pero lo hace de forma que va añadiendo
        //caracter a caracter, lo cual implica multiples cambios, de ahi la necesidad de que la variable sea mutable
        //este metodo recibe la referencia de la variable (tambien se debe marcar como mutable) y se indica con &
        //este metodo retorna un Result que es un enum, podemos mandar un mensaje con expect() en caso de que el result sea err.
        io::stdin()
        .read_line(&mut guess)
        .expect("Error al leer tu numero");
        //Convertimos el numero introducido por el usuario (String) a un int de 32 bits
        //ya que Rust es un lenguaje de tipado estatico (aunque cuenta con inferencia bajo ciertas condiciones).
        //trim() quita los espacios en blanco y los saltos de linea previo al cast.
        //Rust permite el overshadow de las variables.
        let guess: u32 = match guess.trim().parse(){
            //evaluamos con match el resultado de parse que tambien es un enum
            //puede ser Ok si se realizo el cast, Err en caso contrario
            //en un caso exitoso retornamos el valor resultante.
            Ok(num) => num,
            //_ en este contexto es un valor comodin (catchall)
            // cuando se encuentra con un error al hacer el cast, continue indica que se inicie una nueva iteracion del loop
            Err(_) => continue,
        };
        //imprime variable usando placeholder.
        println!("Tu conjetura es {guess}");

        //match evalua una expresion y de acuerdo a los Result, en su cuerpo busca empatar dicho result y ejecutar una tarea determinada
        //en este caso mediante la funcion cmp() compara guess con secret_number, y como resultado se tiene el result que puede ser 
        //Ordering::Less, Ordering::Greater u Ordering::Equal (los podemos usar gracias a use std::cmp::Ordering)
        //Una vez que coincide el result, termina la comparacion y sale del match (patron del primer brazo).
        match guess.cmp(&secret_number){
            Ordering::Less => println!("Tu numero es mas bajo que el numero secreto"),
            Ordering::Greater => println!("Tu numero es mas grande que el numero secreto"),
            Ordering::Equal => {
                println!("Le atinaste al numero, ganaste!");
                break; //salida de loop
            },
        };
    }
}
