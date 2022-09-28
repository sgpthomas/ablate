use std::marker::PhantomData;

pub use ablate_derive::Ablate;

pub trait IntoAblate: Sized {
    type Iterator: Iterator<Item = Self>;

    fn ablate() -> Self::Iterator;
}

impl<T: Ablate> IntoAblate for T {
    type Iterator = AblateIter<T>;

    fn ablate() -> Self::Iterator {
        AblateIter {
            index: 0,
            phantom: PhantomData,
        }
    }
}

pub trait Ablate: Sized {
    fn nth(n: usize) -> Self;
    fn size() -> usize;
}

pub struct AblateIter<T> {
    index: usize,
    phantom: PhantomData<T>,
}

impl<T: Ablate> Iterator for AblateIter<T> {
    type Item = T;

    fn next(&mut self) -> Option<Self::Item> {
        if self.index < Self::Item::size() {
            let item = Self::Item::nth(self.index);
            self.index += 1;
            Some(item)
        } else {
            None
        }
    }
}

pub fn digits<const N: usize>(dim_sizes: [usize; N], raw: usize) -> [usize; N] {
    let mut output = [0; N];
    for i in 0..dim_sizes.len() {
        let rest: usize = dim_sizes[i + 1..].iter().product();
        output[i] = (raw / rest) % dim_sizes[i];
    }
    output
}

impl Ablate for bool {
    fn nth(n: usize) -> Self {
        match n {
            0 => true,
            1 => false,
            _ => unreachable!(),
        }
    }

    fn size() -> usize {
        2
    }
}

impl<T: Ablate> Ablate for Option<T> {
    fn nth(n: usize) -> Self {
        match n {
            0 => None,
            x => Some(T::nth(x - 1)),
        }
    }

    fn size() -> usize {
        T::size() + 1
    }
}
