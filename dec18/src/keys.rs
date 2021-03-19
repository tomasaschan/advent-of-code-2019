use std::{
    cmp::Ordering,
    fmt::{Debug, Error, Formatter, Write},
    iter::FromIterator,
};

#[derive(Clone, Copy, Eq, PartialEq)]
pub struct KeyId {
    value: u32,
}

impl KeyId {
    pub fn to_key_char(&self) -> char {
        ((f64::from(self.value).log2().round() as u8) + b'a') as char
    }

    pub fn to_door_char(&self) -> char {
        ((f64::from(self.value).log2().round() as u8) + b'A') as char
    }
}

impl From<char> for KeyId {
    fn from(c: char) -> KeyId {
        let basis = if c.is_ascii_uppercase() { b'A' } else { b'a' };
        KeyId {
            value: 1_u32 << (c as u32 - basis as u32),
        }
    }
}

impl Debug for KeyId {
    fn fmt(&self, f: &mut Formatter) -> Result<(), Error> {
        f.write_char(self.to_key_char())
    }
}

#[derive(Eq, PartialEq, Hash, Clone)]
pub struct KeyRing {
    pub keys: u32,
}

impl KeyRing {
    pub fn new() -> Self {
        KeyRing { keys: 0 }
    }

    pub fn with(&self, key: &KeyId) -> Self {
        let mut ks = KeyRing::new();
        ks.keys = self.keys;
        ks.keys |= key.value;
        ks
    }
    pub fn union_with(&self, keys: &KeyRing) -> Self {
        let mut ks = KeyRing::new();
        ks.keys = self.keys;
        ks.keys |= keys.keys;
        ks
    }

    pub fn insert(&mut self, key: &KeyId) {
        self.keys |= key.value;
    }

    pub fn insert_all(&mut self, keys: &KeyRing) {
        self.keys |= keys.keys
    }

    pub fn contains(&self, key: &KeyId) -> bool {
        self.keys & key.value == key.value
    }

    pub fn contains_all(&self, keys: &KeyRing) -> bool {
        self.keys & keys.keys == keys.keys
    }

    pub fn len(&self) -> u32 {
        self.keys.count_ones()
    }
}

impl PartialOrd for KeyRing {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        self.len().partial_cmp(&other.len())
    }
}
impl Ord for KeyRing {
    fn cmp(&self, other: &Self) -> Ordering {
        self.len().cmp(&other.len())
    }
}

impl<I, K> From<I> for KeyRing
where
    I: IntoIterator<Item = K>,
    K: Into<KeyId>,
{
    fn from(it: I) -> Self {
        it.into_iter().collect()
    }
}

impl<K> FromIterator<K> for KeyRing
where
    K: Into<KeyId>,
{
    fn from_iter<I>(it: I) -> Self
    where
        I: IntoIterator<Item = K>,
    {
        KeyRing {
            keys: it.into_iter().fold(0, |keys, key| keys | key.into().value),
        }
    }
}

impl Debug for KeyRing {
    fn fmt(&self, f: &mut Formatter) -> Result<(), Error> {
        write!(f, "[")?;

        for i in 0..=('z' as u8 - 'a' as u8) {
            let c = 1 << i;
            write!(
                f,
                "{}",
                if self.keys & c == c {
                    (b'a' + i) as char
                } else {
                    ' '
                }
            )?;
        }

        write!(f, "]")
    }
}
