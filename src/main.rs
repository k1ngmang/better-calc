fn main(){
    let mut code = r#"
        use std::io;
        fn main(){
            loop {
                print!("> ");
                io::Write::flush(&mut io::stdout()).unwrap();
                let mut expr = String::new();
                io::stdin().read_line(&mut expr).unwrap();
                let expr = expr.trim();
    "#.to_string();

    for a in 1..=20 {
        for b in 1..=20 {
            code.push_str(&format!(r#"
                    if expr == "{}+{}" {{ println!("{}"); }}
            "#, a,b, a+b));
            code.push_str(&format!(r#"
                    if expr == "{}-{}" {{ println!("{}"); }}
            "#, a,b, a-b));
        }
    }

    code.push_str(r#"}}"#);
    std::fs::write("calc.rs", code).unwrap();
    println!("hello world");
}

