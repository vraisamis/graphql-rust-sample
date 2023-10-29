use std::{collections::HashMap, hash::Hash, sync::Arc};

use async_graphql::{
    extensions::{Extension, ExtensionContext, ExtensionFactory, NextParseQuery},
    parser::types::{
        DocumentOperations, ExecutableDocument, Field, OperationDefinition, OperationType,
        Selection, SelectionSet,
    },
    Positioned, ServerError, ServerResult, Variables,
};
use async_trait::async_trait;

pub struct RestrictQueryAliases {
    limit: usize,
    limit_per_level: usize,
}

impl RestrictQueryAliases {
    pub fn new(limit: usize, limit_per_level: usize) -> Self {
        // FIXME: 参考用に5にしている
        Self {
            limit,
            limit_per_level,
        }
    }
}

impl Default for RestrictQueryAliases {
    fn default() -> Self {
        Self::new(10, 3)
    }
}

impl ExtensionFactory for RestrictQueryAliases {
    fn create(&self) -> Arc<dyn Extension> {
        Arc::new(RestrictQueryAliasesImpl::new(
            self.limit,
            self.limit_per_level,
        ))
    }
}

pub struct RestrictQueryAliasesImpl {
    limit: usize,
    limit_per_level: usize,
}
impl RestrictQueryAliasesImpl {
    pub fn new(limit: usize, limit_per_level: usize) -> Self {
        Self {
            limit,
            limit_per_level,
        }
    }
}

#[async_trait]
impl Extension for RestrictQueryAliasesImpl {
    async fn parse_query(
        &self,
        ctx: &ExtensionContext<'_>,
        query: &str,
        variables: &Variables,
        next: NextParseQuery<'_>,
    ) -> ServerResult<ExecutableDocument> {
        let result = next.run(ctx, query, variables).await?;

        // after parsed
        let query_selection_set = find_query(&result);

        let aliases = alias_count_recursive(&query_selection_set);
        if aliases > self.limit {
            Err(ServerError::new(
                format!("エイリアスは全部で{}個より多くできません", self.limit),
                None,
            ))?;
        }

        let aliases_per_level = alias_count_by_nested_level_recursive(&query_selection_set, 1);
        let max_aliases_per_level = max_value_or(aliases_per_level, 0);
        if max_aliases_per_level > self.limit_per_level {
            Err(ServerError::new(
                format!(
                    "エイリアスは各階層で{}個より多くできません",
                    self.limit_per_level
                ),
                None,
            ))?;
        }

        Ok(result)
    }
}

fn find_query(doc: &ExecutableDocument) -> Vec<&Positioned<SelectionSet>> {
    match &doc.operations {
        DocumentOperations::Single(op) => selection_set_of(&op.node).into_iter().collect(),
        DocumentOperations::Multiple(map) => map
            .values()
            .filter_map(|op| selection_set_of(&op.node))
            .collect(),
    }
}

fn selection_set_of(op: &OperationDefinition) -> Option<&Positioned<SelectionSet>> {
    (op.ty == OperationType::Query).then(|| &op.selection_set)
}

fn alias_count_recursive(v: &Vec<&Positioned<SelectionSet>>) -> usize {
    v.into_iter().fold(0, |acc, p| {
        acc + fields_iter(p).filter(|f| is_alias(&f.node)).count()
            + alias_count_recursive(&child_selection_set(p))
    })
}

fn alias_count_by_nested_level_recursive(
    v: &Vec<&Positioned<SelectionSet>>,
    level: usize,
) -> HashMap<usize, usize> {
    v.into_iter().fold(HashMap::new(), |mut h, p| {
        let current = fields_iter(p).filter(|f| is_alias(&f.node)).count();
        if current > 0 {
            h.entry(level)
                .and_modify(|e| *e += current)
                .or_insert(current);
        }

        let child = alias_count_by_nested_level_recursive(&child_selection_set(p), level + 1);
        merge_by(h, child, |l, r| l + r)
    })
}

fn child_selection_set(selection_set: &Positioned<SelectionSet>) -> Vec<&Positioned<SelectionSet>> {
    // TODO: Field以外はaliasない？
    selections_iter(selection_set)
        .filter_map(|s| match &s.node {
            Selection::Field(f) => Some(&f.node.selection_set),
            Selection::FragmentSpread(_) => None,
            Selection::InlineFragment(f) => Some(&f.node.selection_set),
        })
        .collect()
}

fn selections_iter(
    selection_set: &Positioned<SelectionSet>,
) -> impl Iterator<Item = &Positioned<Selection>> {
    selection_set.node.items.iter()
}

fn fields_iter(
    selection_set: &Positioned<SelectionSet>,
) -> impl Iterator<Item = &Positioned<Field>> {
    selections_iter(selection_set).filter_map(|s| match &s.node {
        Selection::Field(f) => Some(f),
        _ => None,
    })
}

fn is_alias(field: &Field) -> bool {
    field.alias.is_some()
}

fn merge_by<K, V, F>(dst: HashMap<K, V>, src: HashMap<K, V>, f: F) -> HashMap<K, V>
where
    K: Hash + PartialEq + Eq,
    F: Fn(&V, &V) -> V,
{
    src.into_iter().fold(dst, |mut acc, (k, v)| {
        acc.entry(k).and_modify(|e| *e = f(e, &v)).or_insert(v);
        acc
    })
}

fn max_value_or<K: Hash, V: Ord>(h: HashMap<K, V>, default_value: V) -> V {
    h.into_values().max().unwrap_or(default_value)
}
