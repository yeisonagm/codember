/// CHALLENGE_01.txt
/// ```
/// Un espía está enviando mensajes encriptados.
/// Tu misión es crear un programa que nos ayude a buscar patrones...
/// Los mensajes son palabras separadas por espacios como este: gato perro perro coche Gato peRRo sol
/// Necesitamos que el programa nos devuelva el número de veces que aparece cada palabra en el mensaje, independientemente de si está en mayúsculas o minúsculas.
/// ```
///
/// # Return
/// El resultado será una cadena de texto con la palabra y el número de veces que aparece en el mensaje, con este formato:
/// **_gato2perro3coche1sol1_**
///
/// ¡Las palabras son ordenadas por su primera aparición en el mensaje!
///
/// # Examples
/// llaveS casa CASA casa llaves -> llaves2casa3
///
/// taza ta za taza -> taza2ta1za1
///
/// casas casa casasas -> casas1casa1casasas1
pub fn solve() {
    let message = "murcielago leon jirafa cebra elefante rinoceronte hipopotamo ardilla mapache zorro lobo oso puma jaguar tigre leopardo gato perro caballo vaca toro cerdo oveja cabra gallina pato ganso pavo paloma halcon aguila buho colibri canario loro tucan pinguino flamenco tigre jaguar leopardo oso lobo zorro mapache ardilla elefante rinoceronte hipopotamo cebra jirafa leon murcielago cabra oveja cerdo toro vaca caballo perro gato leopardo tigre jaguar oso lobo zorro mapache ardilla hipopotamo rinoceronte elefante jirafa leon murcielago pavo ganso pato gallina cabra oveja cerdo toro vaca caballo perro gato leopardo tigre jaguar oso lobo zorro mapache ardilla hipopotamo rinoceronte elefante jirafa leon murcielago buho aguila halcon paloma pavo ganso pato gallina cabra oveja cerdo toro vaca caballo perro gato leopardo tigre jaguar oso lobo zorro mapache ardilla hipopotamo rinoceronte elefante jirafa leon murcielago colibri buho aguila halcon paloma pavo ganso pato gallina cabra oveja cerdo toro vaca caballo perro gato leopardo tigre jaguar oso lobo zorro mapache ardilla hipopotamo rinoceronte elefante jirafa leon murcielago tucan loro canario colibri buho aguila halcon paloma pavo ganso pato gallina cabra oveja cerdo toro vaca caballo perro gato leopardo tigre jaguar oso lobo zorro mapache ardilla hipopotamo rinoceronte elefante jirafa leon murcielago flamenco pinguino tucan loro canario colibri buho aguila halcon paloma pavo ganso pato gallina cabra oveja cerdo toro vaca caballo perro gato leopardo tigre jaguar oso lobo zorro mapache ardilla hipopotamo rinoceronte elefante jirafa leon murcielago jaguar oso lobo zorro mapache ardilla cebra elefante rinoceronte hipopotamo leon jirafa murcielago caballo vaca toro cerdo oveja cabra gallina pato ganso pavo paloma halcon aguila buho colibri canario loro tucan pinguino flamenco jaguar oso lobo zorro mapache ardilla cebra elefante rinoceronte hipopotamo leon jirafa murcielago caballo vaca toro cerdo oveja cabra gallina pato ganso pavo paloma halcon aguila buho colibri canario loro tucan pinguino flamenco murcielago leon jirafa cebra elefante rinoceronte hipopotamo ardilla mapache zorro lobo oso puma jaguar tigre leopardo gato perro caballo vaca toro cerdo oveja cabra gallina pato ganso pavo paloma halcon aguila buho colibri canario loro tucan pinguino flamenco oso lobo zorro mapache ardilla hipopotamo rinoceronte elefante jirafa leon murcielago cabra oveja cerdo toro vaca caballo perro gato leopardo tigre jaguar oso lobo zorro mapache ardilla cebra elefante rinoceronte hipopotamo jirafa leon murcielago pavo ganso pato gallina cabra oveja cerdo toro vaca caballo perro gato buho aguila halcon paloma colibri canario loro tucan pinguino flamenco jaguar oso lobo zorro mapache ardilla hipopotamo rinoceronte elefante jirafa leon murcielago cabra oveja cerdo toro vaca caballo perro gato buho aguila halcon paloma colibri canario loro tucan pinguino flamenco jaguar oso lobo zorro mapache ardilla hipopotamo rinoceronte elefante jirafa leon murcielago cabra oveja cerdo toro vaca caballo perro gato buho aguila halcon";

    println!("\n\t\t\t\tCHALLENGE 01");
    println!("{}", look_for_patterns(message));
}

fn look_for_patterns(message: &str) -> String {
    let words: Vec<String> = message
        .to_lowercase()
        .split(" ")
        .map(|s| s.to_string())
        .collect();

    let mut words_count: Vec<(String, u8)> = Vec::new();
    words.iter().for_each(|word| {
        if !words_count.iter().any(|(key, _)| key == word) {
            words_count.push((
                word.to_string(),
                words.iter().filter(|&e| e.as_str() == word).count() as u8,
            ));
        }
    });

    words_count
        .iter()
        .map(|(key, value)| format!("{}{}", key, value))
        .collect()
}

#[cfg(test)]
mod tests {
    use super::*;

    /// Tests for the `look_for_patterns` function.
    #[test]
    fn test_look_for_patterns() {
        let message1 = "llaveS casa CASA casa llaves";
        assert_eq!(look_for_patterns(message1), "llaves2casa3");
        let message2 = "taza ta za taza";
        assert_eq!(look_for_patterns(message2), "taza2ta1za1");
        let message3 = "casas casa casasas";
        assert_eq!(look_for_patterns(message3), "casas1casa1casasas1");
    }
}