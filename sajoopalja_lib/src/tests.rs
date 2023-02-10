use crate::ohaeng;
use crate::cheongan;

#[test]
fn saeng() {
    let ohaeng = ohaeng::create_ohaeng();
    assert_eq!(ohaeng::get_saeng(&ohaeng, ohaeng::Name::Mok), ohaeng::Name::Hwa);
    assert_eq!(ohaeng::get_saeng(&ohaeng, ohaeng::Name::Hwa), ohaeng::Name::To);
    assert_eq!(ohaeng::get_saeng(&ohaeng, ohaeng::Name::To), ohaeng::Name::Gum);
    assert_eq!(ohaeng::get_saeng(&ohaeng, ohaeng::Name::Gum), ohaeng::Name::Soo);
    assert_eq!(ohaeng::get_saeng(&ohaeng, ohaeng::Name::Soo), ohaeng::Name::Mok);
}

#[test]
fn kuk() {
    let ohaeng = ohaeng::create_ohaeng();
    assert_eq!(ohaeng::get_kuk(&ohaeng, ohaeng::Name::Mok), ohaeng::Name::To);
    assert_eq!(ohaeng::get_kuk(&ohaeng, ohaeng::Name::Hwa), ohaeng::Name::Gum);
    assert_eq!(ohaeng::get_kuk(&ohaeng, ohaeng::Name::To), ohaeng::Name::Soo);
    assert_eq!(ohaeng::get_kuk(&ohaeng, ohaeng::Name::Gum), ohaeng::Name::Mok);
    assert_eq!(ohaeng::get_kuk(&ohaeng, ohaeng::Name::Soo), ohaeng::Name::Hwa);
}

#[test]
fn cheongan_order() {
    let ohaeng = ohaeng::create_ohaeng();
    let cheongan = cheongan::create_cheongan(&ohaeng);
    assert_eq!(cheongan::get_next_cheonganja(&cheongan, cheongan::Name::Yin), cheongan::Name::Myo);
    assert_eq!(cheongan::get_next_cheonganja(&cheongan, cheongan::Name::Myo), cheongan::Name::Jin);
    assert_eq!(cheongan::get_next_cheonganja(&cheongan, cheongan::Name::Jin), cheongan::Name::Sa);
    assert_eq!(cheongan::get_next_cheonganja(&cheongan, cheongan::Name::Sa), cheongan::Name::O);
    assert_eq!(cheongan::get_next_cheonganja(&cheongan, cheongan::Name::O), cheongan::Name::Mi);
    assert_eq!(cheongan::get_next_cheonganja(&cheongan, cheongan::Name::Mi), cheongan::Name::Sin);
    assert_eq!(cheongan::get_next_cheonganja(&cheongan, cheongan::Name::Sin), cheongan::Name::Yoo);
    assert_eq!(cheongan::get_next_cheonganja(&cheongan, cheongan::Name::Yoo), cheongan::Name::Sool);
    assert_eq!(cheongan::get_next_cheonganja(&cheongan, cheongan::Name::Sool), cheongan::Name::Hae);
    assert_eq!(cheongan::get_next_cheonganja(&cheongan, cheongan::Name::Hae), cheongan::Name::Ja);
    assert_eq!(cheongan::get_next_cheonganja(&cheongan, cheongan::Name::Ja), cheongan::Name::Chook);
    assert_eq!(cheongan::get_next_cheonganja(&cheongan, cheongan::Name::Chook), cheongan::Name::Yin);
}

#[test]
fn cheongan_haeng() {
    let ohaeng = ohaeng::create_ohaeng();
    let cheongan = cheongan::create_cheongan(&ohaeng);
    assert_eq!(cheongan::get_haeng(&cheongan, cheongan::Name::Yin), ohaeng::Name::Mok);
    assert_eq!(cheongan::get_haeng(&cheongan, cheongan::Name::Myo), ohaeng::Name::Mok);
    assert_eq!(cheongan::get_haeng(&cheongan, cheongan::Name::Jin), ohaeng::Name::To);
    assert_eq!(cheongan::get_haeng(&cheongan, cheongan::Name::Sa), ohaeng::Name::Hwa);
    assert_eq!(cheongan::get_haeng(&cheongan, cheongan::Name::O), ohaeng::Name::Hwa);
    assert_eq!(cheongan::get_haeng(&cheongan, cheongan::Name::Mi), ohaeng::Name::To);
    assert_eq!(cheongan::get_haeng(&cheongan, cheongan::Name::Sin), ohaeng::Name::Gum);
    assert_eq!(cheongan::get_haeng(&cheongan, cheongan::Name::Yoo), ohaeng::Name::Gum);
    assert_eq!(cheongan::get_haeng(&cheongan, cheongan::Name::Sool), ohaeng::Name::To);
    assert_eq!(cheongan::get_haeng(&cheongan, cheongan::Name::Hae), ohaeng::Name::Soo);
    assert_eq!(cheongan::get_haeng(&cheongan, cheongan::Name::Ja), ohaeng::Name::Soo);
    assert_eq!(cheongan::get_haeng(&cheongan, cheongan::Name::Chook), ohaeng::Name::To);
}