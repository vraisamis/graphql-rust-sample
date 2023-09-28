use std::collections::HashMap;

use crate::sample::{self, Column, Data};
use anyhow::{anyhow, Result};
use async_trait::async_trait;
use domain_kanban::column::ColumnId;
use query_resolver::{CardView, CardsQuery};
use shaku::Provider;

#[derive(Debug, Clone, Provider)]
#[shaku(interface = CardsQuery)]
pub struct CardsQueryImpl;

impl CardsQueryImpl {}

fn to_view(u: sample::Card) -> CardView {
    CardView {
        id: u.id.to_string(),
        title: u.title,
        description: u.description,
    }
}

#[async_trait]
impl CardsQuery for CardsQueryImpl {
    async fn find_by_order(&self, column_id: &ColumnId, order: &usize) -> Result<CardView> {
        let data = sample::data();
        let column = get_column(&data, column_id).ok_or_else(|| anyhow!("Not Found!"))?;
        let card = column
            .cards
            .get(*order)
            .ok_or_else(|| anyhow!("Not Found!"))?;
        let result = to_view(card.clone());

        Ok(result)
    }

    async fn list_by_orders(
        &self,
        column_id: &ColumnId,
        orders: &[usize],
    ) -> Result<HashMap<usize, CardView>> {
        let data = sample::data();
        let column = get_column(&data, column_id).ok_or_else(|| anyhow!("Not Found!"))?;
        let result = orders
            .to_vec()
            .iter()
            .filter_map(|i| column.cards.get(*i).map(|v| (*i, to_view(v.clone()))))
            .collect::<HashMap<_, _>>();

        Ok(result)
    }
}

fn get_column<'a>(data: &'a Data, id: &'a ColumnId) -> Option<&'a Column> {
    data.columns.iter().filter(|c| &c.id == id).next()
}
