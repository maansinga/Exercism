use std::iter;

pub fn encode(source: &str) -> String {
    if source.len() == 0 { "".to_owned() } else {
        let mut character = source.chars().next().unwrap();
        let mut running_count = 0;

        let mut string_template = "".to_owned();
        for c in source.chars() {
            if character == c { running_count += 1 } else {
                if running_count > 1{
                    string_template = format!("{}{}{}", string_template, running_count, character);
                }else{
                    string_template = format!("{}{}", string_template, character);
                }

                character = c;
                running_count = 1;
            }
        }
        if running_count > 1{
            string_template = format!("{}{}{}", string_template, running_count, character);
        }else{
            string_template = format!("{}{}", string_template, character);
        }

        string_template
    }
}

pub fn decode(source: &str) -> String {
    let mut count_build = 0usize;
    let mut string_template = String::new();
    for k in source.chars() {
        match k {
            '0'..='9' => { count_build = count_build * 10 + (k.to_digit(10).unwrap() as usize); }
            _ => {
                if count_build<2{
                    string_template = format!("{}{}",
                                              string_template, k);
                }else{
                    string_template = format!("{}{}",
                                              string_template,
                                              iter::repeat(k)
                                                  .take(count_build)
                                                  .fold(String::new(), |a, k| {
                                                      format!("{}{}", a, k)
                                                  })
                    );
                }
                count_build = 0;
            }
        };
    }

    string_template
}
