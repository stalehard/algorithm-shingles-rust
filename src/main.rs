extern crate regex;
extern crate crypto;

use std::thread;
use std::cmp::min;
use regex::Regex;
use crypto::md5::Md5;
use crypto::digest::Digest;

fn main() {
//    let res = shingles("sdsdsd".to_string(), "sdsdsd".to_string());
//    println!("{}", res);

    let len = 2;
    let text1 = "Сегодня, 27 октября, на сессии Госсовета Удмуртии мандат Дмитрия Кулишова, который сложил полномочия для того, чтобы перейти в Городскую Думу Ижевска, был передан Геннадию Зарубежнову – «зарегистрированному кандидату из республиканского списка кандидатов выдвинутого избирательным объединением удмуртское региональное отделение политической партии ЛДПР». По данным ЦИК УР, Геннадий Зарубежнов (1962 г.р.) является преподавателем ОУ ССПО «Нефтяной техникум», а также депутатов Совета депутатов Завьяловского района на непостоянной основе. На сайте Завьяловского района также указано, что он работает заместителем директора группы компаний «Прогресс».";
    let text2 = "Геннадий Зарубежнов стал депутатом Госсовета Удмуртии. Он получил мандат, освободившийся после перехода руководителя фракции ЛДПР Дмитрия Кулишова в Гордуму Ижевска. Зарубежнов шёл по спискам ЛДПР на выборах 2012 года. Сегодня на сессии республиканского парламента ему вручили удостоверение и знак депутата. Геннадий Зарубежнов занимает пост директора фирмы Прогресс, до недавнего времени являлся депутатом совета депутатов Завьяловского района";

    let n = get_same(&text1, &text2, len);

//    let test = " глава Фото прессслужбы главы правительства";
//    let bytes: &[u8] = test.as_bytes();
//    let mut hash = Md5::new();
//    hash.input(bytes);
//    println!("{}", hash.result_str());

    println!("{}", n);
}

fn get_same(x: &str, y: &str, len: usize) -> usize {
    let mut same = 0;

    let res1 = shingles(x, len);
    let res2 = shingles(y, len);

//    for item in &res1 {
//        println!("{}", *item);
//    }
//
//    println!("_____________________");
//
//    for item in &res2 {
//        println!("{}", *item);
//    }

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

    for item in &str {
        println!("{}", *item);
    }
    println!("_____________________");

    let handles: Vec<_> = str.into_iter().map(|s| {
        thread::spawn(move || {
            let bytes: &[u8] = s.as_bytes();
            let mut hash = Md5::new();
            hash.input(bytes);
            hash.result_str()
//            crc32::checksum_ieee(bytes)
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
    let stopWords = Regex::new(r"(?i)\b[а-я]{1,2}\b").unwrap();
//    let proposal = Regex::new(r"(?i)[a-ля|без|ведома|безо|благодаря|[в|по]?близ[ко|и]?|[в]?виде|течение|ввиду|вглубь|вдогонку|вдоль|взамен|включая|вкруг|вместо|вне|внизу|внутр[и|ь]|вовнутрь|вне|внизу|внутр[и|ь]|вовнутрь|возле|вокруг|вопреки|вослед|впереди|вплоть|впредь|вразрез|вроде|вслед[ствие]?|для|за[место]?|вычетом|исключением|счёт|из[-за|-под|нутри|о]?|исходя|к[асательно|о|роме|ругом]?|меж[ду]?|мимо|на[верху|вроде|встречу|д|до|зад|кануне|перекор|перерез|подобие|против|ряду|супротив|счёт|чиная]?|не[смотря]?|ниже|обо[к]?|около|ото|относительно|перед[о]|по-[за|над|под]?|по[близости|верх|д|дле|до|добно|зади|зднее|мимо|перёд|перёк|рядка|середине|середь|сле|среди|средине|средством]?|пред[о]?|пр[и|о|отив]?|ради|рядом|сверх[у]?|свыше|середь|сзади|сквозь|снизу|согласно|спустя|сред[и|ь]?|сродни|судя|супротив|ч[е]?рез]").unwrap();

    let mut temp = html.replace_all(&text, " ");
    temp = stopWords.replace_all(&temp, " ");
    temp
}