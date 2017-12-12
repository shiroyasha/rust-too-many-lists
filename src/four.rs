
#[derive(Debug)]
struct Collection<'a> {
    elem: Option<Box<Elem<'a>>>,
}

#[derive(Debug)]
struct Elem<'a> {
    value: &'a str,
}

fn insert<'a>(c : &mut Collection<'a>, e : Box<Elem<'a>>) {
    c.elem = Some(e);
}


fn a<'a>() -> Collection<'a> {
    let mut c = Collection { elem : None };
    let e = Box::new(Elem { value : "aaa" });

    insert(&mut c, e);

    c
}

#[test]
fn a_test() {
    let c = a();

    assert_eq!(c.elem.unwrap().value, "aaa");
}
