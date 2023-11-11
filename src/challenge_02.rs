/// # CHALLENGE_02 -> **Mini Compiler**
///
/// En el mundo del espionaje, se utiliza un lenguaje de codificación con símbolos que realizan operaciones matemáticas simples.
///
/// ```
/// Tu tarea es crear un mini compilador que interprete y ejecute programas escritos en este lenguaje de símbolos.
///
/// Las operaciones del lenguaje son las siguientes:
/// - `#`: Incrementa el valor numérico en 1.
/// - `@`: Decrementa el valor numérico en 1.
/// - `*`: Multiplica el valor numérico por sí mismo.
/// - `&`: Imprime el valor numérico actual.
///
/// El valor numérico inicial es 0 y las operaciones deben aplicarse en el orden en que aparecen en la cadena de símbolos.
/// ```
///
/// ## Return
/// El mini compilador que tome una cadena de texto y devuelva otra cadena de texto con el resultado de ejecutar el programa.
///
///
/// ## Examples
/// - Entrada: `"##*&"`
/// - Salida esperada: `"4"`
/// - Explicación: Incrementa (1), incrementa (2), multiplica (4), imprime (4).
///
/// - Entrada: `"&##&*&@&"`
/// - Salida esperada: `"0243"`
/// - Explicación: Imprime (0), incrementa (1), incrementa (2), imprime (2), multiplica (4), imprime (4), decrementa (3), imprime (3).
pub fn solve() {
    println!("\n\t\t\t\tCHALLENGE 02");
    let input = "&###@&*&###@@##@##&######@@#####@#@#@#@##@@@@@@@@@@@@@@@*&&@@@@@@@@@####@@@@@@@@@#########&#&##@@##@@##@@##@@##@@##@@##@@##@@##@@##@@##@@##@@##@@##@@##@@&";
    println!(
        "Entrada: {}\nSalida esperada: {}",
        input,
        mini_compiler(input)
    );
}

fn mini_compiler(input: &str) -> String {
    let mut result = String::new();
    let mut value: i32 = 0;

    for operator in input.chars() {
        match operator {
            '#' => value += 1,
            '@' => value -= 1,
            '*' => value *= value,
            '&' => result.push_str(&value.to_string()),
            _ => panic!("Error de sintaxis: El operador '{}' no existe", operator),
        }
    }

    result
}

#[cfg(test)]
mod tests {
    use super::mini_compiler;

    #[test]
    fn test_mini_compiler() {
        assert_eq!(mini_compiler(""), "");
        assert_eq!(mini_compiler("**#"), "");
        assert_eq!(mini_compiler("##*&"), "4");
        assert_eq!(mini_compiler("&##&*&@&"), "0243");
    }

    #[test]
    #[should_panic]
    fn test_mini_compiler_panic() {
        mini_compiler("##*&$");
    }
}
