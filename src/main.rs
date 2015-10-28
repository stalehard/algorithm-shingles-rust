extern crate regex;
extern crate crypto;

use std::thread;
use std::cmp::min;
use regex::Regex;
use crypto::md5::Md5;
use crypto::digest::Digest;

fn main() {
    let len = 2; // length of shingles
    let text1 = "Глава Удмуртской Республики Александр Соловьёв поздравил народную артистку Наталью Пугачеву с юбилеем, сообщает пресс-служба руководителя региона.

Сегодня, 28 марта, участнице всемирно известного творческого коллектива «Бурановские бабушки» Наталье Пугачёвой исполнилось 80 лет. Глава Удмуртии направил в её адрес поздравительную телеграмму.

«Трудно переоценить Ваш личный вклад в популяризацию удмуртской культуры. Ваш замечательный коллектив в 2012 году стал одним из победителей международного конкурса песни «Евровидение» и прославил нашу республику на весь мир. Ваша песня на удмуртском языке зазвучала во многих уголках России и Европы. Ваша обаятельная улыбка стала олицетворением образа российской бабушки. Вас полюбили миллионы почитателей песенного творчества», - говорится в поздравлении.

Глава пожелал бурановской бабушке новых творческих успехов, доброго здоровья на долгие годы, счастья и благополучия.";

    let text2 = "Глава Удмуртской Республики отправил самой улыбчивой и обаятельной бурановской бабушке Наталье Пугачёвой поздравительную телеграмму к юбилею. Об этом сообщает пресс-служба главы и правительства Удмуртии. Завтра, 28 октября, народной артистке исполняется 80 лет.

«Трудно переоценить Ваш личный вклад в популяризацию удмуртской культуры.

Ваш замечательный коллектив в 2012 году стал одним из победителей международного конкурса песни Евровидение и прославил нашу республику на весь мир. Ваша песня на удмуртском языке зазвучала во многих уголках России и Европы.

Ваша обаятельная улыбка стала олицетворением образа российской бабушки. Вас полюбили миллионы почитателей песенного творчества», — сказано в поздравлении Александра Соловьева.

Глава республики пожелал народной артистке Удмуртии новых творческих успехов, доброго здоровья на долгие годы, счастья и благополучия.";

    let n = get_same(&text1, &text2, len);
    println!("{}", n);
}

fn get_same(x: &str, y: &str, len: usize) -> usize {
    let mut same = 0;

    let res1 = shingles(x, len);
    let res2 = shingles(y, len);

    for item in &res1 {
        for el in &res2 {
            if(*item == *el) {
                same += 1;
            }
        }
    }
    same*100/min(res1.len(), res2.len())
}

fn shingles(x: &str, len: usize) -> Vec<String> {
    let mut text = String::from(x);
    text = canonisation(text);
    let splited: Vec<&str> = text.split_whitespace().collect();

    let mut str: Vec<String> = Vec::new();
    for i in 0..(splited.len()-len+1) {
        let mut buf = String::new();
        for y in i..i+len {
            buf = buf + " " + splited[y];
        }

        let el = String::from(buf.trim()).to_lowercase();
        str.push(el);
    }

    let handles: Vec<_> = str.into_iter().map(|s| {
        thread::spawn(move || {
            let bytes: &[u8] = s.as_bytes();
            let mut hash = Md5::new();
            hash.input(bytes);
            hash.result_str()
        })
    }).collect();

    let mut res: Vec<String> = Vec::new();
    for h in handles {
        match h.join() {
            Ok(r) => res.push(r),
            Err(err) => println!("{:?}", err),
        };
    }
    res
}

fn canonisation(text: String) -> String {
    let html = Regex::new(r"<[^>]*>|[:punct:]").unwrap();
    let stop_words = Regex::new(r"(?i)\b[а-я]{1,2}\b").unwrap();
    let mut temp = html.replace_all(&text, " ");
    temp = stop_words.replace_all(&temp, " ");
    temp
}