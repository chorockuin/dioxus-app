use std::rc::Rc;
use std::cell::RefCell;
use std::collections::HashMap;
use crate::umyang;
use crate::wol;
use crate::ohaeng;

#[derive(Debug, PartialEq, Eq, Hash, Copy, Clone)]
pub enum Name {
    Yin,
    Myo,
    Jin,
    Sa,
    O,
    Mi,
    Sin,
    Yoo,
    Sool,
    Hae,
    Ja,
    Chook
}

#[derive(Debug)]
pub struct Jijija {
    name: Name,
    pub character: String,
    umyang_name: umyang::Name,
    haeng_name: ohaeng::Name,
    wol_name: wol::Name,
    next: Option<Rc<RefCell<Jijija>>>
}

pub fn create_jiji(ohaeng: &HashMap<ohaeng::Name, Rc<RefCell<ohaeng::Haeng>>>) -> HashMap<Name, Rc<RefCell<Jijija>>> {
    let chook = Rc::new(RefCell::new(Jijija{name: Name::Chook, character: "丑".to_string(), umyang_name: umyang::Name::Um, haeng_name: ohaeng::Name::To, wol_name: wol::Name::Sibyi, next: None}));
    let ja = Rc::new(RefCell::new(Jijija{name: Name::Ja, character: "子".to_string(), umyang_name: umyang::Name::Yang, haeng_name: ohaeng::Name::Soo, wol_name: wol::Name::Sibyil, next: Some(Rc::clone(&chook))}));
    let hae = Rc::new(RefCell::new(Jijija{name: Name::Hae, character: "亥".to_string(), umyang_name: umyang::Name::Um, haeng_name: ohaeng::Name::Soo, wol_name: wol::Name::Sib, next: Some(Rc::clone(&ja))}));
    let sool = Rc::new(RefCell::new(Jijija{name: Name::Sool, character: "戌".to_string(), umyang_name: umyang::Name::Yang, haeng_name: ohaeng::Name::To, wol_name: wol::Name::Goo, next: Some(Rc::clone(&hae))}));
    let yoo = Rc::new(RefCell::new(Jijija{name: Name::Yoo, character: "酉".to_string(), umyang_name: umyang::Name::Um, haeng_name: ohaeng::Name::Gum, wol_name: wol::Name::Pal, next: Some(Rc::clone(&sool))}));
    let sin = Rc::new(RefCell::new(Jijija{name: Name::Sin, character: "辛".to_string(), umyang_name: umyang::Name::Yang, haeng_name: ohaeng::Name::Gum, wol_name: wol::Name::Chil, next: Some(Rc::clone(&yoo))}));
    let mi = Rc::new(RefCell::new(Jijija{name: Name::Mi, character: "未".to_string(), umyang_name: umyang::Name::Um, haeng_name: ohaeng::Name::To, wol_name: wol::Name::Yook, next: Some(Rc::clone(&sin))}));
    let o = Rc::new(RefCell::new(Jijija{name: Name::O, character: "午".to_string(), umyang_name: umyang::Name::Yang, haeng_name: ohaeng::Name::Hwa, wol_name: wol::Name::O, next: Some(Rc::clone(&mi))}));
    let sa = Rc::new(RefCell::new(Jijija{name: Name::Sa, character: "巳".to_string(), umyang_name: umyang::Name::Um, haeng_name: ohaeng::Name::Hwa, wol_name: wol::Name::Sa, next: Some(Rc::clone(&o))}));
    let jin = Rc::new(RefCell::new(Jijija{name: Name::Jin, character: "辰".to_string(), umyang_name: umyang::Name::Yang, haeng_name: ohaeng::Name::To, wol_name: wol::Name::Sam, next: Some(Rc::clone(&sa))}));
    let myo = Rc::new(RefCell::new(Jijija{name: Name::Myo, character: "卯".to_string(), umyang_name: umyang::Name::Um, haeng_name: ohaeng::Name::Mok, wol_name: wol::Name::Yi, next: Some(Rc::clone(&jin))}));
    let yin = Rc::new(RefCell::new(Jijija{name: Name::Yin, character: "寅".to_string(), umyang_name: umyang::Name::Yang, haeng_name: ohaeng::Name::Mok, wol_name: wol::Name::Yil, next: Some(Rc::clone(&myo))}));
    chook.borrow_mut().next = Some(Rc::clone(&yin));

    let mut jiji = HashMap::new();
    jiji.insert(Name::Yin, yin);
    jiji.insert(Name::Myo, myo);
    jiji.insert(Name::Jin, jin);
    jiji.insert(Name::Sa, sa);
    jiji.insert(Name::O, o);
    jiji.insert(Name::Mi, mi);
    jiji.insert(Name::Sin, sin);
    jiji.insert(Name::Yoo, yoo);
    jiji.insert(Name::Sool, sool);
    jiji.insert(Name::Hae, hae);
    jiji.insert(Name::Ja, ja);
    jiji.insert(Name::Chook, chook);

    jiji
}

pub fn get_next_jijija(jiji: &HashMap<Name, Rc<RefCell<Jijija>>>, jijija_name: Name) -> Name {
    let jijija = &jiji.get(&jijija_name).unwrap().borrow();
    let next_jijija = jijija.next.as_ref().unwrap().borrow();
    next_jijija.name
}

pub fn get_haeng(jiji: &HashMap<Name, Rc<RefCell<Jijija>>>, jijija_name: Name) -> ohaeng::Name {
    let jijija = &jiji.get(&jijija_name).unwrap().borrow();
    jijija.haeng_name
}

pub fn get_umyang(jiji: &HashMap<Name, Rc<RefCell<Jijija>>>, jijija_name: Name) -> umyang::Name {
    let jijija = &jiji.get(&jijija_name).unwrap().borrow();
    jijija.umyang_name
}

pub fn get_wol(jiji: &HashMap<Name, Rc<RefCell<Jijija>>>, jijija_name: Name) -> wol::Name {
    let jijija = &jiji.get(&jijija_name).unwrap().borrow();
    jijija.wol_name
}
