use crate::haeng;

#[test]
fn saeng() {    
    assert_eq!(haeng::get_saeng(haeng::Name::Mok), haeng::Name::Hwa);
    assert_eq!(haeng::get_saeng(haeng::Name::Hwa), haeng::Name::To);
    assert_eq!(haeng::get_saeng(haeng::Name::To), haeng::Name::Kum);
    assert_eq!(haeng::get_saeng(haeng::Name::Kum), haeng::Name::Soo);
    assert_eq!(haeng::get_saeng(haeng::Name::Soo), haeng::Name::Mok);
}

#[test]
fn kuk() {
    assert_eq!(haeng::get_kuk(haeng::Name::Mok), haeng::Name::To);
    assert_eq!(haeng::get_kuk(haeng::Name::Hwa), haeng::Name::Kum);
    assert_eq!(haeng::get_kuk(haeng::Name::To), haeng::Name::Soo);
    assert_eq!(haeng::get_kuk(haeng::Name::Kum), haeng::Name::Mok);
    assert_eq!(haeng::get_kuk(haeng::Name::Soo), haeng::Name::Hwa);
}
