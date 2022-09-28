pub use ablate_derive::Ablate;

pub trait AblateIter: Sized {
    type Iterator: Iterator<Item = Self>;

    fn iter() -> Self::Iterator;
}

// pub trait Ablate: Iterator<Item = Self> + Sized {
//     fn ablate(self) -> AblateIter;
// }
