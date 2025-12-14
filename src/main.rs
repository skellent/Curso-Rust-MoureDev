use std::io::Write;

/* Ejercicio 1 */
fn main() {
    let mut saludo: &str = "¡Hola, Skellent!";
    println!("> Saludo: {}", saludo); // Saluda: ¡Hola, Skellent!
    saludo = "Adios, Skellent!";
    println!("> Despedida: {saludo}"); // Saluda: Adios, Skellent!
    // saludo = 6; /* ERROR, hay tipado fuerte en Rust */

    /* Obtencion de Inputs */
    let mut nombre: String = String::new();
    print!("> ¿Cómo te llamas?:\n [Input]> ");
    std::io::stdout()
        .flush()
        .expect("Ocurrio un error al vaciar stdout");
    std::io::stdin()
        .read_line(&mut nombre)
        .expect("> Ocurrio un error al leer el Input");
    println!("> ¡Encantado de conocerte, {}!", nombre.trim());

    /* Aritmetica */
    let mut numero_ingresado: String = String::new();
    print!("> Ingresa un número:\n [Input]> ");
    std::io::stdout()
        .flush()
        .expect("Ocurrio un error al vaciar stdout");
    std::io::stdin()
        .read_line(&mut numero_ingresado)
        .expect("> Ocurrio un error al leer el Input");
    let numero: i32 = numero_ingresado
        .trim()
        .parse()
        .expect("Error al convertir String a i32, entrada inválida...");

    println!("> Operaciones del numero {numero} consigo mismo:");
    println!("> Suma ( {} + {} ) es {} ", numero, numero, numero + numero);
    println!("> Resta ( {} - {} ) es {} ", numero, numero, numero - numero);
    println!("> Multiplicación ( {} * {} ) es {} ", numero, numero, numero * numero);
    println!("> División ( {} / {} ) es {} ", numero, numero, numero / numero);
    println!("> Potencia de 2 ( {} ) es {} ", numero, numero.pow(2));


}
