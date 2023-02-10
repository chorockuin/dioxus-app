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
pub struct Cheonganja {
    name: Name,
    haeng_name: ohaeng::Name,
    next: Option<Rc<RefCell<Cheonganja>>>
}

pub fn create_cheongan(ohaeng: &HashMap<ohaeng::Name, Rc<RefCell<ohaeng::Haeng>>>) -> HashMap<Name, Rc<RefCell<Cheonganja>>> {
    let chook = Rc::new(RefCell::new(Cheonganja{name: Name::Chook, haeng_name: ohaeng::Name::To, next: None}));
    let ja = Rc::new(RefCell::new(Cheonganja{name: Name::Ja, haeng_name: ohaeng::Name::Soo, next: Some(Rc::clone(&chook))}));
    let hae = Rc::new(RefCell::new(Cheonganja{name: Name::Hae, haeng_name: ohaeng::Name::Soo, next: Some(Rc::clone(&ja))}));
    let sool = Rc::new(RefCell::new(Cheonganja{name: Name::Sool, haeng_name: ohaeng::Name::To, next: Some(Rc::clone(&hae))}));
    let yoo = Rc::new(RefCell::new(Cheonganja{name: Name::Yoo, haeng_name: ohaeng::Name::Gum, next: Some(Rc::clone(&sool))}));
    let sin = Rc::new(RefCell::new(Cheonganja{name: Name::Sin, haeng_name: ohaeng::Name::Gum, next: Some(Rc::clone(&yoo))}));
    let mi = Rc::new(RefCell::new(Cheonganja{name: Name::Mi, haeng_name: ohaeng::Name::To, next: Some(Rc::clone(&sin))}));
    let o = Rc::new(RefCell::new(Cheonganja{name: Name::O, haeng_name: ohaeng::Name::Hwa, next: Some(Rc::clone(&mi))}));
    let sa = Rc::new(RefCell::new(Cheonganja{name: Name::Sa, haeng_name: ohaeng::Name::Hwa, next: Some(Rc::clone(&o))}));
    let jin = Rc::new(RefCell::new(Cheonganja{name: Name::Jin, haeng_name: ohaeng::Name::To, next: Some(Rc::clone(&sa))}));
    let myo = Rc::new(RefCell::new(Cheonganja{name: Name::Myo, haeng_name: ohaeng::Name::Mok, next: Some(Rc::clone(&jin))}));
    let yin = Rc::new(RefCell::new(Cheonganja{name: Name::Yin, haeng_name: ohaeng::Name::Mok, next: Some(Rc::clone(&myo))}));
    chook.borrow_mut().next = Some(Rc::clone(&yin));

    let mut cheongan = HashMap::new();
    cheongan.insert(Name::Yin, yin);
    cheongan.insert(Name::Myo, myo);
    cheongan.insert(Name::Jin, jin);
    cheongan.insert(Name::Sa, sa);
    cheongan.insert(Name::O, o);
    cheongan.insert(Name::Mi, mi);
    cheongan.insert(Name::Sin, sin);
    cheongan.insert(Name::Yoo, yoo);
    cheongan.insert(Name::Sool, sool);
    cheongan.insert(Name::Hae, hae);
    cheongan.insert(Name::Ja, ja);
    cheongan.insert(Name::Chook, chook);

    cheongan
}

pub fn get_next_cheonganja(cheongan: &HashMap<Name, Rc<RefCell<Cheonganja>>>, cheonganja_name: Name) -> Name {
    let cheonganja = &cheongan.get(&cheonganja_name).unwrap().borrow();
    let next_cheonganja = cheonganja.next.as_ref().unwrap().borrow();
    next_cheonganja.name
}

pub fn get_haeng(cheongan: &HashMap<Name, Rc<RefCell<Cheonganja>>>, cheonganja_name: Name) -> ohaeng::Name {
    let cheonganja = &cheongan.get(&cheonganja_name).unwrap().borrow();
    cheonganja.haeng_name
}
