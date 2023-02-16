use crate::umyang;
use crate::ohaeng;

#[derive(Debug, PartialEq, Eq, Hash, Copy, Clone)]
pub enum Name {
    Yin,
    Myo,
    Jin,
    Sa,
    O,
    Mi,
    Sin,
    Yoo,
    Sool,
    Hae,
    Ja,
    Chook
}

#[derive(Debug)]
pub struct Jijija {
    name: Name,
    pub character: String,
    pub umyang_name: umyang::Name,
    pub haeng_name: ohaeng::Name,
    pub wol: u8,
}

pub fn create_jiji(ohaeng: &Vec<ohaeng::Haeng>) -> Vec<Jijija> {
    let mut jiji = Vec::new();
    jiji.push(Jijija{
        name: Name::Yin,
        character: "寅".to_string(),
        umyang_name: umyang::Name::Yang,
        haeng_name: ohaeng::Name::Mok,
        wol: 1});
    jiji.push(Jijija{
        name: Name::Myo,
        character: "卯".to_string(),
        umyang_name: umyang::Name::Um,
        haeng_name: ohaeng::Name::Mok,
        wol: 2});
    jiji.push(Jijija{
        name: Name::Jin,
        character: "辰".to_string(),
        umyang_name: umyang::Name::Yang,
        haeng_name: ohaeng::Name::To,
        wol: 3});
    jiji.push(Jijija{
        name: Name::Sa, 
        character: "巳".to_string(),
        umyang_name: umyang::Name::Um,
        haeng_name: ohaeng::Name::Hwa,
        wol: 4});
    jiji.push(Jijija{
        name:Name::O,
        character: "午".to_string(),
        umyang_name: umyang::Name::Yang,
        haeng_name: ohaeng::Name::Hwa,
        wol: 5});
    jiji.push(Jijija{
        name: Name::Mi,
        character: "未".to_string(),
        umyang_name: umyang::Name::Um,
        haeng_name: ohaeng::Name::To,
        wol: 6});
    jiji.push(Jijija{
        name: Name::Sin,
        character: "辛".to_string(),
        umyang_name: umyang::Name::Yang,
        haeng_name: ohaeng::Name::Gum,
        wol: 7});
    jiji.push(Jijija{
        name: Name::Yoo,
        character: "酉".to_string(),
        umyang_name: umyang::Name::Um,
        haeng_name: ohaeng::Name::Gum,
        wol: 8});
    jiji.push(Jijija{
        name: Name::Sool,
        character: "戌".to_string(),
        umyang_name: umyang::Name::Yang,
        haeng_name: ohaeng::Name::To,
        wol: 9});
    jiji.push(Jijija{
        name: Name::Hae,
        character: "亥".to_string(),
        umyang_name: umyang::Name::Um,
        haeng_name: ohaeng::Name::Soo,
        wol: 10});
    jiji.push(Jijija{
        name: Name::Ja,
        character: "子".to_string(),
        umyang_name: umyang::Name::Yang,
        haeng_name: ohaeng::Name::Soo,
        wol: 11});
    jiji.push(Jijija{
        name: Name::Chook,
        character: "丑".to_string(),
        umyang_name: umyang::Name::Um,
        haeng_name: ohaeng::Name::To,
        wol: 12});
    jiji
}

pub fn get_next_jijija_name(jiji: &Vec<Jijija>, jijija_name: Name) -> Option<Name> {
    for (i, jijija) in jiji.iter().enumerate() {
        if jijija.name == jijija_name {
            return Some(jiji[(i+1)%jiji.len()].name);
        }
    }
    None
}

pub fn get_jijija(jiji: &Vec<Jijija>, jijija_name: Name) -> Option<&Jijija> {
    for jijija in jiji.iter() {
        if jijija.name == jijija_name {
            return Some(jijija);
        }
    }
    None
}
