#[derive(Debug, PartialEq)]
pub enum Id {
    Tree,
    Fire,
    Soil,
    Metal,
    Water
}

#[derive(Debug)]
pub struct Elements {
    pub id: Id
}

pub fn get_attack_target(id: Id) -> Id {
    match id {
        Id::Tree => Id::Soil,
        Id::Fire => Id::Metal,
        Id::Soil => Id::Water,
        Id::Metal => Id::Tree,
        Id::Water => Id::Fire
    }
}