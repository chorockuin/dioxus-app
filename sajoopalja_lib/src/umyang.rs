#[derive(Debug, PartialEq, Eq, Hash, Copy, Clone)]
pub enum Name {
    Um,
    Yang
}

pub fn get_opacity(umyang_name: Name) -> f32 {
    match umyang_name {
        Name::Um => 0.5,
        Name::Yang => 1.0
    }
}
