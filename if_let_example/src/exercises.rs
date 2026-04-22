pub fn divide_result(a: i32, b: i32) -> Result<i32, String> {
    match b {
        // Retourner un result Ok contenant le résultat de la division 
        // si b n'est pas zéro
    }
}

pub fn divide_option(a: i32, b: i32) -> Option<i32> {
    match b {
        // Retourner un option Some contenant le résultat de la division 
        // si b n'est pas zéro
    }
}

pub fn divide_if_let(a: i32, b: i32) -> i32 {
    // Utiliser devide result ou divide option pour effectuer la division
    // Utiliser if let pour effectuer la division et retourner le résultat, 
    // ou une valeur par défaut en cas d'erreur
    // La valeur par défaut doit être -1 en cas d'erreur
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_divide_result() {
        assert_eq!(divide_result(10, 2), Ok(5));
        assert_eq!(divide_result(10, 0), Err(String::from("Division by zero")));
    }

    #[test]
    fn test_divide_option() {
        assert_eq!(divide_option(10, 2), Some(5));
        assert_eq!(divide_option(10, 0), None);
    }

    #[test]
    fn test_divide_if_let() {
        assert_eq!(divide_if_let(10, 2), 5);
        assert_eq!(divide_if_let(10, 0), -1); // Valeur par défaut en cas d'erreur
    }
}