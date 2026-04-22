mod exercises;

// L'énumération Result est utilisée pour représenter le résultat d'une
// opération qui peut réussir ou échouer.
// Resut {
//    Ok(T) --> succès, contient une valeur de type T
//    Err(E) --> échec, contient une valeur de type E
// }
fn get_result_even(num: i32) -> Result<i32, i32> {
    if num % 2 == 0 {
        Ok(num)
    } else {
        Err(-1)
    }    
}

// L'énumération Option est utilisée pour représenter une valeur qui peut 
// être présente ou absente.
// Option<T> {
//   Some(T) --> contient une valeur de type T
//   None --> aucune valeur
// }
fn get_option_even(num: i32) -> Option<i32> {
    if num % 2 == 0 {
        Some(num)
    } else {
        None
    }
}

fn main() {
    //----- RESULT ----- //
    
    // Result --> Ok(value) or Err(error)
    let res = get_result_even(10);
    dbg!(res);
    
    // Destructurer le Result pour obtenir la valeur contenue dans Ok, 
    // ou afficher un message d'erreur en cas de Err et retourner une 
    // valeur par défaut
    let res_ok = match res {
        Ok(value) => value,
        Err(err) => {
            println!("Error: {err}");
            -1
        }
    };
    dbg!(res_ok);
    
    // Destructurer le Result pour obtenir la valeur contenue dans Ok, ou une valeur par défaut en cas d'erreur
    let res_value = if let Ok(value) = res {
        value 
    } else {
        -1 
    };
    
    dbg!(res_value);
    
    // Option --> Some(value) or None
    let some_num = get_option_even(10);
    dbg!(some_num);
    
    // Destructurer l'Option pour voir la valeur contenue dans Some, 
    // ou afficher un message si c'est None
    match some_num {
        Some(num) => println!("{num}"),
        None => println!("No number"),
    }
    
    // if let --> expression qui permet d'extraire la valeur de l'option de la 
    // meme facon
    let num = if let Some(num) = some_num {
        num
    } else {
        0
    };
    dbg!(num);
    
    // je peux raccourcir en utilisant let else 
    let Some(num) = some_num else {
        return 
        // je dois faire autre chose et non returner une valeur par défaut,
        // sinon le type de num serait i32 et pas Option<i32>
    };
    dbg!(num);
}
