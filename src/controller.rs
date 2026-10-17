//! Tiny typed controller: it supplies legal choices and owns Rust syntax.

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Need {
    Root,
    Expression,
}

impl Need {
    pub(crate) fn id(self) -> u32 {
        match self {
            Self::Root => 0,
            Self::Expression => 1,
        }
    }
}

pub struct Step {
    pub need: Need,
    /// Stable candidate IDs, including entries that are currently illegal.
    pub legal: Vec<u8>,
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
enum Root {
    Function,
    Block,
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
enum Expression {
    Zero,
    One,
    Add,
}

/// Only `apply` changes the controller state. Model code cannot emit punctuation.
#[derive(Default)]
pub struct Controller {
    root: Option<Root>,
    expression: Option<Expression>,
}

impl Controller {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn step(&self) -> Option<Step> {
        match (self.root, self.expression) {
            (None, _) => Some(Step {
                need: Need::Root,
                legal: vec![1, 1],
            }),
            (Some(Root::Function), None) => Some(Step {
                need: Need::Expression,
                legal: vec![1, 1, 0],
            }),
            (Some(Root::Block), None) => Some(Step {
                need: Need::Expression,
                legal: vec![1, 1, 1],
            }),
            (Some(_), Some(_)) => None,
        }
    }

    pub fn apply(&mut self, choice: usize) -> Result<(), &'static str> {
        let step = self.step().ok_or("generation is complete")?;
        if step.legal.get(choice) != Some(&1) {
            return Err("illegal action for current need");
        }
        match step.need {
            Need::Root => {
                self.root = Some(if choice == 0 {
                    Root::Function
                } else {
                    Root::Block
                })
            }
            Need::Expression => {
                self.expression = Some(match choice {
                    0 => Expression::Zero,
                    1 => Expression::One,
                    2 => Expression::Add,
                    _ => return Err("unknown expression action"),
                })
            }
        }
        Ok(())
    }

    pub fn render(&self) -> Result<String, &'static str> {
        let root = self.root.ok_or("root is incomplete")?;
        let expression = match self.expression.ok_or("expression is incomplete")? {
            Expression::Zero => "0",
            Expression::One => "1",
            Expression::Add => "1 + 1",
        };
        Ok(match root {
            Root::Function => format!("fn generated() -> i32 {{ {expression} }}\n"),
            Root::Block => format!("{{ {expression} }}"),
        })
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn function_rejects_disallowed_expression() {
        let mut controller = Controller::new();
        assert!(controller.render().is_err());
        controller.apply(0).unwrap();
        assert_eq!(controller.step().unwrap().legal, vec![1, 1, 0]);
        assert!(controller.apply(2).is_err());
        controller.apply(1).unwrap();
        assert_eq!(
            controller.render().unwrap(),
            "fn generated() -> i32 { 1 }\n"
        );
        assert!(controller.apply(0).is_err());
    }

    #[test]
    fn block_can_contain_addition() {
        let mut controller = Controller::new();
        controller.apply(1).unwrap();
        controller.apply(2).unwrap();
        assert_eq!(controller.render().unwrap(), "{ 1 + 1 }");
    }
}
