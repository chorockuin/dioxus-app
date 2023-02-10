use std::rc::Rc;
use std::cell::RefCell;
use std::collections::HashMap;

#[derive(Debug, PartialEq, Eq, Hash, Copy, Clone)]
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
    /*
    Haeng instance를 RefCell로 하나 생성하고, RefCell을 참조하는 Rc instance도 하나 생성한 후 불변 변수에 assign
    즉, Rc instance는 불변이고 따라서 Rc가 가리키는 대상인 RefCell도 불변
    Rc instance가 assign 된 불변 변수를 다룰 때에는 그냥 RefCell instance 처럼 다룰 수 있음(soo = *soo)
    Haeng의 next 값을 변경해야 하기 때문에 RefCell을 사용한 것이고, RefCell의 borrow_mut()을 사용해서 변경한다
    */
    let soo = Rc::new(RefCell::new(Haeng{name: Name::Soo, next: None}));
    let kum = Rc::new(RefCell::new(Haeng{name: Name::Kum, next: Some(Rc::clone(&soo))}));
    let to = Rc::new(RefCell::new(Haeng{name: Name::To, next: Some(Rc::clone(&kum))}));
    let hwa = Rc::new(RefCell::new(Haeng{name: Name::Hwa, next: Some(Rc::clone(&to))}));
    let mok = Rc::new(RefCell::new(Haeng{name: Name::Mok, next: Some(Rc::clone(&hwa))}));
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
    let haeng = &o_haeng.get(&name).unwrap().borrow();
    let saeng_haeng = haeng.next.as_ref().unwrap().borrow();
    saeng_haeng.name
}

pub fn get_kuk(o_haeng: &HashMap<Name, Rc<RefCell<Haeng>>>, name: Name) -> Name {
    let haeng = &o_haeng.get(&name).unwrap().borrow();
    let saeng_haeng = haeng.next.as_ref().unwrap().borrow();
    let kuk_haeng = saeng_haeng.next.as_ref().unwrap().borrow();
    kuk_haeng.name
}
