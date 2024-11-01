use std::collections::BTreeMap;

use lex::token::teer;

use super::{gc::Heap, objects::ObjectInner};

#[test]
fn test_gc_base() {
    let mut hp = Heap::new();

    let a = hp.alloc(ObjectInner::Int(1));
    let b = hp.alloc(ObjectInner::Float(-114.514));
    let c = hp.alloc(ObjectInner::String("Hello, SL!".to_string()));

    // , ("world".to_string(), b)

    let d = hp.alloc(ObjectInner::Model {
        model_name: "Mdl".into(),
        fields: vec![("hello".to_string(), a)].into_iter().collect(),
    });

    hp.gc(vec![b, d]);

    dbg!(&hp.objs);
    let mut iter = hp.objs.into_iter();
    assert!(iter.next().unwrap().is_some());
    assert!(iter.next().unwrap().is_some());
    assert!(iter.next().unwrap().is_none()); // collected
    assert!(iter.next().unwrap().is_some());
    assert!(iter.next().is_none()); // iterator ends
}

#[test]
fn test_gc_cyc() {
    let mut hp = Heap::new();

    let a = hp.alloc(ObjectInner::Int(1));
    let b = hp.alloc(ObjectInner::Float(-114.514));
    let c = hp.alloc(ObjectInner::String("Hello, SL!".to_string()));
    let z = hp.alloc(ObjectInner::Teer(teer::excel));

    // , ("world".to_string(), b)

    let d = hp.alloc(ObjectInner::Model {
        model_name: "Mdl".into(),
        fields: vec![("hello".to_string(), a), ("world".to_string(), b)]
            .into_iter()
            .collect(),
    });
    let e = hp.alloc(ObjectInner::Nil);

    let f = hp.alloc(ObjectInner::Model {
        model_name: "Mdl".into(),
        fields: vec![("world".to_string(), e), ("ffff".to_string(), d)]
            .into_iter()
            .collect(),
    });

    d.refs(&mut hp, vec!["world".to_string()], f).unwrap();

    hp.gc(vec![c, d]);
    dbg!(&hp.objs);
    let mut iter = hp.objs.iter();
    assert!(iter.next().unwrap().is_some());
    assert!(iter.next().unwrap().is_none());
    assert!(iter.next().unwrap().is_some());
    assert!(iter.next().unwrap().is_none());
    assert!(iter.next().unwrap().is_some());
    assert!(iter.next().unwrap().is_some());
    assert!(iter.next().unwrap().is_some());

    hp.gc(vec![c]);
    dbg!(&hp.objs);
    let mut iter = hp.objs.iter();
    assert!(iter.next().unwrap().is_none());
    assert!(iter.next().unwrap().is_none());
    assert!(iter.next().unwrap().is_some());
    assert!(iter.next().unwrap().is_none());
    assert!(iter.next().unwrap().is_none());
    assert!(iter.next().unwrap().is_none());
    assert!(iter.next().unwrap().is_none());
}
