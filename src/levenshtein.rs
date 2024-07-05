pub fn distance(str1: &str, str2: &str) -> bool {
    let len1 = str1.len();
    let len2 = str2.len();

    if (len1 as i32 - len2 as i32).abs() > 1 {
        return false;
    } 

    let mut edits = 0;
    let mut i = 0;
    let mut j = 0;

    while i < len1 && j < len2 {
        if str1.chars().nth(i) != str2.chars().nth(j) {
            edits += 1;
            if len1 > len2 {
                i += 1;
            } else if len1 < len2 {
                j += 1;
            } else {
                i += 1;
                j += 1;
            }
        } else {
            i += 1;
            j += 1;
        }

        // se houver mais de 1 edição, retorne falso
        if edits > 1 {
            return false
        }
    }

    if i < len1 || j < len2 {
        edits += 1;
        println!("Diferença de comprimento de 1. Incrementando edições");
    } 

    println!("Número total de edições: {}", edits);

    edits <= 1
}