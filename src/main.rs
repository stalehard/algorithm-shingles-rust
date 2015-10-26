extern crate regex;
extern crate crc;

use std::thread;
use regex::Regex;
use crc::{crc32, Hasher32};

fn main() {
//    let res = shingles("sdsdsd".to_string(), "sdsdsd".to_string());
//    println!("{}", res);

    let len = 10;
    let text1 = "<p><hc id=\"hc_select_index44\" class=\"hc_select_index\"></hc> По решению суда детский сад закрыт на 20 суток.</p> Они выявили, что для внутренней отделки стен  в группе использовались обои, не допускающие проведение влажной уборки и дезинфекции; стены в мясо-рыбном цехе пищеблока имеют отделку, не позволяющую проводить дезинфекцию, осыпается штукатурка с краской; на потолках подтеки; полы из деревянных досок неровный, с щелями, что способствует проникновению грызунов и насекомых; стекла на окнах с трещинами, заклеены скотчем.";
    let text2 = "<p><hc id=\"hc_select_index44\" class=\"hc_select_index\"></hc> По решению суда детский сад закрыт на 20 суток.</p> Они выявили, что для внутренней отделки стен  в группе использовались обои, не допускающие проведение влажной уборки и дезинфекции; стены в мясо-рыбном цехе пищеблока имеют отделку, не позволяющую проводить дезинфекцию, осыпается штукатурка с краской; на потолках подтеки; полы из деревянных досок неровный, с щелями, что способствует проникновению грызунов и насекомых; стекла на окнах с трещинами, заклеены скотчем.";
//    let res1 = canonisation(text1);
//    let res2 = canonisation(text2);

//    let v1: Vec<&str> = res1.split_whitespace().collect();


    let n = get_same(&text1, &text2, len);


    println!("{}", n);
}

fn get_same(x: &str, y: &str, len: usize) -> usize {
    let same = 0;

    let res1 = shingles(x, len);
    let res2 = shingles(y, len);

    for r in res1 {
        println!("{}", r);
    }
    for r in res2 {
        println!("{}", r);
    }
    1
}

fn shingles(x: &str, len: usize) -> Vec<u32> {
    let mut text = String::from(x);
    text = canonisation(text);
    let splited: Vec<&str> = text.split_whitespace().collect();

    let mut str: Vec<String> = Vec::new();
    for i in 0..(splited.len()-len+1) {
        let mut buf = String::new();
        for y in i..i+len {
            buf = buf + " " + splited[y];
        }

        str.push(buf);
    }

    let handles: Vec<_> = str.into_iter().map(|s| {
        thread::spawn(move || {
            let bytes: &[u8] = s.as_bytes();
            crc32::checksum_ieee(bytes)
        })
    }).collect();

    let mut res: Vec<u32> = Vec::new();
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

    let mut temp = html.replace_all(&text, "");
    temp = stopWords.replace_all(&temp, "");
    temp
}






