#![feature(plugin)]
#![plugin(phf_macros)]

extern crate phf;

#[cfg(test)]
mod test;
static ESCAPE_TABLE: phf::Map<&'static str, char> = phf_map! {
    "%20" => ' ',
    "%C3%84" => 'Ä',
    "%C3%A4" => 'ä',
    "%C3%96" => 'Ö',
    "%C3%B6" => 'ö',
    "%C3%85" => 'Å',
    "%C3%A5" => 'å',
    "%2C" => ','
};

static CHAR_TABLE: phf::Map<char, &'static str> = phf_map! {
    ' ' => "%20",
    'Ä' => "%C3%84",
    'ä' => "%C3%A4", 
    'Ö' => "%C3%96",
    'ö' => "%C3%B6", 
    'Å' => "%C3%85",
    'å' => "%C3%A5",
    ',' => "%2C"
};



 

static LONGEST_ESCAPE: u8 = 6;
 

fn main() {
    let encoded = encode(&String::from("äiti on ötökkä åååå"));
    println!("{}", encoded);
    let decoded = decode(&encoded);
    println!("{}", decoded);
}

fn encode(target: &String) -> String {
    let mut output: String = String::new();

    'outer:for i in target.chars() {
        let escape = CHAR_TABLE.get(&i);
        match escape {
            Some(escape) => output.push_str(escape),
            None => {
                if !i.is_ascii() {
                    panic!("Unknown char: {}", i);
                } else {
                    output.push(i);
                }
            }
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
               mode = normal_scan(c, &mut output, &mut buffer);
           },
           ScanMode::Escape => {
               mode = escape_scan(c, &mut output, &mut buffer);
           }
       }
    }
    if mode == ScanMode::Escape {
        panic!("Unknown escape: {}", buffer)
    }
    output
}

fn normal_scan(c: char, output: &mut String, buffer: &mut String) -> ScanMode {
    match c {
        '%' => {
            buffer.push('%');
            ScanMode::Escape
        },
        _ => {
            output.push(c);
            ScanMode::Normal
        }    
    }
}

fn escape_scan(c: char, output: &mut String, buffer: &mut String) -> ScanMode {
    buffer.push(c);

    if buffer.len() as u8 > LONGEST_ESCAPE {
        panic!("EVERYTHING IS ON FIRE: {}", buffer);
    }

    let chara = ESCAPE_TABLE.get(&buffer[..]);
    match chara {
        Some(chara) => {
            output.push(*chara);
            buffer.clear();
            ScanMode::Normal
        }    
        None => ScanMode::Escape,
    }     
} 
