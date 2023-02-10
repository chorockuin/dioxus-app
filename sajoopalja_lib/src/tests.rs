use crate::ohaeng;
use crate::jiji;

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
    let jiji = jiji::create_jiji(&ohaeng);
    assert_eq!(jiji::get_next_jijija(&jiji, jiji::Name::Yin), jiji::Name::Myo);
    assert_eq!(jiji::get_next_jijija(&jiji, jiji::Name::Myo), jiji::Name::Jin);
    assert_eq!(jiji::get_next_jijija(&jiji, jiji::Name::Jin), jiji::Name::Sa);
    assert_eq!(jiji::get_next_jijija(&jiji, jiji::Name::Sa), jiji::Name::O);
    assert_eq!(jiji::get_next_jijija(&jiji, jiji::Name::O), jiji::Name::Mi);
    assert_eq!(jiji::get_next_jijija(&jiji, jiji::Name::Mi), jiji::Name::Sin);
    assert_eq!(jiji::get_next_jijija(&jiji, jiji::Name::Sin), jiji::Name::Yoo);
    assert_eq!(jiji::get_next_jijija(&jiji, jiji::Name::Yoo), jiji::Name::Sool);
    assert_eq!(jiji::get_next_jijija(&jiji, jiji::Name::Sool), jiji::Name::Hae);
    assert_eq!(jiji::get_next_jijija(&jiji, jiji::Name::Hae), jiji::Name::Ja);
    assert_eq!(jiji::get_next_jijija(&jiji, jiji::Name::Ja), jiji::Name::Chook);
    assert_eq!(jiji::get_next_jijija(&jiji, jiji::Name::Chook), jiji::Name::Yin);
}

#[test]
fn cheongan_haeng() {
    let ohaeng = ohaeng::create_ohaeng();
    let jiji = jiji::create_jiji(&ohaeng);
    assert_eq!(jiji::get_haeng(&jiji, jiji::Name::Yin), ohaeng::Name::Mok);
    assert_eq!(jiji::get_haeng(&jiji, jiji::Name::Myo), ohaeng::Name::Mok);
    assert_eq!(jiji::get_haeng(&jiji, jiji::Name::Jin), ohaeng::Name::To);
    assert_eq!(jiji::get_haeng(&jiji, jiji::Name::Sa), ohaeng::Name::Hwa);
    assert_eq!(jiji::get_haeng(&jiji, jiji::Name::O), ohaeng::Name::Hwa);
    assert_eq!(jiji::get_haeng(&jiji, jiji::Name::Mi), ohaeng::Name::To);
    assert_eq!(jiji::get_haeng(&jiji, jiji::Name::Sin), ohaeng::Name::Gum);
    assert_eq!(jiji::get_haeng(&jiji, jiji::Name::Yoo), ohaeng::Name::Gum);
    assert_eq!(jiji::get_haeng(&jiji, jiji::Name::Sool), ohaeng::Name::To);
    assert_eq!(jiji::get_haeng(&jiji, jiji::Name::Hae), ohaeng::Name::Soo);
    assert_eq!(jiji::get_haeng(&jiji, jiji::Name::Ja), ohaeng::Name::Soo);
    assert_eq!(jiji::get_haeng(&jiji, jiji::Name::Chook), ohaeng::Name::To);
}