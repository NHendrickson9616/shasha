//! Typed action routing, live symbol slots, bounded naming, and Rust rendering.

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Need {
    Root,
    NoParameterExpression,
    ParameterExpression,
    TwoParameterExpression,
    SymbolPointer,
    NameStart,
    NameContinue,
}

impl Need {
    pub(crate) fn id(self) -> u32 {
        match self {
            Self::Root => 0,
            Self::NoParameterExpression => 1,
            Self::ParameterExpression => 2,
            Self::TwoParameterExpression => 3,
            Self::SymbolPointer => 4,
            Self::NameStart => 5,
            Self::NameContinue => 6,
        }
    }

    fn fixed_candidate_count(self) -> Option<usize> {
        match self {
            Self::Root | Self::TwoParameterExpression => Some(3),
            Self::NoParameterExpression => Some(4),
            Self::ParameterExpression => Some(2),
            Self::NameStart => Some(26),
            Self::NameContinue => Some(27),
            Self::SymbolPointer => None,
        }
    }

    pub fn label(self) -> &'static str {
        match self {
            Self::Root => "Function signature head",
            Self::NoParameterExpression => "No-parameter expression head",
            Self::ParameterExpression => "One-parameter expression head",
            Self::TwoParameterExpression => "Two-parameter expression head",
            Self::SymbolPointer => "Live symbol pointer",
            Self::NameStart => "Name character head",
            Self::NameContinue => "Name continuation head",
        }
    }

    fn action_label(self, choice: usize) -> Option<&'static str> {
        match (self, choice) {
            (Self::Root, 0) => Some("no parameters"),
            (Self::Root, 1) => Some("one i32 parameter"),
            (Self::Root, 2) => Some("two i32 parameters"),
            (Self::NoParameterExpression, 0) => Some("integer literal 0"),
            (Self::NoParameterExpression, 1) => Some("integer literal 1"),
            (Self::NoParameterExpression, 2) => Some("add 1 + 1"),
            (Self::NoParameterExpression, 3) => Some("subtract 1 - 1"),
            (Self::ParameterExpression, 0) => Some("return a parameter"),
            (Self::ParameterExpression, 1) => Some("increment a parameter"),
            (Self::TwoParameterExpression, 0) => Some("wrapping addition"),
            (Self::TwoParameterExpression, 1) => Some("wrapping subtraction"),
            (Self::TwoParameterExpression, 2) => Some("return a parameter"),
            _ => None,
        }
    }
}

/// A stable internal symbol identity, independent of its generated spelling.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct SymbolId(pub u32);

pub struct Step {
    pub need: Need,
    /// Ordered live parameter slots, used only by the dynamic pointer head.
    pub symbols: Vec<SymbolId>,
}

impl Step {
    pub fn candidate_count(&self) -> usize {
        self.need
            .fixed_candidate_count()
            .unwrap_or(self.symbols.len())
    }

    pub fn describe(&self, choice: usize) -> String {
        match self.need {
            Need::SymbolPointer => self.symbols.get(choice).map_or_else(
                || "invalid pointer".into(),
                |id| format!("parameter @{}", id.0),
            ),
            Need::NameStart | Need::NameContinue if choice < 26 => {
                format!("character '{}'", (b'a' + choice as u8) as char)
            }
            Need::NameContinue if choice == 26 => "finish identifier".into(),
            _ => self
                .need
                .action_label(choice)
                .unwrap_or("invalid action")
                .into(),
        }
    }
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
enum Root {
    NoParameter,
    OneParameter,
    TwoParameters,
}

impl Root {
    fn parameter_count(self) -> usize {
        match self {
            Self::NoParameter => 0,
            Self::OneParameter => 1,
            Self::TwoParameters => 2,
        }
    }
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
enum Expression {
    Zero,
    One,
    ConstantAdd,
    ConstantSub,
    Parameter,
    Increment,
    WrappingAdd,
    WrappingSub,
}

impl Expression {
    fn pointer_count(self) -> usize {
        match self {
            Self::Zero | Self::One | Self::ConstantAdd | Self::ConstantSub => 0,
            Self::Parameter | Self::Increment => 1,
            Self::WrappingAdd | Self::WrappingSub => 2,
        }
    }
}

/// Names are generated after structure. Symbols are allocated before spelling.
#[derive(Default)]
pub struct Controller {
    root: Option<Root>,
    expression: Option<Expression>,
    operands: Vec<SymbolId>,
    names: Vec<String>,
    draft: String,
}

impl Controller {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn step(&self) -> Option<Step> {
        let root = match self.root {
            Some(root) => root,
            None => {
                return Some(Step {
                    need: Need::Root,
                    symbols: vec![],
                });
            }
        };
        let expression = match self.expression {
            Some(expression) => expression,
            None => {
                let need = match root {
                    Root::NoParameter => Need::NoParameterExpression,
                    Root::OneParameter => Need::ParameterExpression,
                    Root::TwoParameters => Need::TwoParameterExpression,
                };
                return Some(Step {
                    need,
                    symbols: vec![],
                });
            }
        };
        if self.operands.len() < expression.pointer_count() {
            let symbols = (1..=root.parameter_count())
                .map(|id| SymbolId(id as u32))
                .collect();
            return Some(Step {
                need: Need::SymbolPointer,
                symbols,
            });
        }
        if self.names.len() < root.parameter_count() + 1 {
            let need = if self.draft.is_empty() {
                Need::NameStart
            } else {
                Need::NameContinue
            };
            return Some(Step {
                need,
                symbols: vec![],
            });
        }
        None
    }

    pub fn apply(&mut self, choice: usize) -> Result<(), &'static str> {
        let step = self.step().ok_or("generation is complete")?;
        if choice >= step.candidate_count() {
            return Err("action is outside the selected head");
        }
        match step.need {
            Need::Root => {
                self.root = Some(match choice {
                    0 => Root::NoParameter,
                    1 => Root::OneParameter,
                    2 => Root::TwoParameters,
                    _ => return Err("invalid signature action"),
                })
            }
            Need::NoParameterExpression => {
                self.expression = Some(match choice {
                    0 => Expression::Zero,
                    1 => Expression::One,
                    2 => Expression::ConstantAdd,
                    3 => Expression::ConstantSub,
                    _ => return Err("invalid expression action"),
                })
            }
            Need::ParameterExpression => {
                self.expression = Some(match choice {
                    0 => Expression::Parameter,
                    1 => Expression::Increment,
                    _ => return Err("invalid expression action"),
                })
            }
            Need::TwoParameterExpression => {
                self.expression = Some(match choice {
                    0 => Expression::WrappingAdd,
                    1 => Expression::WrappingSub,
                    2 => Expression::Parameter,
                    _ => return Err("invalid expression action"),
                })
            }
            Need::SymbolPointer => self.operands.push(step.symbols[choice]),
            Need::NameStart | Need::NameContinue => {
                if choice == 26 {
                    self.names.push(std::mem::take(&mut self.draft));
                } else {
                    self.draft.push((b'a' + choice as u8) as char);
                    if self.draft.len() == 3 {
                        self.names.push(std::mem::take(&mut self.draft));
                    }
                }
            }
        }
        Ok(())
    }

    fn spelling(&self, id: SymbolId) -> Result<String, &'static str> {
        let suffix = self
            .names
            .get(id.0 as usize)
            .ok_or("symbol has no generated name")?;
        // The prefix excludes Rust keywords; the ID prevents collisions.
        Ok(format!("n_{suffix}_{}", id.0))
    }

    pub fn render(&self) -> Result<String, &'static str> {
        if self.step().is_some() {
            return Err("generation is incomplete");
        }
        let root = self.root.ok_or("root is incomplete")?;
        let expression = self.expression.ok_or("expression is incomplete")?;
        let operand = |index| {
            self.operands
                .get(index)
                .copied()
                .ok_or("missing symbol pointer")
                .and_then(|id| self.spelling(id))
        };
        let body = match expression {
            Expression::Zero => "0".to_owned(),
            Expression::One => "1".to_owned(),
            Expression::ConstantAdd => "1 + 1".to_owned(),
            Expression::ConstantSub => "1 - 1".to_owned(),
            Expression::Parameter => operand(0)?,
            Expression::Increment => format!("{}.wrapping_add(1)", operand(0)?),
            Expression::WrappingAdd => format!("{}.wrapping_add({})", operand(0)?, operand(1)?),
            Expression::WrappingSub => format!("{}.wrapping_sub({})", operand(0)?, operand(1)?),
        };
        let parameters = (1..=root.parameter_count())
            .map(|id| {
                self.spelling(SymbolId(id as u32))
                    .map(|name| format!("{name}: i32"))
            })
            .collect::<Result<Vec<_>, _>>()?
            .join(", ");
        Ok(format!(
            "fn {}({parameters}) -> i32 {{ {body} }}\n",
            self.spelling(SymbolId(0))?
        ))
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::validation::validate_rust;

    fn finish(controller: &mut Controller) {
        while let Some(step) = controller.step() {
            match step.need {
                Need::SymbolPointer | Need::NameStart => controller.apply(0).unwrap(),
                Need::NameContinue => controller.apply(26).unwrap(),
                _ => panic!("unexpected structural need: {:?}", step.need),
            }
        }
    }

    #[test]
    fn names_are_unique_even_when_the_model_repeats_a_character() {
        let mut controller = Controller::new();
        controller.apply(2).unwrap();
        controller.apply(0).unwrap();
        assert_eq!(
            controller.step().unwrap().symbols,
            vec![SymbolId(1), SymbolId(2)]
        );
        controller.apply(1).unwrap(); // first operand is the second parameter
        controller.apply(0).unwrap(); // second operand is the first parameter
        finish(&mut controller);
        assert_eq!(
            controller.render().unwrap(),
            "fn n_a_0(n_a_1: i32, n_a_2: i32) -> i32 { n_a_2.wrapping_add(n_a_1) }\n"
        );
    }

    #[test]
    fn invalid_actions_and_incomplete_names_are_rejected() {
        let mut controller = Controller::new();
        assert!(controller.render().is_err());
        controller.apply(1).unwrap();
        controller.apply(0).unwrap();
        assert!(controller.apply(1).is_err()); // only one live parameter
        controller.apply(0).unwrap();
        assert!(controller.apply(26).is_err()); // first name character cannot be EOS
        controller.apply(5).unwrap();
        assert!(controller.render().is_err());
        finish(&mut controller);
        assert!(controller.render().unwrap().contains("n_f_0"));
    }

    #[test]
    fn generated_addition_runs_with_pointer_selected_operands() {
        let mut controller = Controller::new();
        controller.apply(2).unwrap(); // two parameters
        controller.apply(0).unwrap(); // wrapping addition
        controller.apply(0).unwrap(); // @1
        controller.apply(1).unwrap(); // @2
        finish(&mut controller);
        let code = controller.render().unwrap();
        let path = std::env::temp_dir().join(format!("shasha-pointer-add-{}", std::process::id()));
        let source = format!(
            "{code}\nfn main() {{ assert_eq!(n_a_0(20, 22), 42); assert_eq!(n_a_0(-5, 2), -3); }}"
        );
        use std::io::Write;
        let mut child = std::process::Command::new("rustc")
            .args(["--edition=2024", "-o"])
            .arg(&path)
            .arg("-")
            .stdin(std::process::Stdio::piped())
            .stderr(std::process::Stdio::piped())
            .spawn()
            .unwrap();
        child
            .stdin
            .take()
            .unwrap()
            .write_all(source.as_bytes())
            .unwrap();
        let output = child.wait_with_output().unwrap();
        assert!(
            output.status.success(),
            "{}",
            String::from_utf8_lossy(&output.stderr)
        );
        let result = std::process::Command::new(&path).status().unwrap();
        let _ = std::fs::remove_file(path);
        assert!(result.success());
    }

    #[test]
    fn constant_add_and_subtract_are_parallel_actions() {
        for (choice, expression) in [(2, "1 + 1"), (3, "1 - 1")] {
            let mut controller = Controller::new();
            controller.apply(0).unwrap();
            assert_eq!(controller.step().unwrap().candidate_count(), 4);
            controller.apply(choice).unwrap();
            finish(&mut controller);
            let code = controller.render().unwrap();
            assert!(code.contains(expression));
            validate_rust(&code).unwrap();
        }
    }

    #[test]
    fn every_structural_path_compiles() {
        for root in 0..3 {
            for expression in 0..4 {
                let mut controller = Controller::new();
                controller.apply(root).unwrap();
                if controller.step().unwrap().candidate_count() <= expression {
                    continue;
                }
                controller.apply(expression).unwrap();
                finish(&mut controller);
                validate_rust(&controller.render().unwrap()).unwrap();
            }
        }
    }
}
