use crate::umyang;
use crate::ohaeng;

#[derive(Debug, PartialEq, Eq, Hash, Copy, Clone)]
pub enum Name {
    Gab,
    Ul,
    Byoung,
    Jung,
    Moo,
    Gi,
    Gyoung,
    Sin,
    Yim,
    Gye
}

#[derive(Debug)]
pub struct Cheonganja {
    name: Name,
    pub character: String,
    pub umyang_name: umyang::Name,
    pub haeng_name: ohaeng::Name,
}

pub fn create_cheongan() -> Vec<Cheonganja> {
    let mut cheongan = Vec::new();
    cheongan.push(Cheonganja{
        name: Name::Gab,
        character: "甲".to_string(),
        umyang_name: umyang::Name::Yang,
        haeng_name: ohaeng::Name::Mok});
    cheongan.push(Cheonganja{
        name: Name::Ul,
        character: "乙".to_string(),
        umyang_name: umyang::Name::Um,
        haeng_name: ohaeng::Name::Mok});
    cheongan.push(Cheonganja{
        name: Name::Byoung,
        character: "丙".to_string(),
        umyang_name: umyang::Name::Yang,
        haeng_name: ohaeng::Name::Hwa});
    cheongan.push(Cheonganja{
        name: Name::Jung,
        character: "丁".to_string(),
        umyang_name: umyang::Name::Um,
        haeng_name: ohaeng::Name::Hwa});
    cheongan.push(Cheonganja{
        name: Name::Moo,
        character: "戊".to_string(),
        umyang_name: umyang::Name::Yang,
        haeng_name: ohaeng::Name::To});
    cheongan.push(Cheonganja{
        name: Name::Gi,
        character: "己".to_string(),
        umyang_name: umyang::Name::Um,
        haeng_name: ohaeng::Name::To});
    cheongan.push(Cheonganja{
        name: Name::Gyoung,
        character: "庚".to_string(),
        umyang_name: umyang::Name::Yang,
        haeng_name: ohaeng::Name::Gum});
    cheongan.push(Cheonganja{
        name: Name::Sin,
        character: "辛".to_string(),
        umyang_name: umyang::Name::Um,
        haeng_name: ohaeng::Name::Gum});
    cheongan.push(Cheonganja{
        name: Name::Yim,
        character: "壬".to_string(),
        umyang_name: umyang::Name::Yang,
        haeng_name: ohaeng::Name::Soo});
    cheongan.push(Cheonganja{
        name: Name::Gye,
        character: "癸".to_string(),
        umyang_name: umyang::Name::Um,
        haeng_name: ohaeng::Name::Soo});
    cheongan
}

pub fn get_cheonganja(cheongan: &Vec<Cheonganja>, cheonganja_name: Name) -> Option<&Cheonganja> {
    for cheonganja in cheongan.iter() {
        if cheonganja.name == cheonganja_name {
            return Some(cheonganja);
        }
    }
    None
}
