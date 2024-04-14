pub trait InvariantSheild
where
    Self: 'static + Sized,
{
    type Error: std::error::Error;

    fn sheilds() -> Vec<&'static dyn Fn(&Self) -> Result<(), <Self as InvariantSheild>::Error>>;
    fn satisfy_sheilds_ref(&self) -> Result<&Self, <Self as InvariantSheild>::Error> {
        let init = Ok(());
        Self::sheilds()
            .iter()
            .fold(init, |result, func| result.and_then(|_| func(self)))
            .map(|_| self)
    }
    fn satisfy_sheilds_mut(&mut self) -> Result<&mut Self, <Self as InvariantSheild>::Error> {
        let init = Ok(());
        Self::sheilds()
            .iter()
            .fold(init, |result, func| result.and_then(|_| func(&self)))
            .map(|_| self)
    }
    fn satisfy_sheilds(self) -> Result<Self, <Self as InvariantSheild>::Error> {
        let init = Ok(());
        Self::sheilds()
            .iter()
            .fold(init, |result, func| result.and_then(|_| func(&self)))
            .map(|_| self)
    }
}
