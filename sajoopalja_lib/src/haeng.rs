#[derive(Debug, PartialEq)]
pub enum Name {
    Mok,
    Hwa,
    To,
    Kum,
    Soo
}

#[derive(Debug)]
pub struct Haeng {
    pub name: Name
}

pub fn get_saeng(name: Name) -> Name {
    match name {
        Name::Mok => Name::Hwa,
        Name::Hwa => Name::To,
        Name::To => Name::Kum,
        Name::Kum => Name::Soo,
        Name::Soo => Name::Mok
    }
}
pub fn get_kuk(name: Name) -> Name {
    match name {
        Name::Mok => Name::To,
        Name::Hwa => Name::Kum,
        Name::To => Name::Soo,
        Name::Kum => Name::Mok,
        Name::Soo => Name::Hwa
    }
}
