use domain_util::{Entity, Identifier};

#[allow(unused)]
pub struct Column {
    id: ColumnId,
    title: ColumnTitle,
    cards: Vec<Card>,
}

impl Column {
    pub fn new(title: ColumnTitle) -> Self {
        Self {
            id: ColumnId::gen(),
            title,
            cards: vec![],
        }
    }

    pub fn add_card(mut self, title: CardTitle) -> Self {
        self.cards.push(Card::new(title));
        self
    }

    pub fn remove_card(mut self, index: usize) -> Self {
        assert!(index < self.cards.len());

        self.cards.remove(index);
        self
    }

    pub fn rerank_card(mut self, src_index: usize, dst_index: usize) -> Self {
        assert!(src_index < self.cards.len());
        assert!(dst_index < self.cards.len());

        let card = self.cards.remove(src_index);
        self.cards.insert(dst_index, card);
        self
    }

    pub fn get_card_mut(&mut self, index: usize) -> Option<&mut Card> {
        assert!(index < self.cards.len());

        self.cards.get_mut(index)
    }
}

impl Entity for Column {
    fn entity_type() -> &'static str {
        "column"
    }
}

pub type ColumnId = Identifier<Column>;

#[allow(unused)]
pub struct ColumnTitle(String);

impl ColumnTitle {
    pub fn new(s: String) -> Self {
        Self(s)
    }
}

#[allow(unused)]
pub struct Card {
    // idは参考用
    id: CardId,
    title: CardTitle,
    description: CardDescription,
    // status: String,
}

impl Card {
    pub fn new(title: CardTitle) -> Self {
        Self {
            id: CardId::gen(),
            title,
            description: CardDescription::new("".to_owned()),
        }
    }
    pub fn with_description(title: CardTitle, description: CardDescription) -> Self {
        Self {
            id: CardId::gen(),
            title,
            description,
        }
    }

    pub fn edit_title(mut self, new_title: CardTitle) -> Self {
        self.title = new_title;
        self
    }

    pub fn edit_description(mut self, new_description: CardDescription) -> Self {
        self.description = new_description;
        self
    }
}

impl Entity for Card {
    fn entity_type() -> &'static str {
        "card"
    }
}

pub type CardId = Identifier<Card>;

#[allow(unused)]
pub struct CardTitle(String);

impl CardTitle {
    pub fn new(s: String) -> Self {
        Self(s)
    }
}

#[allow(unused)]
pub struct CardDescription(String);

impl CardDescription {
    pub fn new(s: String) -> Self {
        Self(s)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    struct TestValues {
        title1: &'static str,
        card_title1: CardTitle,
        title2: &'static str,
        card_title2: CardTitle,
        description: &'static str,
        card_description: CardDescription,
    }
    fn init() -> TestValues {
        let title1 = "foo";
        let title2 = "bar";
        let description = "hoge";
        TestValues {
            title1,
            card_title1: CardTitle::new(title1.to_owned()),
            title2,
            card_title2: CardTitle::new(title2.to_owned()),
            description,
            card_description: CardDescription::new(description.to_owned()),
        }
    }

    #[test]
    fn card_title_new_test() {
        let TestValues { title1: s, .. } = init();
        let title = CardTitle::new(s.to_owned());
        assert!(title.0 == s);
    }

    #[test]
    fn card_description_new_test() {
        let TestValues { description: s, .. } = init();
        let description = CardDescription::new(s.to_owned());
        assert!(description.0 == s);
    }

    #[test]
    fn card_edit_test() {
        let TestValues {
            title1: old_title,
            card_title1: old_card_title,
            title2: new_title,
            card_title2: new_card_title,
            description: new_description,
            card_description: new_card_description,
        } = init();
        let card = Card::new(old_card_title);

        assert!(card.title.0 == old_title);
        assert!(card.description.0 == "");

        let card = card.edit_title(new_card_title);

        assert!(card.title.0 == new_title);
        assert!(card.description.0 == "");

        let card = card.edit_description(new_card_description);

        assert!(card.title.0 == new_title);
        assert!(card.description.0 == new_description);
    }
}
