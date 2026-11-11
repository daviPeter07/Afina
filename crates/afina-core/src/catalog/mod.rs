//! The tweak catalog — pure data, one file per [`Category`].
//!
//! To add a tweak, append a [`Tweak`] to the file of its category. The
//! `catalog_is_consistent` test below enforces the catalog rules.

mod games;
mod system;
mod visual;

use crate::tweak::{Category, Tweak};

/// Every tweak, in display order.
pub fn all() -> impl Iterator<Item = &'static Tweak> {
    games::TWEAKS.iter().chain(system::TWEAKS).chain(visual::TWEAKS)
}

pub fn by_category(category: Category) -> impl Iterator<Item = &'static Tweak> {
    all().filter(move |t| t.category == category)
}

pub fn recommended() -> impl Iterator<Item = &'static Tweak> {
    all().filter(|t| t.recommended)
}

pub fn find(id: &str) -> Option<&'static Tweak> {
    all().find(|t| t.id == id)
}

pub fn count() -> usize {
    all().count()
}

#[cfg(test)]
mod tests {
    use afina_platform::registry::Hive;

    use super::*;
    use crate::tweak::{Kind, Risk};

    #[test]
    fn catalog_is_consistent() {
        let mut ids: Vec<_> = all().map(|t| t.id).collect();
        ids.sort_unstable();
        ids.dedup();
        assert_eq!(ids.len(), count(), "ids duplicados");

        for t in all() {
            assert!(!t.title.is_empty() && !t.summary.is_empty(), "{}: título/resumo vazio", t.id);
            assert!(t.id.chars().all(|c| c.is_ascii_lowercase() || c == '_'), "{}: id deve ser snake_case", t.id);
            if let Kind::Registry(ops) = t.kind {
                assert!(!ops.is_empty(), "{}: sem operações", t.id);
                for o in ops {
                    assert_ne!(o.on, o.off, "{}: on == off em {}", t.id, o.name);
                    if o.hive == Hive::Hklm {
                        assert!(t.admin, "{} escreve em HKLM mas não marca admin", t.id);
                    }
                }
            }
            if t.recommended {
                assert_eq!(t.risk, Risk::Seguro, "{}: recomendado precisa ser Seguro", t.id);
                assert!(!t.admin, "{}: recomendado não pode exigir admin", t.id);
                assert_ne!(t.category, Category::Visual, "{}: recomendado não pode mudar a aparência", t.id);
            }
            if t.risk != Risk::Seguro {
                assert!(!t.details.is_empty(), "{}: ajuste com risco precisa explicar os detalhes", t.id);
            }
        }
    }

    #[test]
    fn lookup_helpers() {
        assert!(find("game_mode").is_some());
        assert!(find("nao_existe").is_none());
        assert!(recommended().count() > 0);
        assert_eq!(
            by_category(Category::Jogos).count()
                + by_category(Category::Sistema).count()
                + by_category(Category::Visual).count(),
            count()
        );
    }
}
