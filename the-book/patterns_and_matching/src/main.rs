fn main() {
    // =======================
    //    Match Expressions
    // =======================
    #[derive(Debug)]
    enum Language {
        English,
        Spanish,
        Russian,
        Japanese,
    }

    let language = Language::English;

    match language {
        Language::English => println!("Hello World!"),
        Language::Spanish => println!("Hola Mondo!"),
        Language::Russian => println!("Привет мир!"),
        lang => println!("Unsupported language! {:?}", lang),
    }

    // =======================
    //    Match Expressions
    // =======================
}
