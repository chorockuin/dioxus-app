use crate::haeng;

#[test]
fn saeng() {
    let o_haeng = haeng::create_o_haeng();
    assert_eq!(haeng::get_saeng(&o_haeng, haeng::Name::Mok), haeng::Name::Hwa);
    assert_eq!(haeng::get_saeng(&o_haeng, haeng::Name::Hwa), haeng::Name::To);
    assert_eq!(haeng::get_saeng(&o_haeng, haeng::Name::To), haeng::Name::Kum);
    assert_eq!(haeng::get_saeng(&o_haeng, haeng::Name::Kum), haeng::Name::Soo);
    assert_eq!(haeng::get_saeng(&o_haeng, haeng::Name::Soo), haeng::Name::Mok);
}

#[test]
fn kuk() {
    let o_haeng = haeng::create_o_haeng();
    assert_eq!(haeng::get_kuk(&o_haeng, haeng::Name::Mok), haeng::Name::To);
    assert_eq!(haeng::get_kuk(&o_haeng, haeng::Name::Hwa), haeng::Name::Kum);
    assert_eq!(haeng::get_kuk(&o_haeng, haeng::Name::To), haeng::Name::Soo);
    assert_eq!(haeng::get_kuk(&o_haeng, haeng::Name::Kum), haeng::Name::Mok);
    assert_eq!(haeng::get_kuk(&o_haeng, haeng::Name::Soo), haeng::Name::Hwa);
}
