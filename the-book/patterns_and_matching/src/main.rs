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

    // ===================================
    //   Conditional if let Expressions
    // ===================================
    let authorization_status: Option<&str> = None;
    let is_admin = false;
    let group_id: Result<u8, _>  = "34".parse();

    if let Some(status) = authorization_status {
        println!("Authorization status: {}", status);
    } else if is_admin {
        println!("Authorization status: admin");
    } else if let Ok(group_id) = group_id {
        if group_id > 30 {
            println!("Authorization status: privileged")
        } else {
            println!("Authorization status: basic")
        }
    }

    // ===================================
    //   While let Conditional Loops
    // ===================================
    let mut stack = Vec::new();

    stack.push(1);
    stack.push(2);
    stack.push(3);

    while let Some(top) = stack.pop() {
        println!("{}", top)
    }
    
}
