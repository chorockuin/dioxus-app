#[derive(Debug, PartialEq, Eq, Copy, Clone)]
pub enum Name {
    Mok,
    Hwa,
    To,
    Gum,
    Soo
}

#[derive(Debug)]
pub struct Haeng {
    name: Name,
    pub character: String,
    pub color: String,
}

pub fn create_ohaeng() -> Vec<Haeng> {
    let mut ohaeng = Vec::new();
    ohaeng.push(Haeng{
        name: Name::Mok,
        character: "木".to_string(),
        color: "forestgreen".to_string()});
    ohaeng.push(Haeng{
        name: Name::Hwa,
        character: "火".to_string(),
        color: "red".to_string()});
    ohaeng.push(Haeng{
        name: Name::To,
        character: "土".to_string(),
        color: "sandybrown".to_string()});
    ohaeng.push(Haeng{
        name: Name::Gum,
        character: "金".to_string(),
        color: "white".to_string()});
    ohaeng.push(Haeng{
        name: Name::Soo,
        character: "水".to_string(),
        color: "black".to_string()});
    ohaeng
}

pub fn get_saeng(ohaeng: &Vec<Haeng>, haeng_name: Name) -> Option<Name> {
    for (i, haeng) in ohaeng.iter().enumerate() {
        if haeng.name == haeng_name {
            return Some(ohaeng[(i+1)%ohaeng.len()].name);
        }
    }
    None
}

pub fn get_kuk(ohaeng: &Vec<Haeng>, haeng_name: Name) -> Option<Name> {
    for (i, haeng) in ohaeng.iter().enumerate() {
        if haeng.name == haeng_name {
            return Some(ohaeng[(i+2)%ohaeng.len()].name);
        }
    }
    None
}

pub fn get_haeng(ohaeng: &Vec<Haeng>, haeng_name: Name) -> Option<&Haeng> {
    for haeng in ohaeng.iter() {
        if haeng.name == haeng_name {
            return Some(haeng);
        }
    }
    None
}
