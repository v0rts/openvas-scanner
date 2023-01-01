//! Handles the prefix statement within Lexer
use crate::{
    error::SyntaxError,
    grouping_extension::Grouping,
    keyword_extension::Keywords,
    lexer::{Lexer, End},
    {AssignOrder, Statement},
    operation::Operation,
    token::{Category, Token},
    unexpected_end, unexpected_token,
    variable_extension::Variables,
};
pub(crate) trait Prefix {
    /// Handles statements before operation statements get handled.
    ///
    /// This must be called before handling postfix or infix operations to parse the initial statement.
    fn prefix_statement(
        &mut self,
        token: Token,
        abort: &impl Fn(Category) -> bool,
    ) -> Result<(End, Statement), SyntaxError>;
}

/// Is used to verify operations.
fn prefix_binding_power(token: Token) -> Result<u8, SyntaxError> {
    match token.category() {
        Category::Plus | Category::Minus | Category::Tilde | Category::Bang => Ok(21),
        _ => Err(unexpected_token!(token)),
    }
}


impl<'a> Lexer<'a> {
    /// Parses Operations that have an prefix (e.g. -1)
    fn parse_prefix_assign_operator(
        &mut self,
        assign: Category,
        token: Token,
    ) -> Result<Statement, SyntaxError> {
        let next = self
            .token()
            .ok_or_else(|| unexpected_end!("parsing prefix statement"))?;
        match self.parse_variable(next)? {
            (_, Statement::Variable(value)) => Ok(Statement::Assign(
                assign,
                AssignOrder::AssignReturn,
                Box::new(Statement::Variable(value)),
                Box::new(Statement::NoOp(None)),
            )),
            (_, Statement::Array(token, resolver)) => Ok(Statement::Assign(
                assign,
                AssignOrder::AssignReturn,
                Box::new(Statement::Array(token, resolver)),
                Box::new(Statement::NoOp(None)),
            )),
            _ => Err(unexpected_token!(token)),
        }
    }
}

impl<'a> Prefix for Lexer<'a> {
    fn prefix_statement(
        &mut self,
        token: Token,
        abort: &impl Fn(Category) -> bool,
    ) -> Result<(End, Statement), SyntaxError> {
        use End::*;
        let op = Operation::new(token).ok_or_else(|| unexpected_token!(token))?;
        match op {
            Operation::Operator(kind) => {
                let bp = prefix_binding_power(token)?;
                let (end, right) = self.statement(bp, abort)?;
                Ok((end, Statement::Operator(kind, vec![right])))
            }
            Operation::Primitive => Ok((Continue, Statement::Primitive(token))),
            Operation::Variable => self.parse_variable(token),
            Operation::Grouping(_) => self.parse_grouping(token),
            Operation::Assign(Category::MinusMinus) => self
                .parse_prefix_assign_operator(Category::MinusMinus, token)
                .map(|stmt| (Continue, stmt)),
            Operation::Assign(Category::PlusPlus) => self
                .parse_prefix_assign_operator(Category::PlusPlus, token)
                .map(|stmt| (Continue, stmt)),
            Operation::Assign(_) => Err(unexpected_token!(token)),
            Operation::Keyword(keyword) => self.parse_keyword(keyword, token),
            Operation::NoOp => Ok((Done(token.category()), Statement::NoOp(Some(token)))),
        }
    }
}

#[cfg(test)]
mod test {

    use crate::{
        AssignOrder,
        Statement,
        parse,
        token::{Base, Category, StringCategory, Token},
    };

    use Base::*;
    use Category::*;
    use Statement::*;

    fn result(code: &str) -> Statement {
        parse(code).next().unwrap().unwrap()
    }
    fn token(category: Category, start: usize, end: usize) -> Token {
        Token {
            category,
            position: (start, end),
        }
    }

    #[test]
    fn operations() {
        fn expected(category: Category) -> Statement {
            Statement::Operator(
                category,
                vec![Statement::Primitive(token(Number(Base10), 1, 2))],
            )
        }

        assert_eq!(result("-1;"), expected(Category::Minus));
        assert_eq!(result("+1;"), expected(Category::Plus));
        assert_eq!(result("~1;"), expected(Category::Tilde));
        assert_eq!(result("!1;"), expected(Category::Bang));
    }

    #[test]
    fn single_statement() {
        assert_eq!(result("1;"), Primitive(token(Number(Base10), 0, 1)));
        assert_eq!(
            result("'a';"),
            Primitive(token(String(StringCategory::Quotable), 1, 2))
        );
    }

    #[test]
    fn assignment_operator() {
        let expected = |assign_operator: Category| {
            Operator(
                Plus,
                vec![
                    Primitive(Token {
                        category: Number(Base10),
                        position: (0, 1),
                    }),
                    Operator(
                        Star,
                        vec![
                            Assign(
                                assign_operator,
                                AssignOrder::AssignReturn,
                                Box::new(Variable(Token {
                                    category: Identifier(None),
                                    position: (6, 7),
                                })),
                                Box::new(NoOp(None)),
                            ),
                            Primitive(Token {
                                category: Number(Base10),
                                position: (10, 11),
                            }),
                        ],
                    ),
                ],
            )
        };
        assert_eq!(result("1 + ++a * 1;"), expected(PlusPlus));
        assert_eq!(result("1 + --a * 1;"), expected(MinusMinus));
    }
    #[test]
    fn assignment_array_operator() {
        use AssignOrder::*;
        let expected = |assign_operator: Category| {
            Assign(
                assign_operator,
                AssignReturn,
                Box::new(Array(
                    Token {
                        category: Identifier(None),
                        position: (2, 3),
                    },
                    Some(Box::new(Primitive(Token {
                        category: Number(Base10),
                        position: (4, 5),
                    }))),
                )),
                Box::new(NoOp(None)),
            )
        };
        assert_eq!(result("++a[0];"), expected(PlusPlus));
        assert_eq!(result("--a[0];"), expected(MinusMinus));
    }
}
