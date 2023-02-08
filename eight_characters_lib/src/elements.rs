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

pub fn get_help_target(id: Id) -> Id {
    match id {
        Id::Tree => Id::Fire,
        Id::Fire => Id::Soil,
        Id::Soil => Id::Metal,
        Id::Metal => Id::Water,
        Id::Water => Id::Tree
    }
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

