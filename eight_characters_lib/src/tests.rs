use crate::elements;

#[test]
fn help() {    
    assert_eq!(elements::get_help_target(elements::Id::Tree), elements::Id::Fire);
    assert_eq!(elements::get_help_target(elements::Id::Fire), elements::Id::Soil);
    assert_eq!(elements::get_help_target(elements::Id::Soil), elements::Id::Metal);
    assert_eq!(elements::get_help_target(elements::Id::Metal), elements::Id::Water);
    assert_eq!(elements::get_help_target(elements::Id::Water), elements::Id::Tree);
}

#[test]
fn attack() {
    assert_eq!(elements::get_attack_target(elements::Id::Tree), elements::Id::Soil);
    assert_eq!(elements::get_attack_target(elements::Id::Fire), elements::Id::Metal);
    assert_eq!(elements::get_attack_target(elements::Id::Soil), elements::Id::Water);
    assert_eq!(elements::get_attack_target(elements::Id::Metal), elements::Id::Tree);
    assert_eq!(elements::get_attack_target(elements::Id::Water), elements::Id::Fire);
}
