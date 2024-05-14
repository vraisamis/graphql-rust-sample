use domain_util::{InvariantError, InvariantResult};
use invariant_sheild::{invariant_sheild, InvariantSheild};
use serde::Deserialize;
use serde::Serialize;

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct UserName(String);

#[invariant_sheild(InvariantError)]
impl UserName {
    pub fn new(value: String) -> InvariantResult<Self> {
        Self::new_unchecked(value).satisfy_sheilds()
    }

    pub(crate) fn new_unchecked(value: String) -> Self {
        Self(value)
    }

    #[sheild]
    pub(crate) fn user_name_length_less_than_21(&self) -> InvariantResult<()> {
        if self.0.len() < 21 {
            Ok(())
        } else {
            Err(InvariantError::ViolationError(
                "名前が長すぎます。20文字以内にしてください".to_owned(),
            ))
        }
    }
}

impl TryFrom<String> for UserName {
    type Error = InvariantError;

    fn try_from(value: String) -> Result<Self, Self::Error> {
        Self::new(value)
    }
}

impl ToString for UserName {
    fn to_string(&self) -> String {
        self.0.clone()
    }
}

#[cfg(feature = "dummy")]
mod dummy {
    use super::*;
    use fake::{faker::name::ja_jp::Name, Dummy, Fake, Faker};

    impl Dummy<Faker> for UserName {
        fn dummy_with_rng<R: rand::Rng + ?Sized>(_config: &Faker, rng: &mut R) -> Self {
            let f = Name();
            (0..1000)
                .filter_map(|_| {
                    let s: String = f.fake_with_rng(rng);
                    UserName::new(s).ok()
                })
                .next()
                .unwrap()
        }
    }

    #[cfg(test)]
    #[test]
    fn usage() {
        use fake::vec as fake_vec;

        let names: Vec<UserName> = fake_vec![UserName; 3..5];
        for (i, name) in names.into_iter().enumerate() {
            println!("{}: {}", i, name.to_string());
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn user_name_less_than_21_is_ok() -> InvariantResult<()> {
        let name = UserName::new("12345678901234567890".to_owned())?;
        assert_eq!(name.satisfy_sheilds_ref(), Ok(&name));
        Ok(())
    }
}
