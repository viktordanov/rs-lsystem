use std::collections::HashMap;
use lsystem::token::TokenId;

use crate::arena::{Arena};
use crate::errors::LSystemError;
use crate::system::LSystem;
use crate::token::Token;

#[derive(Debug, Clone)]
struct ProductionRule {
    predecessor: TokenId,
    successor: Vec<TokenId>,
}

impl ProductionRule {
    pub fn new(predecessor: TokenId, successor: Vec<TokenId>) -> Self {
        Self {
            predecessor,
            successor,
        }
    }
}

#[derive(Default, Clone)]
pub struct LSystemBuilder {
    arena: Arena,
    axiom: Option<Vec<TokenId>>,
    rules: Vec<ProductionRule>,
}

impl LSystemBuilder {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn token<S: Into<String>>(&mut self, name: S) -> Result<TokenId, LSystemError> {
        let token = Token::new(name)?;
        Ok(self.arena.push_token(token))
    }

    fn validate_ids(&self, ids: &[TokenId]) -> Result<(), LSystemError> {
        for &id in ids {
            if !self.arena.is_valid(&id) {
                return Err(LSystemError::InvalidTokenId(id));
            }
        }

        Ok(())
    }

    pub fn production_rule(
        &mut self,
        predecessor: TokenId,
        successor: Vec<TokenId>,
    ) -> Result<(), LSystemError> {
        // Verify that all provided TokenId's correspond to a token in this LSystem.
        self.validate_ids(&[predecessor])?;
        self.validate_ids(&successor)?;

        // Add the rule to this system
        self.rules
            .push(ProductionRule::new(predecessor, successor));

        Ok(())
    }

    pub fn axiom(&mut self, axiom: Vec<TokenId>) -> Result<(), LSystemError> {
        self.validate_ids(axiom.as_slice())?;
        self.axiom = Some(axiom);

        Ok(())
    }

    pub fn finish(self) -> Result<LSystem, LSystemError> {
        let axiom = self.axiom.ok_or(LSystemError::MissingAxiom)?;

        // Construct a HashMap associating each variable with its corresponding transformation rule
        let mut rules_map = HashMap::new();

        for rule in self.rules.into_iter() {
            rules_map.insert(rule.predecessor, rule.successor);
        }

        // We also add constant production rules of the form P => P.
        for (id, _token) in self.arena.enumerate() {
            // no rule associated to this token, so its a constant token
            rules_map.entry(id).or_insert_with(|| vec![id]);
        }

        // If we set our system up correctly, it should be that each token
        // contributes exactly one rule, so we check for that here.
        assert_eq!(self.arena.len() as usize, rules_map.len());

        Ok(LSystem::new(self.arena, axiom, rules_map))
    }
}

fn render_tokens(arena: &Arena, tokens: &[TokenId]) -> String {
    let tokens = tokens.iter()
        .flat_map(|id| arena.get_token(id))
        .map(|token| token.name()).collect::<Vec<_>>();

    tokens.join("")
}

fn build_rules_string(rules: &[ProductionRule], arena: &Arena) -> String {
    let mut st = Vec::new();

    for rule in rules {
        st.push(format!(
            "{} => {}",
            render_tokens(arena, &[rule.predecessor]),
            render_tokens(arena, &rule.successor),
        ));
    }

    st.join(",")
}

impl std::fmt::Debug for LSystemBuilder {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> Result<(), std::fmt::Error> {
        f.debug_struct("LSystemBuilder")
            .field("arena", &self.arena)
            .field("axiom", &self.axiom)
            .field("rules", &build_rules_string(&self.rules, &self.arena))
            .finish()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_builder_invalid_token() -> Result<(), LSystemError> {
        let mut builder = LSystemBuilder::new();

        let _daisy = builder.token("daisy")?;

        // make sure we can't add a token with a space in it
        assert!(builder.token("space cadet").is_err());

        Ok(())
    }

    #[test]
    fn test_builder_axiom_and_transformation_rule_errors() -> Result<(), LSystemError> {
        let mut builder = LSystemBuilder::new();

        let x = builder.token("x")?;
        let y = builder.token("y")?;

        let mut some_other_builder = LSystemBuilder::new();

        // `x` won't be valid for an empty builder
        assert!(some_other_builder.axiom(vec![x]).is_err());

        let q = some_other_builder.token("q")?;

        // make sure `y` still isn't valid
        assert!(some_other_builder.axiom(vec![y]).is_err());

        // similarly trying to add a transformation rule should go badly.
        assert!(some_other_builder
            .production_rule(q, vec![x, y, q])
            .is_err());
        assert!(some_other_builder
            .production_rule(y, vec![q, q])
            .is_err());

        Ok(())
    }
}
