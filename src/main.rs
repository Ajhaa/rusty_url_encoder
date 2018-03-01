fn main() {
    let encoded = encode(&String::from("äiti on ötökkä åååå"));
    println!("{}", encoded);
    let decoded = decode(&encoded);
    println!("{}", decoded);
    let e = String::from("a%");
    println!("{}", decode(&e));
}

fn encode(target: &String) -> String {
    let mut output: String = String::new();

    for i in target.chars() {
        match i {
             ' ' => output.push_str("%20"),
             'Ä' => output.push_str("%C3%84"),
             'ä' => output.push_str("%C3%A4"),
             'Ö' => output.push_str("%C3%96"),
             'ö' => output.push_str("%C3%B6"),
             'Å' => output.push_str("%C3%85"),
             'å' => output.push_str("%C3%A5"),
             _ => output.push(i)
        }
    }
    output
}
#[derive(PartialEq)]
enum ScanMode {
    Normal, Escape
}

fn decode(target: &String) -> String {
    let mut output = String::new();
    let mut mode = ScanMode::Normal;
    let mut buffer = String::new();
    for c in target.chars() {
       match mode {
           ScanMode::Normal => {
               mode = normal_scan(c, &mut output);
           }, 
           ScanMode::Escape => {
               mode = escape_scan(c, &mut output, &mut buffer);
           }
       }
    }
    if mode == ScanMode::Escape {
        panic!("Unknown escape: %{}", buffer)
    }
    output
}

fn normal_scan(c: char, output: &mut String) -> ScanMode {
    match c {
        '%' => ScanMode::Escape,
        _ => {
            output.push(c);
            ScanMode::Normal
        }    
    }
}

fn escape_scan(c: char, output: &mut String, buffer: &mut String) -> ScanMode {
    buffer.push(c);
    if buffer.len() == 2 {
        match &buffer[..] {
            "20" => {
                output.push(' ');
                buffer.clear();
                ScanMode::Normal
            }
            _ => ScanMode::Escape
        }
    } else if buffer.len() == 5 {
        output.push(match &buffer[..] {
            "C3%84" => 'Ä',
            "C3%A4" => 'ä',
            "C3%96" => 'Ö',
            "C3%B6" => 'ö',
            "C3%85" => 'Å',
            "C3%A5" => 'Å',
            _ => panic!("Unknown escape: %{}", buffer),
        });
        buffer.clear();
        ScanMode::Normal
    } else {
        ScanMode::Escape
    }

    
} 


