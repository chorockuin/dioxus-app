use crate::umyang;
use crate::ohaeng;
use crate::jiji;

#[test]
fn ohaeng_saeng() {
    let ohaeng = ohaeng::create_ohaeng();
    assert_eq!(ohaeng::get_saeng(&ohaeng, ohaeng::Name::Mok), Some(ohaeng::Name::Hwa));
    assert_eq!(ohaeng::get_saeng(&ohaeng, ohaeng::Name::Hwa), Some(ohaeng::Name::To));
    assert_eq!(ohaeng::get_saeng(&ohaeng, ohaeng::Name::To), Some(ohaeng::Name::Gum));
    assert_eq!(ohaeng::get_saeng(&ohaeng, ohaeng::Name::Gum), Some(ohaeng::Name::Soo));
    assert_eq!(ohaeng::get_saeng(&ohaeng, ohaeng::Name::Soo), Some(ohaeng::Name::Mok));
}

#[test]
fn ohaeng_kuk() {
    let ohaeng = ohaeng::create_ohaeng();
    assert_eq!(ohaeng::get_kuk(&ohaeng, ohaeng::Name::Mok), Some(ohaeng::Name::To));
    assert_eq!(ohaeng::get_kuk(&ohaeng, ohaeng::Name::Hwa), Some(ohaeng::Name::Gum));
    assert_eq!(ohaeng::get_kuk(&ohaeng, ohaeng::Name::To), Some(ohaeng::Name::Soo));
    assert_eq!(ohaeng::get_kuk(&ohaeng, ohaeng::Name::Gum), Some(ohaeng::Name::Mok));
    assert_eq!(ohaeng::get_kuk(&ohaeng, ohaeng::Name::Soo), Some(ohaeng::Name::Hwa));
}

#[test]
fn jiji_order() {
    let ohaeng = ohaeng::create_ohaeng();
    let jiji = jiji::create_jiji(&ohaeng);
    assert_eq!(jiji::get_next_jijija_name(&jiji, jiji::Name::Yin), Some(jiji::Name::Myo));
    assert_eq!(jiji::get_next_jijija_name(&jiji, jiji::Name::Myo), Some(jiji::Name::Jin));
    assert_eq!(jiji::get_next_jijija_name(&jiji, jiji::Name::Jin), Some(jiji::Name::Sa));
    assert_eq!(jiji::get_next_jijija_name(&jiji, jiji::Name::Sa), Some(jiji::Name::O));
    assert_eq!(jiji::get_next_jijija_name(&jiji, jiji::Name::O), Some(jiji::Name::Mi));
    assert_eq!(jiji::get_next_jijija_name(&jiji, jiji::Name::Mi), Some(jiji::Name::Sin));
    assert_eq!(jiji::get_next_jijija_name(&jiji, jiji::Name::Sin), Some(jiji::Name::Yoo));
    assert_eq!(jiji::get_next_jijija_name(&jiji, jiji::Name::Yoo), Some(jiji::Name::Sool));
    assert_eq!(jiji::get_next_jijija_name(&jiji, jiji::Name::Sool), Some(jiji::Name::Hae));
    assert_eq!(jiji::get_next_jijija_name(&jiji, jiji::Name::Hae), Some(jiji::Name::Ja));
    assert_eq!(jiji::get_next_jijija_name(&jiji, jiji::Name::Ja), Some(jiji::Name::Chook));
    assert_eq!(jiji::get_next_jijija_name(&jiji, jiji::Name::Chook), Some(jiji::Name::Yin));
}

#[test]
fn jiji_umyang() {
    let ohaeng = ohaeng::create_ohaeng();
    let jiji = jiji::create_jiji(&ohaeng);
    assert_eq!(jiji::get_jijija(&jiji, jiji::Name::Yin).unwrap().umyang_name, umyang::Name::Yang);
    assert_eq!(jiji::get_jijija(&jiji, jiji::Name::Myo).unwrap().umyang_name, umyang::Name::Um);
    assert_eq!(jiji::get_jijija(&jiji, jiji::Name::Jin).unwrap().umyang_name, umyang::Name::Yang);
    assert_eq!(jiji::get_jijija(&jiji, jiji::Name::Sa).unwrap().umyang_name, umyang::Name::Um);
    assert_eq!(jiji::get_jijija(&jiji, jiji::Name::O).unwrap().umyang_name, umyang::Name::Yang);
    assert_eq!(jiji::get_jijija(&jiji, jiji::Name::Mi).unwrap().umyang_name, umyang::Name::Um);
    assert_eq!(jiji::get_jijija(&jiji, jiji::Name::Sin).unwrap().umyang_name, umyang::Name::Yang);
    assert_eq!(jiji::get_jijija(&jiji, jiji::Name::Yoo).unwrap().umyang_name, umyang::Name::Um);
    assert_eq!(jiji::get_jijija(&jiji, jiji::Name::Sool).unwrap().umyang_name, umyang::Name::Yang);
    assert_eq!(jiji::get_jijija(&jiji, jiji::Name::Hae).unwrap().umyang_name, umyang::Name::Um);
    assert_eq!(jiji::get_jijija(&jiji, jiji::Name::Ja).unwrap().umyang_name, umyang::Name::Yang);
    assert_eq!(jiji::get_jijija(&jiji, jiji::Name::Chook).unwrap().umyang_name, umyang::Name::Um);
}

#[test]
fn jiji_haeng_name() {
    let ohaeng = ohaeng::create_ohaeng();
    let jiji = jiji::create_jiji(&ohaeng);
    assert_eq!(jiji::get_jijija(&jiji, jiji::Name::Yin).unwrap().haeng_name, ohaeng::Name::Mok);
    assert_eq!(jiji::get_jijija(&jiji, jiji::Name::Myo).unwrap().haeng_name, ohaeng::Name::Mok);
    assert_eq!(jiji::get_jijija(&jiji, jiji::Name::Jin).unwrap().haeng_name, ohaeng::Name::To);
    assert_eq!(jiji::get_jijija(&jiji, jiji::Name::Sa).unwrap().haeng_name, ohaeng::Name::Hwa);
    assert_eq!(jiji::get_jijija(&jiji, jiji::Name::O).unwrap().haeng_name, ohaeng::Name::Hwa);
    assert_eq!(jiji::get_jijija(&jiji, jiji::Name::Mi).unwrap().haeng_name, ohaeng::Name::To);
    assert_eq!(jiji::get_jijija(&jiji, jiji::Name::Sin).unwrap().haeng_name, ohaeng::Name::Gum);
    assert_eq!(jiji::get_jijija(&jiji, jiji::Name::Yoo).unwrap().haeng_name, ohaeng::Name::Gum);
    assert_eq!(jiji::get_jijija(&jiji, jiji::Name::Sool).unwrap().haeng_name, ohaeng::Name::To);
    assert_eq!(jiji::get_jijija(&jiji, jiji::Name::Hae).unwrap().haeng_name, ohaeng::Name::Soo);
    assert_eq!(jiji::get_jijija(&jiji, jiji::Name::Ja).unwrap().haeng_name, ohaeng::Name::Soo);
    assert_eq!(jiji::get_jijija(&jiji, jiji::Name::Chook).unwrap().haeng_name, ohaeng::Name::To);
}

#[test]
fn jiji_wol() {
    let ohaeng = ohaeng::create_ohaeng();
    let jiji = jiji::create_jiji(&ohaeng);
    assert_eq!(jiji::get_jijija(&jiji, jiji::Name::Yin).unwrap().wol, 1);
    assert_eq!(jiji::get_jijija(&jiji, jiji::Name::Myo).unwrap().wol, 2);
    assert_eq!(jiji::get_jijija(&jiji, jiji::Name::Jin).unwrap().wol, 3);
    assert_eq!(jiji::get_jijija(&jiji, jiji::Name::Sa).unwrap().wol, 4);
    assert_eq!(jiji::get_jijija(&jiji, jiji::Name::O).unwrap().wol, 5);
    assert_eq!(jiji::get_jijija(&jiji, jiji::Name::Mi).unwrap().wol, 6);
    assert_eq!(jiji::get_jijija(&jiji, jiji::Name::Sin).unwrap().wol, 7);
    assert_eq!(jiji::get_jijija(&jiji, jiji::Name::Yoo).unwrap().wol, 8);
    assert_eq!(jiji::get_jijija(&jiji, jiji::Name::Sool).unwrap().wol, 9);
    assert_eq!(jiji::get_jijija(&jiji, jiji::Name::Hae).unwrap().wol, 10);
    assert_eq!(jiji::get_jijija(&jiji, jiji::Name::Ja).unwrap().wol, 11);
    assert_eq!(jiji::get_jijija(&jiji, jiji::Name::Chook).unwrap().wol, 12);
}