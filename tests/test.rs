use ablate::{Ablate, AblateIter};

// #[derive(Ablate)]
#[derive(Debug, PartialEq)]
struct Thing {
    field_a: A,
    field_b: Bool,
}

#[derive(Debug, PartialEq)]
enum A {
    Aa,
    Bb,
    Cc,
}

#[derive(Debug, PartialEq)]
enum Bool {
    True,
    False,
}

struct AIter(usize);

impl Default for AIter {
    fn default() -> Self {
        AIter(0)
    }
}

impl Iterator for AIter {
    type Item = A;

    fn next(&mut self) -> Option<Self::Item> {
        let ret = match self.0 {
            0 => Some(A::Aa),
            1 => Some(A::Bb),
            2 => Some(A::Cc),
            _ => None,
        };
        self.0 += 1;
        ret
    }
}

struct BoolIter(usize);

impl Default for BoolIter {
    fn default() -> Self {
        BoolIter(0)
    }
}

impl Iterator for BoolIter {
    type Item = Bool;

    fn next(&mut self) -> Option<Self::Item> {
        let ret = match self.0 {
            0 => Some(Bool::True),
            1 => Some(Bool::False),
            _ => None,
        };
        self.0 += 1;
        ret
    }
}

impl ablate::AblateIter for A {
    type Iterator = AIter;

    fn iter() -> Self::Iterator {
        AIter(0)
    }
}

#[derive(Default)]
struct ThingIter<AI: Iterator<Item = A> + Default, BI: Iterator<Item = Bool> + Default> {
    field_a_iter: AI,
    cache_a: Option<A>,
    field_b_iter: BI,
    cache_b: Option<Bool>,
}

impl<AI, BI> Iterator for ThingIter<AI, BI>
where
    AI: Iterator<Item = A> + Default,
    BI: Iterator<Item = Bool> + Default,
{
    type Item = Thing;

    fn next(&mut self) -> Option<Self::Item> {
        todo!()
    }
}

impl ablate::AblateIter for Thing {
    type Iterator = ThingIter<AIter, BoolIter>;

    fn iter() -> Self::Iterator {
        ThingIter {
            field_a_iter: AIter::default(),
            field_b_iter: BoolIter::default(),
        }
    }
}

#[test]
fn it_does_thing() {
    let gold = [
        Thing {
            field_a: A::Aa,
            field_b: Bool::True,
        },
        Thing {
            field_a: A::Bb,
            field_b: Bool::True,
        },
        Thing {
            field_a: A::Cc,
            field_b: Bool::True,
        },
        Thing {
            field_a: A::Aa,
            field_b: Bool::False,
        },
        Thing {
            field_a: A::Bb,
            field_b: Bool::False,
        },
        Thing {
            field_a: A::Cc,
            field_b: Bool::False,
        },
    ];
    for (i, v) in Thing::iter().enumerate() {
        assert_eq!(v, gold[i]);
    }
}
