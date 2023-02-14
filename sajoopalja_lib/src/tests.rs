use crate::umyang;
use crate::wol;
use crate::ohaeng;
use crate::jiji;

#[test]
fn ohaeng_saeng() {
    let ohaeng = ohaeng::create_ohaeng();
    assert_eq!(ohaeng::get_saeng(&ohaeng, ohaeng::Name::Mok), ohaeng::Name::Hwa);
    assert_eq!(ohaeng::get_saeng(&ohaeng, ohaeng::Name::Hwa), ohaeng::Name::To);
    assert_eq!(ohaeng::get_saeng(&ohaeng, ohaeng::Name::To), ohaeng::Name::Gum);
    assert_eq!(ohaeng::get_saeng(&ohaeng, ohaeng::Name::Gum), ohaeng::Name::Soo);
    assert_eq!(ohaeng::get_saeng(&ohaeng, ohaeng::Name::Soo), ohaeng::Name::Mok);
}

#[test]
fn ohaeng_kuk() {
    let ohaeng = ohaeng::create_ohaeng();
    assert_eq!(ohaeng::get_kuk(&ohaeng, ohaeng::Name::Mok), ohaeng::Name::To);
    assert_eq!(ohaeng::get_kuk(&ohaeng, ohaeng::Name::Hwa), ohaeng::Name::Gum);
    assert_eq!(ohaeng::get_kuk(&ohaeng, ohaeng::Name::To), ohaeng::Name::Soo);
    assert_eq!(ohaeng::get_kuk(&ohaeng, ohaeng::Name::Gum), ohaeng::Name::Mok);
    assert_eq!(ohaeng::get_kuk(&ohaeng, ohaeng::Name::Soo), ohaeng::Name::Hwa);
}

#[test]
fn jiji_order() {
    let ohaeng = ohaeng::create_ohaeng();
    let jiji = jiji::create_jiji(&ohaeng);
    assert_eq!(jiji::get_next_jijija_name(&jiji, jiji::Name::Yin), jiji::Name::Myo);
    assert_eq!(jiji::get_next_jijija_name(&jiji, jiji::Name::Myo), jiji::Name::Jin);
    assert_eq!(jiji::get_next_jijija_name(&jiji, jiji::Name::Jin), jiji::Name::Sa);
    assert_eq!(jiji::get_next_jijija_name(&jiji, jiji::Name::Sa), jiji::Name::O);
    assert_eq!(jiji::get_next_jijija_name(&jiji, jiji::Name::O), jiji::Name::Mi);
    assert_eq!(jiji::get_next_jijija_name(&jiji, jiji::Name::Mi), jiji::Name::Sin);
    assert_eq!(jiji::get_next_jijija_name(&jiji, jiji::Name::Sin), jiji::Name::Yoo);
    assert_eq!(jiji::get_next_jijija_name(&jiji, jiji::Name::Yoo), jiji::Name::Sool);
    assert_eq!(jiji::get_next_jijija_name(&jiji, jiji::Name::Sool), jiji::Name::Hae);
    assert_eq!(jiji::get_next_jijija_name(&jiji, jiji::Name::Hae), jiji::Name::Ja);
    assert_eq!(jiji::get_next_jijija_name(&jiji, jiji::Name::Ja), jiji::Name::Chook);
    assert_eq!(jiji::get_next_jijija_name(&jiji, jiji::Name::Chook), jiji::Name::Yin);
}

#[test]
fn jiji_umyang() {
    let ohaeng = ohaeng::create_ohaeng();
    let jiji = jiji::create_jiji(&ohaeng);
    assert_eq!(jiji::get_umyang(&jiji, jiji::Name::Yin), umyang::Name::Yang);
    assert_eq!(jiji::get_umyang(&jiji, jiji::Name::Myo), umyang::Name::Um);
    assert_eq!(jiji::get_umyang(&jiji, jiji::Name::Jin), umyang::Name::Yang);
    assert_eq!(jiji::get_umyang(&jiji, jiji::Name::Sa), umyang::Name::Um);
    assert_eq!(jiji::get_umyang(&jiji, jiji::Name::O), umyang::Name::Yang);
    assert_eq!(jiji::get_umyang(&jiji, jiji::Name::Mi), umyang::Name::Um);
    assert_eq!(jiji::get_umyang(&jiji, jiji::Name::Sin), umyang::Name::Yang);
    assert_eq!(jiji::get_umyang(&jiji, jiji::Name::Yoo), umyang::Name::Um);
    assert_eq!(jiji::get_umyang(&jiji, jiji::Name::Sool), umyang::Name::Yang);
    assert_eq!(jiji::get_umyang(&jiji, jiji::Name::Hae), umyang::Name::Um);
    assert_eq!(jiji::get_umyang(&jiji, jiji::Name::Ja), umyang::Name::Yang);
    assert_eq!(jiji::get_umyang(&jiji, jiji::Name::Chook), umyang::Name::Um);
}

#[test]
fn jiji_haeng_name() {
    let ohaeng = ohaeng::create_ohaeng();
    let jiji = jiji::create_jiji(&ohaeng);
    assert_eq!(jiji::get_haeng_name(&jiji, jiji::Name::Yin), ohaeng::Name::Mok);
    assert_eq!(jiji::get_haeng_name(&jiji, jiji::Name::Myo), ohaeng::Name::Mok);
    assert_eq!(jiji::get_haeng_name(&jiji, jiji::Name::Jin), ohaeng::Name::To);
    assert_eq!(jiji::get_haeng_name(&jiji, jiji::Name::Sa), ohaeng::Name::Hwa);
    assert_eq!(jiji::get_haeng_name(&jiji, jiji::Name::O), ohaeng::Name::Hwa);
    assert_eq!(jiji::get_haeng_name(&jiji, jiji::Name::Mi), ohaeng::Name::To);
    assert_eq!(jiji::get_haeng_name(&jiji, jiji::Name::Sin), ohaeng::Name::Gum);
    assert_eq!(jiji::get_haeng_name(&jiji, jiji::Name::Yoo), ohaeng::Name::Gum);
    assert_eq!(jiji::get_haeng_name(&jiji, jiji::Name::Sool), ohaeng::Name::To);
    assert_eq!(jiji::get_haeng_name(&jiji, jiji::Name::Hae), ohaeng::Name::Soo);
    assert_eq!(jiji::get_haeng_name(&jiji, jiji::Name::Ja), ohaeng::Name::Soo);
    assert_eq!(jiji::get_haeng_name(&jiji, jiji::Name::Chook), ohaeng::Name::To);
}

#[test]
fn jiji_wol() {
    let ohaeng = ohaeng::create_ohaeng();
    let jiji = jiji::create_jiji(&ohaeng);
    assert_eq!(jiji::get_wol(&jiji, jiji::Name::Yin), wol::Name::Yil);
    assert_eq!(jiji::get_wol(&jiji, jiji::Name::Myo), wol::Name::Yi);
    assert_eq!(jiji::get_wol(&jiji, jiji::Name::Jin), wol::Name::Sam);
    assert_eq!(jiji::get_wol(&jiji, jiji::Name::Sa), wol::Name::Sa);
    assert_eq!(jiji::get_wol(&jiji, jiji::Name::O), wol::Name::O);
    assert_eq!(jiji::get_wol(&jiji, jiji::Name::Mi), wol::Name::Yook);
    assert_eq!(jiji::get_wol(&jiji, jiji::Name::Sin), wol::Name::Chil);
    assert_eq!(jiji::get_wol(&jiji, jiji::Name::Yoo), wol::Name::Pal);
    assert_eq!(jiji::get_wol(&jiji, jiji::Name::Sool), wol::Name::Goo);
    assert_eq!(jiji::get_wol(&jiji, jiji::Name::Hae), wol::Name::Sib);
    assert_eq!(jiji::get_wol(&jiji, jiji::Name::Ja), wol::Name::Sibyil);
    assert_eq!(jiji::get_wol(&jiji, jiji::Name::Chook), wol::Name::Sibyi);
}