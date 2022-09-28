use ablate::{Ablate, IntoAblate};

#[derive(Ablate, Debug, PartialEq)]
enum T {
    A,
    B,
    C,
}

#[derive(Ablate, Debug, PartialEq)]
struct Thing {
    field_a: T,
    field_b: bool,
    field_c: Hi,
}

#[derive(Debug, PartialEq)]
struct Hi {
    a: bool,
    b: bool,
}

impl Ablate for Hi {
    fn nth(n: usize) -> Self {
        let [ia, ib] = ablate::digits([bool::size(), bool::size()], n);
        let a = bool::nth(ia);
        let b = bool::nth(ib);
        Hi { a, b }
    }

    fn size() -> usize {
        bool::size() * bool::size()
    }
}

// impl Ablate for Thing {
//     fn nth(n: usize) -> Self {
//         let [ia, ib, ic] = ablate::digits([T::size(), bool::size(), Hi::size()], n);
//         let a = T::nth(ia);
//         let b = bool::nth(ib);
//         let c = Hi::nth(ic);
//         Thing {
//             field_a: a,
//             field_b: b,
//             field_c: c,
//         }
//     }

//     fn size() -> usize {
//         T::size() * bool::size() * Hi::size()
//     }
// }

#[test]
fn play() {
    for i in Thing::ablate() {
        eprintln!("{i:?}");
    }
    assert!(false)
}
