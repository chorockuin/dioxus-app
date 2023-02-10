use std::rc::Rc;
use std::cell::RefCell;
use std::collections::HashMap;
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
    haeng_name: ohaeng::Name,
    next: Option<Rc<RefCell<Jijija>>>
}

pub fn create_jiji(ohaeng: &HashMap<ohaeng::Name, Rc<RefCell<ohaeng::Haeng>>>) -> HashMap<Name, Rc<RefCell<Jijija>>> {
    let chook = Rc::new(RefCell::new(Jijija{name: Name::Chook, haeng_name: ohaeng::Name::To, next: None}));
    let ja = Rc::new(RefCell::new(Jijija{name: Name::Ja, haeng_name: ohaeng::Name::Soo, next: Some(Rc::clone(&chook))}));
    let hae = Rc::new(RefCell::new(Jijija{name: Name::Hae, haeng_name: ohaeng::Name::Soo, next: Some(Rc::clone(&ja))}));
    let sool = Rc::new(RefCell::new(Jijija{name: Name::Sool, haeng_name: ohaeng::Name::To, next: Some(Rc::clone(&hae))}));
    let yoo = Rc::new(RefCell::new(Jijija{name: Name::Yoo, haeng_name: ohaeng::Name::Gum, next: Some(Rc::clone(&sool))}));
    let sin = Rc::new(RefCell::new(Jijija{name: Name::Sin, haeng_name: ohaeng::Name::Gum, next: Some(Rc::clone(&yoo))}));
    let mi = Rc::new(RefCell::new(Jijija{name: Name::Mi, haeng_name: ohaeng::Name::To, next: Some(Rc::clone(&sin))}));
    let o = Rc::new(RefCell::new(Jijija{name: Name::O, haeng_name: ohaeng::Name::Hwa, next: Some(Rc::clone(&mi))}));
    let sa = Rc::new(RefCell::new(Jijija{name: Name::Sa, haeng_name: ohaeng::Name::Hwa, next: Some(Rc::clone(&o))}));
    let jin = Rc::new(RefCell::new(Jijija{name: Name::Jin, haeng_name: ohaeng::Name::To, next: Some(Rc::clone(&sa))}));
    let myo = Rc::new(RefCell::new(Jijija{name: Name::Myo, haeng_name: ohaeng::Name::Mok, next: Some(Rc::clone(&jin))}));
    let yin = Rc::new(RefCell::new(Jijija{name: Name::Yin, haeng_name: ohaeng::Name::Mok, next: Some(Rc::clone(&myo))}));
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
