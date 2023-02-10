use std::rc::Rc;
use std::cell::RefCell;
use std::collections::HashMap;

#[derive(Debug, PartialEq, Eq, Hash, Copy, Clone)]
pub enum Name {
    Mok,
    Hwa,
    To,
    Gum,
    Soo
}

#[derive(Debug)]
pub struct Haeng {
    name: Name,
    pub character: String,
    next: Option<Rc<RefCell<Haeng>>>
}

pub fn create_ohaeng() -> HashMap<Name, Rc<RefCell<Haeng>>> {
    /*
    Haeng instance를 RefCell로 하나 생성하고, RefCell을 참조하는 Rc instance도 하나 생성한 후 불변 변수에 assign
    즉, Rc instance는 불변이고 따라서 Rc가 가리키는 대상인 RefCell도 불변
    Rc instance가 assign 된 불변 변수를 다룰 때에는 그냥 RefCell instance 처럼 다룰 수 있음(soo = *soo)
    Haeng의 next 값을 변경해야 하기 때문에 RefCell을 사용한 것이고, RefCell의 borrow_mut()을 사용해서 변경한다
    */
    let soo = Rc::new(RefCell::new(Haeng{name: Name::Soo, character: "水".to_string(), next: None}));
    let kum = Rc::new(RefCell::new(Haeng{name: Name::Gum, character: "金".to_string(), next: Some(Rc::clone(&soo))}));
    let to = Rc::new(RefCell::new(Haeng{name: Name::To, character: "土".to_string(), next: Some(Rc::clone(&kum))}));
    let hwa = Rc::new(RefCell::new(Haeng{name: Name::Hwa, character: "火".to_string(), next: Some(Rc::clone(&to))}));
    let mok = Rc::new(RefCell::new(Haeng{name: Name::Mok, character: "木".to_string(), next: Some(Rc::clone(&hwa))}));
    soo.borrow_mut().next = Some(Rc::clone(&mok));

    let mut ohaeng = HashMap::new();
    ohaeng.insert(Name::Mok, mok);
    ohaeng.insert(Name::Hwa, hwa);
    ohaeng.insert(Name::To, to);
    ohaeng.insert(Name::Gum, kum);
    ohaeng.insert(Name::Soo, soo);

    ohaeng
}

pub fn get_saeng(ohaeng: &HashMap<Name, Rc<RefCell<Haeng>>>, haeng_name: Name) -> Name {
    let haeng = &ohaeng.get(&haeng_name).unwrap().borrow();
    let saeng_haeng = haeng.next.as_ref().unwrap().borrow();
    saeng_haeng.name
}

pub fn get_kuk(ohaeng: &HashMap<Name, Rc<RefCell<Haeng>>>, haeng_name: Name) -> Name {
    let haeng = &ohaeng.get(&haeng_name).unwrap().borrow();
    let saeng_haeng = haeng.next.as_ref().unwrap().borrow();
    let kuk_haeng = saeng_haeng.next.as_ref().unwrap().borrow();
    kuk_haeng.name
}
