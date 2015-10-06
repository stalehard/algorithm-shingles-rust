extern crate regex;

use regex::Regex;

fn main() {
//    let res = shingles("sdsdsd".to_string(), "sdsdsd".to_string());
//    println!("{}", res);

    let text = "<p><hc id=\"hc_select_index42\" class=\"hc_select_index\"></hc>По информации ведомства, житель Ижевска, гражданин России Нико Чигладзе размещал в социальной сети «Вконтакте» доступные для широкого круга лиц материалы, содержащие высказывания о признании идеологии и практики терроризма правильными, нуждающимися в поддержке и подражании.</p>";
    let res = canonisation(text);
    println!("{}", res);
}

fn shingles(x: String, y: String) -> f32 {
    
    3.22
}


fn canonisation(text: &str) -> String {
    let re = Regex::new(r"<[^>]*>").unwrap();
    let res = re.replace_all(text, "");
    res
}