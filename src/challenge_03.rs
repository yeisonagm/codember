use std::{env, fs};

/// # CHALLENGE_03 -> El Desafío del Cifrado Espía
/// Un grupo de espías ha descubierto que su sistema de cifrado de mensajes está comprometido.
///
/// Han encontrado algunas contraseñas que no cumplen con la Política de Seguridad de Cifrado que tenían establecida cuando fueron creadas.
///
/// Determina cuántas claves de cifrado son válidas según sus políticas.
///
/// ## Return
/// Devuelva la clave inválida número 42 (de todas las claves inválidas, la 42ª en orden de aparición)
///
/// ## Example
/// Cada línea indica, separado por :, la política de la clave y la clave misma.
/// - 2-4 f: fgff
/// - 4-6 z: zzzsg
/// - 1-6 h: hhhhhh
///
/// La política de la clave especifica el número mínimo y máximo de veces que un carácter dado debe aparecer para que la clave sea válida.
/// Entonces, 2-4 f significa que la clave debe contener f al menos 2 veces y como máximo 4 veces.
/// Sabiendo esto, en el ejemplo anterior, hay 2 claves válidas:
/// La segunda clave, zzzsg, no lo es; contiene 3 veces la letra z, pero necesita al menos 4.
/// Las primeras y terceras claves son válidas: contienen la cantidad adecuada de f y h, respectivamente, según sus políticas.
pub fn solve() {
    let filename = "encryption_policies.txt";
    let policies_keys = read_file(filename);
    let n = policies_keys.len();
    let invalid_keys = invalid_keys(policies_keys);

    println!("\n\t\t\t\tCHALLENGE 03");
    println!(
        "Hay {} claves de cifrado que son válidas según sus políticas.",
        n - invalid_keys.len()
    );
    println!("Clave inválida número 42 es [{}]", invalid_keys[41]);
}

fn invalid_keys(policies_keys: Vec<String>) -> Vec<String> {
    let mut invalid: Vec<String> = Vec::new();

    for policy in policies_keys {
        let policy_key: Vec<&str> = policy.split(':').collect();
        let key = policy_key[1].trim();
        if !check_policy(policy_key[0], key) {
            invalid.push(key.to_string());
        }
    }

    invalid
}

fn check_policy(policy: &str, key: &str) -> bool {
    let policy_key: Vec<&str> = policy.split_whitespace().collect();
    let min_max: Vec<u8> = policy_key[0]
        .split('-')
        .map(|e| e.parse::<u8>().unwrap_or(0))
        .collect();
    let count_letter = key.matches(policy_key[1].trim()).count() as u8;
    min_max[0] <= count_letter && min_max[1] >= count_letter
}

pub fn read_file(filename: &str) -> Vec<String> {
    let mut lines: Vec<String> = Vec::new();
    let path = format!("{}/src/data/{}", working_directory(), filename);
    let content = fs::read_to_string(path)
        .expect("No se pudo leer el acrchivo con con las políticas de seguridad");

    for line in content.lines() {
        lines.push(line.to_string());
    }

    lines
}

pub fn working_directory() -> String {
    let mut dir = String::new();
    // get current working directory
    if let Ok(current_dir) = env::current_dir() {
        dir.push_str(current_dir.to_str().expect("Directory error"));
    }
    dir
}

#[cfg(test)]
mod tests {
    use super::*;

    /// Tests for the 'check_policy' function  
    #[test]
    fn test_check_policy() {
        assert_eq!(check_policy("2-4 f", "fgff"), true);
        assert_eq!(check_policy("4-6 z", "zzzsg"), false);
        assert_eq!(check_policy("1-6 h", "hhhhhh"), true);
        assert_eq!(check_policy("9-10 d", "bgamidqewtbus"), false);
        assert_eq!(check_policy("3-10 n", "llysdv"), false);
    }

    /// Tests for the 'invalid_key' function
    #[test]
    fn test_invalid_keys() {
        let policies_keys: Vec<String> = vec![
            "2-4 f: fgff".to_string(),
            "4-6 z: zzzsg".to_string(),
            "1-6 h: hhhhhh".to_string(),
        ];
        assert_eq!(invalid_keys(policies_keys), vec!["zzzsg".to_string()])
    }
}
