use crate::{PointerBuf, Token};
use alloc::{
    boxed::Box,
    string::{String, ToString},
    vec::Vec,
};
use quickcheck::Arbitrary;

impl Arbitrary for Token {
    fn arbitrary(g: &mut quickcheck::Gen) -> Self {
        Self::new(String::arbitrary(g))
    }

    fn shrink(&self) -> Box<dyn Iterator<Item = Self>> {
        Box::new(ToString::to_string(self).shrink().map(Self::new))
    }
}

impl Arbitrary for PointerBuf {
    fn arbitrary(g: &mut quickcheck::Gen) -> Self {
        let size = usize::arbitrary(g) % g.size();
        Self::from_tokens((0..size).map(|_| Token::arbitrary(g)).collect::<Vec<_>>())
    }

    fn shrink(&self) -> Box<dyn Iterator<Item = Self>> {
        let tokens: Vec<_> = self.tokens().collect();
        Box::new((0..self.count()).map(move |i| {
            let subset: Vec<_> = tokens
                .iter()
                .enumerate()
                .filter_map(|(j, t)| (i != j).then_some(t.clone()))
                .collect();
            Self::from_tokens(subset)
        }))
    }
}
