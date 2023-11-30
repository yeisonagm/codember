use crate::challenge_03::read_file;

/// # CHALLENGE_04 -> Hackers dañan sistema de archivos
///
/// En un mundo donde la información es poder, un hacker conocido como Savipo Yatar descubre una serie de archivos en un servidor altamente protegido.
///
/// Estos archivos contienen secretos que podrían cambiar el curso de la historia. Pero hay un problema: algunos de los archivos son falsos, diseñados para engañar a los intrusos. Savipo Yatar debe determinar rápidamente cuáles archivos son reales y cuáles son falsos.
///
/// Cada archivo tiene un nombre con dos partes, separadas por un guion (-). La primera parte es una cadena alfanumérica y la segunda es unchecksum, que es una cadena formada por los caracteres que sólo aparecen una vez en la primera parte y en el orden en que aparecen.
///
/// ```
/// Determine si un archivo es real o falso basado en estas reglas.
/// ```
///
///  ## Return
/// Busca el archivo real número 33 (de todos los archivos reales, el 33º en orden de apareción).
///
/// ## Example
/// Nombre del archivo: xyzz33-xy
///
/// Resultado: ✅ Real (El checksum es válido)
///
/// Nombre del archivo: abcca1-ab1
///
/// Resultado: ❌ Falso (El checksum debería ser b1, es incorrecto)
///
/// Nombre del archivo: abbc11-ca
///
///Resultado: ❌ Falso (El checksum debería ser ac, el orden es incorrecto)
pub fn solve() {
    let files = read_file("files_quarantine.txt");
    let real_files = real_files(files);
    println!("\n\t\t\t\tCHALLENGE 04");
    println!("Número de archivos reales: {}", real_files.len());
    println!("Archivo real número 33: {}", &real_files[32]);
}

fn real_files(files: Vec<String>) -> Vec<String> {
    files.into_iter().filter(|e| check_file(e)).collect()
}

fn check_file(name: &str) -> bool {
    let filename = name.split('-').collect::<Vec<&str>>();
    let mut previous_index = 0;

    for letter in filename[1].chars() {
        if filename[0].matches(letter).count() != 1 {
            return false;
        }
        // check the order of unchecksum
        let index = filename[0].find(letter).unwrap();
        if index < previous_index {
            return false;
        };
        previous_index = index;
    }

    true
}

#[cfg(test)]
mod tests {
    use super::{check_file, real_files};

    #[test]
    fn test_check_file() {
        assert_eq!(check_file("xyzz33-xy"), true);
        assert_eq!(check_file("abcca1-ab1"), false);
        assert_eq!(check_file("abbc11-ca"), false);
        assert_eq!(check_file("4VNq3-4VNq3"), true);
        assert_eq!(check_file("MsW8Mg-sW8g"), true);
    }

    #[test]
    fn test_real_files() {
        let files = vec![
            String::from("xyzz33-xy"),
            String::from("abcca1-ab1"),
            String::from("abbc11-ca"),
        ];
        let result = vec![String::from("xyzz33-xy")];
        assert_eq!(real_files(files), result);
    }
}
