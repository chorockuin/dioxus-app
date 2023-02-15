use std::rc::Rc;
use std::cell::{Ref, RefCell};
use std::collections::HashMap;
use crate::umyang;
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
    pub haeng_name: ohaeng::Name,
    wol: u8,
    next: Option<Rc<RefCell<Jijija>>>
}

pub fn create_jiji(ohaeng: &HashMap<ohaeng::Name, Rc<RefCell<ohaeng::Haeng>>>) -> HashMap<Name, Rc<RefCell<Jijija>>> {
    let chook = Rc::new(RefCell::new(Jijija{
        name: Name::Chook,
        character: "丑".to_string(),
        umyang_name: umyang::Name::Um,
        haeng_name: ohaeng::Name::To,
        wol: 12,
        next: None}));
    let ja = Rc::new(RefCell::new(Jijija{
        name: Name::Ja,
        character: "子".to_string(),
        umyang_name: umyang::Name::Yang,
        haeng_name: ohaeng::Name::Soo,
        wol: 11,
        next: Some(Rc::clone(&chook))}));
    let hae = Rc::new(RefCell::new(Jijija{
        name: Name::Hae,
        character: "亥".to_string(),
        umyang_name: umyang::Name::Um,
        haeng_name: ohaeng::Name::Soo,
        wol: 10,
        next: Some(Rc::clone(&ja))}));
    let sool = Rc::new(RefCell::new(Jijija{
        name: Name::Sool,
        character: "戌".to_string(),
        umyang_name: umyang::Name::Yang,
        haeng_name: ohaeng::Name::To,
        wol: 9,
        next: Some(Rc::clone(&hae))}));
    let yoo = Rc::new(RefCell::new(Jijija{
        name: Name::Yoo,
        character: "酉".to_string(),
        umyang_name: umyang::Name::Um,
        haeng_name: ohaeng::Name::Gum,
        wol: 8,
        next: Some(Rc::clone(&sool))}));
    let sin = Rc::new(RefCell::new(Jijija{
        name: Name::Sin,
        character: "辛".to_string(),
        umyang_name: umyang::Name::Yang,
        haeng_name: ohaeng::Name::Gum,
        wol: 7,
        next: Some(Rc::clone(&yoo))}));
    let mi = Rc::new(RefCell::new(Jijija{
        name: Name::Mi,
        character: "未".to_string(),
        umyang_name: umyang::Name::Um,
        haeng_name: ohaeng::Name::To,
        wol: 6,
        next: Some(Rc::clone(&sin))}));
    let o = Rc::new(RefCell::new(Jijija{
        name:Name::O,
        character: "午".to_string(),
        umyang_name: umyang::Name::Yang,
        haeng_name: ohaeng::Name::Hwa,
        wol: 5,
        next: Some(Rc::clone(&mi))}));
    let sa = Rc::new(RefCell::new(Jijija{
        name: Name::Sa, 
        character: "巳".to_string(),
        umyang_name: umyang::Name::Um,
        haeng_name: ohaeng::Name::Hwa,
        wol: 4,
        next: Some(Rc::clone(&o))}));
    let jin = Rc::new(RefCell::new(Jijija{
        name: Name::Jin,
        character: "辰".to_string(),
        umyang_name: umyang::Name::Yang,
        haeng_name: ohaeng::Name::To,
        wol: 3,
        next: Some(Rc::clone(&sa))}));
    let myo = Rc::new(RefCell::new(Jijija{
        name: Name::Myo,
        character: "卯".to_string(),
        umyang_name: umyang::Name::Um,
        haeng_name: ohaeng::Name::Mok,
        wol: 2,
        next: Some(Rc::clone(&jin))}));
    let yin = Rc::new(RefCell::new(Jijija{
        name: Name::Yin,
        character: "寅".to_string(),
        umyang_name: umyang::Name::Yang,
        haeng_name: ohaeng::Name::Mok,
        wol: 1,
        next: Some(Rc::clone(&myo))}));
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

pub fn get_next_jijija_name(jiji: &HashMap<Name, Rc<RefCell<Jijija>>>, jijija_name: Name) -> Name {
    let jijija = &jiji.get(&jijija_name).unwrap().borrow();
    let next_jijija = jijija.next.as_ref().unwrap().borrow();
    next_jijija.name
}

pub fn get_jijija(jiji: &HashMap<Name, Rc<RefCell<Jijija>>>, jijija_name: Name) -> Ref<Jijija> {
    jiji.get(&jijija_name).unwrap().borrow()
}

pub fn get_haeng_name(jiji: &HashMap<Name, Rc<RefCell<Jijija>>>, jijija_name: Name) -> ohaeng::Name {
    let jijija = &jiji.get(&jijija_name).unwrap().borrow();
    jijija.haeng_name
}

pub fn get_umyang(jiji: &HashMap<Name, Rc<RefCell<Jijija>>>, jijija_name: Name) -> umyang::Name {
    let jijija = &jiji.get(&jijija_name).unwrap().borrow();
    jijija.umyang_name
}

pub fn get_wol(jiji: &HashMap<Name, Rc<RefCell<Jijija>>>, jijija_name: Name) -> u8 {
    let jijija = &jiji.get(&jijija_name).unwrap().borrow();
    jijija.wol
}
