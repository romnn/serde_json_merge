use std::any::Any;
use std::rc::Rc;

pub trait JsonIndex: std::fmt::Display + std::fmt::Debug {
    // fn as_any(&self) -> Rc<dyn Any + '_>;
    fn as_any(&self) -> Rc<dyn Any>;
    fn eq(&self, other: &dyn JsonIndex) -> bool;
}

impl JsonIndex for str {
    // fn as_any(&self) -> &dyn Any {
    // fn as_any(&self) -> Rc<dyn Any + '_> {
    fn as_any(&self) -> Rc<dyn Any> {
        let s: Rc<str> = Rc::from(self);
        Rc::new(s)
    }

    fn eq(&self, other: &dyn JsonIndex) -> bool {
        // dbg!(other.as_any().downcast_ref::<Rc<str>>());
        if let Some(recovered) = other.as_any().downcast_ref::<Rc<str>>() {
            self == recovered.as_ref()
        } else {
            false
        }
    }
}

impl JsonIndex for String {
    // fn as_any(&self) -> &dyn Any {
    // fn as_any(&self) -> Rc<dyn Any + '_> {
    fn as_any(&self) -> Rc<dyn Any> {
        Rc::new(self.clone())
    }

    fn eq(&self, other: &dyn JsonIndex) -> bool {
        dbg!(other.as_any().downcast_ref::<Rc<String>>());
        dbg!(other.as_any().downcast_ref::<String>());
        if let Some(recovered) = other.as_any().downcast_ref::<String>() {
            // dbg!(self, recovered);
            // let test: String = self;
            // let test: String = recovered;
            self == recovered
        } else {
            false
        }
    }
}

impl JsonIndex for usize {
    // fn as_any(&self) -> &dyn Any {
    // fn as_any(&self) -> Rc<dyn Any + '_> {
    fn as_any(&self) -> Rc<dyn Any> {
        Rc::new(*self)
    }

    fn eq(&self, other: &dyn JsonIndex) -> bool {
        // dbg!(other.as_any().downcast_ref::<Rc<usize>>());
        // dbg!(other.as_any().downcast_ref::<usize>());
        if let Some(recovered) = other.as_any().downcast_ref::<usize>() {
            self == recovered
        } else {
            false
        }
    }
}

// &str, &String, &usize
impl<'a, I> JsonIndex for &'a I
where
    I: ?Sized + JsonIndex,
{
    // fn as_any(&self) -> &dyn Any {
    // fn as_any(&self) -> Rc<dyn Any + '_> {
    fn as_any(&self) -> Rc<dyn Any> {
        (*self).as_any()
        // self.as_any()
    }

    fn eq(&self, other: &dyn JsonIndex) -> bool {
        JsonIndex::eq(*self, other)
        // (*self).
    }
}

// trait A {
//     // An &Any can be cast to a reference to a concrete type.
//     fn as_any(&self) -> &dyn Any;

//     // Perform the test.
//     fn equals_a(&self, _: &dyn A) -> bool;
// }

// impl<S: 'static + PartialEq> A for S {
//     fn as_any(&self) -> &dyn Any {
//         self
//     }

//     fn equals_a(&self, other: &dyn A) -> bool {
//         // Do a type-safe casting. If the types are different,
//         // return false, otherwise test the values for equality.
//         other
//             .as_any()
//             .downcast_ref::<S>()
//             .map_or(false, |a| self == a)
//     }
// }

pub trait Test {
    fn eq(&self, other: &dyn Test) -> bool;
    fn as_any(&self) -> &dyn Any;
}

// impl Test for Rc<usize> {
//     fn as_any(&self) -> &dyn Any {
//         self
//     }
// }

// impl Test for Rc<str> {
//     fn as_any(&self) -> &dyn Any {
//         self
//     }
// }

// impl Test for Rc<String> {
//     fn as_any(&self) -> &dyn Any {
//         self
//     }
// }

// impl<'a, I> Test for &'a I
// where
//     I: ?Sized + Test + Any,
// {
//     fn as_any(&self) -> &dyn Any {
//         (*self).as_any()
//     }
// }

// implement all of those for Rc<usize>, Rc<str> etc. ourselves
impl<I> Test for Rc<I>
where
    I: ?Sized + JsonIndex + Any, // +  PartialEq,
{
    fn eq(&self, other: &dyn Test) -> bool {
        // fn eq(&self, other: &Rc<dyn Any>) -> bool {
        dbg!(self);
        dbg!(other.as_any().downcast_ref::<Rc<I>>());
        if let Some(other) = other.as_any().downcast_ref::<Rc<I>>() {
            // dbg!(other.downcast::<I>());
            // dbg!(other.downcast::<usize>());

            // now we have I ... which is useless?
            // return self == other;
        }
        false
    }

    fn as_any(&self) -> &dyn Any {
        self
    }
}

use std::borrow::Borrow;

// impl<'a, I> PartialEq<I> for &'a dyn JsonIndex
// where
//     I: Borrow<dyn JsonIndex>,
// {
//     fn eq(&self, other: &I) -> bool {
//         // JsonIndex::eq(*self, *other)
//         false
//     }
// }

// impl PartialEq<dyn JsonIndex> for dyn JsonIndex {
//     fn eq(&self, other: &dyn JsonIndex) -> bool {
//         // JsonIndex::eq(*self, *other)
//         false
//     }
// }

// impl<'a> PartialEq for &'a (dyn JsonIndex + '_) {
//     fn eq(&self, other: &Self) -> bool {
//         // self.id() == other.id()
//         false
//     }
// }

impl PartialEq for dyn JsonIndex + '_ {
    fn eq(&self, other: &Self) -> bool {
        // false
        // JsonIndex::eq(*self, *other)
        JsonIndex::eq(self, other)
    }
}

// impl<'a> PartialEq for &'a dyn JsonIndex {
//     fn eq(&self, other: &&dyn JsonIndex) -> bool {
//         // JsonIndex::eq(self, other)
//         // if let Some(other) = (*other).downcast_ref::<usize>() {
//         // if let Some(other) = (&*other as &dyn Any).downcast_ref::<usize>() {
//         //     return true;
//         // }
//         // self and other are &&dyn
//         // JsonIndex::eq(*self, *other)
//         false
//     }
// }

// impl PartialEq<Rc<dyn JsonIndex>> for dyn JsonIndex {
//     fn eq(&self, other: &Rc<dyn JsonIndex>) -> bool {
//         // JsonIndex::eq(*self, *other)
//         false
//     }
// }

// impl<'a> PartialEq<dyn JsonIndex> for &'a dyn JsonIndex {
//     fn eq(&self, other: &dyn JsonIndex) -> bool {
//         // JsonIndex::eq(*self, *other)
//         false
//     }
// }

// impl<'a> PartialEq<&'a dyn JsonIndex> for &'a dyn JsonIndex {
//     fn eq(&self, other: &&dyn JsonIndex) -> bool {
//         // JsonIndex::eq(*self, *other)
//         false
//     }
// }

// #[derive(Debug)]
// struct Wrapper(Rc<dyn JsonIndex>);

// impl PartialEq<Wrapper> for Wrapper {
//     fn eq(&self, other: &Wrapper) -> bool {
//         dbg!(self, other);
//         // dbg!(self.0.as_any());
//         // dbg!(other.0.as_any());
//         // JsonIndex::eq(&self.0, &other.0)
//         false
//     }
// }

fn main() {
    let src: &str = "test";
    // let box: Box<str> = Box::from(s);
    let raw_str: Rc<str> = Rc::from(src);
    let any: Rc<dyn Any> = Rc::new(raw_str);
    // dbg!(any.downcast_ref::<Rc<str>>());
    if let Some(recovered) = any.downcast_ref::<Rc<str>>() {
        assert_eq!(&**recovered, src);
    }
    // let index: &Rc<dyn JsonIndex> = &raw_str;
    // let index = &raw_str as &Rc<dyn JsonIndex>;
    // let a: Rc<str> = Rc::new("test");
    // let b: &Rc<dyn Any> = &a;

    let s1: String = "test".into();
    let s2: String = "test".into();
    let test = s1 == s2;
    assert_eq!(s1, s2);

    let s1: Rc<String> = Rc::new(String::from("test"));
    let s2: Rc<String> = Rc::new(String::from("test"));
    let test = s1 == s2;
    assert_eq!(s1, s2);

    let a: &dyn JsonIndex = &12usize;
    let b: &dyn JsonIndex = &24usize;
    assert_ne!(a, b);
    assert_ne!(&12usize as &dyn JsonIndex, &24usize as &dyn JsonIndex);
    assert_eq!(&12usize as &dyn JsonIndex, &12usize as &dyn JsonIndex);
    assert_eq!(&"test" as &dyn JsonIndex, &"test" as &dyn JsonIndex);
    assert_ne!(&"test hallo" as &dyn JsonIndex, &"test" as &dyn JsonIndex);
    assert_ne!(
        &String::from("test") as &dyn JsonIndex,
        &"test" as &dyn JsonIndex
    );
    assert_eq!(
        &String::from("test") as &dyn JsonIndex,
        &String::from("test") as &dyn JsonIndex
    );
    let s1: String = "test".into();
    let s2: String = "test".into();
    assert_eq!(&s1 as &dyn JsonIndex, &s2 as &dyn JsonIndex);
    assert_eq!(&&s1 as &dyn JsonIndex, &&s2 as &dyn JsonIndex);

    let i1: usize = 100;
    let i2: usize = 100;
    assert_eq!(&i1 as &dyn JsonIndex, &i2 as &dyn JsonIndex);
    assert_eq!(&&i1 as &dyn JsonIndex, &&i2 as &dyn JsonIndex);

    let s1: &str = "test";
    let s2: &str = "test";
    assert_eq!(&s1 as &dyn JsonIndex, &s2 as &dyn JsonIndex);
    assert_eq!(&&s1 as &dyn JsonIndex, &&s2 as &dyn JsonIndex);

    let s1: &str = "test";
    let s2: &str = "different";
    assert_ne!(&s1 as &dyn JsonIndex, &s2 as &dyn JsonIndex);
    assert_ne!(&&s1 as &dyn JsonIndex, &&s2 as &dyn JsonIndex);

    assert_ne!(&12usize as &dyn JsonIndex, &"test" as &dyn JsonIndex);
    assert_ne!(&&12usize as &dyn JsonIndex, &&"test" as &dyn JsonIndex);

    // assert_eq!(a, b);

    // let a: Rc<dyn JsonIndex> = Rc::new(12usize);
    // let b: Rc<dyn JsonIndex> = Rc::new(24usize);
    // assert_ne!(a, b);
    // assert_ne!(Wrapper(a), Wrapper(b));

    let a: Rc<dyn JsonIndex> = Rc::new(24usize);
    let b: Rc<dyn JsonIndex> = Rc::new(24usize);
    assert_eq!(&a, &b);

    let a: Rc<dyn JsonIndex> = Rc::new(12usize);
    let b: Rc<dyn JsonIndex> = Rc::new(24usize);
    assert_ne!(&a, &b);

    // assert_eq!(a.clone(), b.clone());
    // assert_eq!(Wrapper(a), Wrapper(b));
    // assert!(a.equals_a(b));
    // assert_eq!(index!(12), index!(24));
}
