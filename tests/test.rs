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

#[derive(Ablate, Debug, PartialEq)]
struct Hi {
    a: bool,
    b: bool,
}

#[test]
fn play() {
    #[rustfmt::skip]
    let gold = vec![
	Thing { field_a: T::A, field_b: true, field_c: Hi { a: true, b: true } },
	Thing { field_a: T::A, field_b: true, field_c: Hi { a: true, b: false } },
	Thing { field_a: T::A, field_b: true, field_c: Hi { a: false, b: true } },
	Thing { field_a: T::A, field_b: true, field_c: Hi { a: false, b: false } },
	Thing { field_a: T::A, field_b: false, field_c: Hi { a: true, b: true } },
	Thing { field_a: T::A, field_b: false, field_c: Hi { a: true, b: false } },
	Thing { field_a: T::A, field_b: false, field_c: Hi { a: false, b: true } },
	Thing { field_a: T::A, field_b: false, field_c: Hi { a: false, b: false } },
	Thing { field_a: T::B, field_b: true, field_c: Hi { a: true, b: true } },
	Thing { field_a: T::B, field_b: true, field_c: Hi { a: true, b: false } },
	Thing { field_a: T::B, field_b: true, field_c: Hi { a: false, b: true } },
	Thing { field_a: T::B, field_b: true, field_c: Hi { a: false, b: false } },
	Thing { field_a: T::B, field_b: false, field_c: Hi { a: true, b: true } },
	Thing { field_a: T::B, field_b: false, field_c: Hi { a: true, b: false } },
	Thing { field_a: T::B, field_b: false, field_c: Hi { a: false, b: true } },
	Thing { field_a: T::B, field_b: false, field_c: Hi { a: false, b: false } },
	Thing { field_a: T::C, field_b: true, field_c: Hi { a: true, b: true } },
	Thing { field_a: T::C, field_b: true, field_c: Hi { a: true, b: false } },
	Thing { field_a: T::C, field_b: true, field_c: Hi { a: false, b: true } },
	Thing { field_a: T::C, field_b: true, field_c: Hi { a: false, b: false } },
	Thing { field_a: T::C, field_b: false, field_c: Hi { a: true, b: true } },
	Thing { field_a: T::C, field_b: false, field_c: Hi { a: true, b: false } },
	Thing { field_a: T::C, field_b: false, field_c: Hi { a: false, b: true } },
	Thing { field_a: T::C, field_b: false, field_c: Hi { a: false, b: false } }
    ];
    for (i, config) in Thing::ablate().enumerate() {
        assert_eq!(gold[i], config);
    }
}
