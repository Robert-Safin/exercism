pub fn abbreviate(phrase: &str) -> String {
    let clean: String = sanitize(phrase);
    if is_camel(&clean) {
        camel_acronym(&clean)
    } else {
        non_camel_acronym(&clean)
    }
}

fn camel_acronym(clean: &str) -> String {
    let mut acronym: String = String::new();
    let words: Vec<_> = clean.split(" ").collect();
    for word in words {
        let chars: std::str::Chars<'_> = word.chars();
        chars.for_each(|char| {
            if char.is_ascii_uppercase() {
                acronym.push(' ');
                acronym.push(char)
            } else {
                acronym.push(char)
            }
        })
    }
    non_camel_acronym(&acronym)
}

fn non_camel_acronym(clean: &str) -> String {
    let mut acronym: String = String::new();
    clean.split(" ").for_each(|word| {
        let first: Option<char> = word.chars().next();
        if first.is_some() {
            acronym.push(first.unwrap().to_ascii_uppercase())
        }
    });
    acronym
}

fn sanitize(phrase: &str) -> String {
    phrase.replace("-", " ").replace("_", " ")
}

fn is_camel(clean: &str) -> bool {
    let words: Vec<_> = clean.split(" ").collect();
    let mut vec: Vec<bool> = Vec::new();
    for word in words {
        let chars: std::str::Chars<'_> = word.chars();
        let mut capital_count = 0;
        chars.for_each(|char| {
            if char.is_ascii_uppercase() {
                capital_count += 1;
            }
        });
        if capital_count > 1 && capital_count < word.len() {
            vec.push(true)
        }
    }
    if vec.into_iter().any(|val| val == true) {
        true
    } else {
        false
    }
}
