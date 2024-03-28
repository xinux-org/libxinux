use chrono::DateTime;
use fuzzy_matcher::FuzzyMatcher;
use fuzzy_matcher::skim::SkimMatcherV2;
use serde::{Deserialize, Serialize};
use crate::pkgs::std::{Arch, Repo};

#[derive(Serialize, Deserialize, Debug, Clone)]
#[serde(rename_all = "lowercase")]
pub enum Type {
    Aur,
    Std,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct Data {
    pub name: String,
    pub base: String,
    pub description: Option<String>,
    pub arch: Arch,
    pub repo: Option<Repo>,
    pub types: Type,
    pub version: String,
    pub url: String,
    pub author: Vec<String>,
    pub updated: DateTime<chrono::Utc>,
    pub install: String,
}

impl Data {
    pub fn fuzzy_search<T>(data: Vec<Self>, query: T) -> Vec<(Self, i64)>
    where
        T: AsRef<str> + Clone,
    {
        let matcher = SkimMatcherV2::default();

        let mut scores: Vec<(Self, i64)> = data
            .iter()
            .filter_map(|item| {
                matcher.fuzzy_match(&item.name, query.as_ref())
                    .map(|score| (item.clone(), score))
            })
            .collect();

        scores.sort_by(|a, b| b.1.cmp(&a.1));

        scores
    }
}
