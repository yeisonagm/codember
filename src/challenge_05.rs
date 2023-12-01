use crate::challenge_03::read_file;
use regex::Regex;

/// # CHALLENGE_05 -> El problema final
///
/// Finalmente los hackers han conseguido acceder a la base de datos y la han dejado corrupta. Pero parece que han dejado un mensaje oculto en la base de datos. ¿Podrás encontrarlo?
///
/// ```
/// Nuestra base de datos está en formato .csv. Las columnas son id, username, email, age, location.
///
/// Un usuario sólo es válido si:
/// - id: existe y es alfanumérica
/// - username: existe y es alfanumérico
/// - email: existe y es válido (sigue el patrón user@dominio.com)
/// - age: es opcional pero si aparece es un número
/// - location: es opcional pero si aparece es una cadena de texto
/// ```
///
/// ## Return
/// Encuentra el primer caracter (número o letra) del username de cada usuario inválido. Júntalos por orden de aparición y descubre el mensaje oculto.
///
/// ## Example
/// Entrada: 1a421fa,alex,alex9@gmail.com,18,Barcelona
///
/// Resultado: ✅ Válido
///
/// Entrada: 9412p_m,maria,mb@hotmail.com,22,CDMX
///
/// Resultado: ❌ Inválido (id no es alfanumérica, sobra el _)
///
/// Entrada: 494ee0,madeval,mdv@twitch.tv,,
///
/// Resultado: ✅ Válido (age y location son opcionales)
///
/// Entrada: 494ee0,madeval,twitch.tv,22,Montevideo
///
/// Resultado: ❌ Inválido (email no es válido)
pub fn solve() {
    let database = read_file("database_attacked.txt");
    let invalid_users: Vec<String> = database.into_iter().filter(|e| !validate_user(e)).collect();
    println!("\n\t\t\t\tCHALLENGE 05");
    println!("Hay {} usuarios inválidos", invalid_users.len());
    println!("Mensaje oculto: {}", hidden_message(invalid_users));
}

fn hidden_message(invalid_users: Vec<String>) -> String {
    let mut message = String::new();
    invalid_users.iter().for_each(|e| {
        let user = e.split(',').collect::<Vec<&str>>();
        message.push(user[1].chars().next().unwrap());
    });
    message
}

fn validate_user(user: &str) -> bool {
    let column = user.split(',').collect::<Vec<&str>>();
    return check_id(column[0])
        && check_username(column[1])
        && check_email(column[2])
        && check_age(column[3])
        && check_location(column[4]);
}

fn check_id(id: &str) -> bool {
    !id.is_empty() && id.chars().all(char::is_alphanumeric)
}

fn check_username(username: &str) -> bool {
    !username.is_empty() && username.chars().all(char::is_alphanumeric)
}

fn check_email(email: &str) -> bool {
    Regex::new(r"^[a-zA-Z0-9_.+-]+@[a-zA-Z0-9-]+\.[a-zA-Z0-9-.]+$")
        .unwrap()
        .is_match(email)
}

fn check_age(age: &str) -> bool {
    age.is_empty() || age.chars().all(char::is_numeric)
}

fn check_location(location: &str) -> bool {
    location.is_empty()
        || location
            .chars()
            .all(|c| c.is_alphabetic() || c.is_whitespace())
}

#[cfg(test)]
mod tests {
    use super::validate_user;

    /// Test that a user is valid
    #[test]
    fn test_validate_user() {
        assert_eq!(
            validate_user("1a421fa,alex,alex9@gmail.com,18,Barcelona"),
            true
        );
        assert_eq!(validate_user("9412p_m,maria,mb@hotmail.com,22,CDMX"), false);
        assert_eq!(validate_user("494ee0,madeval,mdv@twitch.tv,,"), true);
        assert_eq!(validate_user("aN21Mhj,yoHrWHE,oHrWHE@example,40,"), false);
        assert_eq!(
            validate_user("494ee0,madeval,twitch.tv,22,Montevideo"),
            false
        );

        assert_eq!(
            validate_user("(MIDUni,3vsWtBk,midu@gmail.com,,Barcelona"),
            false
        );
        assert_eq!(
            validate_user("f9Fw1q6tWq,djrWrL,djrwrl@outlook.com,20,New York"),
            true
        );
    }
}
