use std::rc::Rc;
use std::cell::RefCell;
use std::collections::HashMap;

#[derive(Debug, PartialEq, Eq, Hash)]
pub enum Name {
    Mok,
    Hwa,
    To,
    Kum,
    Soo
}

#[derive(Debug)]
pub struct Haeng {
    name: Name,
    next: Option<Rc<RefCell<Haeng>>>
}

pub fn create_o_haeng() -> HashMap<Name, Rc<RefCell<Haeng>>> {
    // Haeng instance를 RefCell로 하나 생성. RefCell을 참조하는 Rc instance도 하나 생성
    let soo = Rc::new(RefCell::new(Haeng{name: Name::Soo, next: None}));
    let kum = Rc::new(RefCell::new(Haeng{name: Name::Kum, next: Some(Rc::clone(&soo))}));
    let to = Rc::new(RefCell::new(Haeng{name: Name::To, next: Some(Rc::clone(&kum))}));
    let hwa = Rc::new(RefCell::new(Haeng{name: Name::Hwa, next: Some(Rc::clone(&to))}));
    let mok = Rc::new(RefCell::new(Haeng{name: Name::Mok, next: Some(Rc::clone(&hwa))}));
    // Rc instance 생성 후 변수(soo)에 assign 했지만, 해당 변수는 Rc instance임에도 불구하고 RefCell instance로 다룰 수 있어 편함
    soo.borrow_mut().next = Some(Rc::clone(&mok));

    let mut o_haeng = HashMap::new();
    o_haeng.insert(Name::Mok, mok);
    o_haeng.insert(Name::Hwa, hwa);
    o_haeng.insert(Name::To, to);
    o_haeng.insert(Name::Kum, kum);
    o_haeng.insert(Name::Soo, soo);

    o_haeng
}

pub fn get_saeng(o_haeng: &HashMap<Name, Rc<RefCell<Haeng>>>, name: Name) -> Name {
    match name {
        Name::Mok => Name::Hwa,
        Name::Hwa => Name::To,
        Name::To => Name::Kum,
        Name::Kum => Name::Soo,
        Name::Soo => Name::Mok
    }
}

pub fn get_kuk(o_haeng: &HashMap<Name, Rc<RefCell<Haeng>>>, name: Name) -> Name {
    match name {
        Name::Mok => Name::To,
        Name::Hwa => Name::Kum,
        Name::To => Name::Soo,
        Name::Kum => Name::Mok,
        Name::Soo => Name::Hwa
    }
}
