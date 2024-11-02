#[derive(Debug, PartialEq)]
pub enum Token {
    Number(f64),
    Operator(char),
    LParen, 
    RParen,
}

impl std::fmt::Display for Token {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        match self {
            Token::Number(num) => write!(f, "{}", num),
            Token::Operator(op) => write!(f, "{}", op),
            Token::RParen => write!(f, ")"),
            Token::LParen => write!(f, "("),
        }
    }
}

#[derive(Debug)]
pub enum TokenError {
    InvalidCharacter(char),
    ParseError(String),
}

impl std::fmt::Display for TokenError {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        match self {
            TokenError::InvalidCharacter(c) => write!(f, "Invalid character: {}", c),
            TokenError::ParseError(num) => write!(f, "Failed to parse number: {}", num),
        }
    }
}

pub fn tokenize(input: &str) -> Result<Vec<Token>, TokenError> {
    let mut tokens = Vec::new();
    let mut iter = input.chars().peekable();

    while let Some(&c) = iter.peek() {
        match c {
            '0'..='9' | '.' /*| '-' if matches!(tokens.last(), Some(Token::Operator(_)) | Some(Token::LParen) | None)*/ => parse_number_or_negative(&mut iter, &mut tokens)?,
            '(' | ')' | '[' | ']' => parse_parenthesis(&mut iter, &mut tokens)?,
            '+' | '-' | '*' | '/' | '^' => parse_operator(&mut iter, &mut tokens)?,
            ' ' | '\t' | '\n' => {
                iter.next();
            },
            _ => return Err(TokenError::InvalidCharacter(c)),
        }
    }
    Ok(tokens)
}

fn parse_number_or_negative(iter: &mut std::iter::Peekable<std::str::Chars>, tokens: &mut Vec<Token>) -> Result<(), TokenError> {
    let mut num = String::new();
    let mut is_negative = false;

    if let Some(&c) = iter.peek() {
        if c == '-' {
            is_negative = true;
            iter.next();
        }
    }

    while let Some(&c) = iter.peek() {
        match c {
            '0'..='9' | '.' => {
                num.push(c);
                iter.next();
            },
            _ => break,
        }
    }

    if let Ok(num) = if is_negative { num.parse::<f64>().map(|n: f64| -n) } else { num.parse::<f64>() } {
        tokens.push(Token::Number(num));
    } else {
        return Err(TokenError::ParseError(num));
    }
    Ok(())
}

fn parse_operator(iter: &mut std::iter::Peekable<std::str::Chars>, tokens: &mut Vec<Token>) -> Result<(), TokenError> {
    if let Some(&op) = iter.peek() {
        tokens.push(Token::Operator(op));
        iter.next();
    }
    Ok(())
}

fn parse_parenthesis(iter: &mut std::iter::Peekable<std::str::Chars>, tokens: &mut Vec<Token>) -> Result<(), TokenError> {
    if let Some(&paren) = iter.peek() {
        match paren {
            '(' => tokens.push(Token::LParen),
            ')' => tokens.push(Token::RParen),
            // these are the same, its just too differentiate between layered parenthesis
            '[' => tokens.push(Token::LParen),
            ']' => tokens.push(Token::RParen),
            _ => return Err(TokenError::InvalidCharacter(paren)),
        }
        iter.next();
    }
    Ok(())
}

#[derive(Debug)]
pub enum AST {
    Number(f64),
    BinOp(char, Box<AST>, Box<AST>),
    Neg(Box<AST>),
}

impl std::fmt::Display for AST {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        match self {
            AST::Number(num) => write!(f, "{}", num),
            AST::BinOp(op, lhs, rhs) => write!(f, "({} {} {})", lhs, op, rhs),
            AST::Neg(expr) => write!(f, "-{}", expr),
        }
    }
}

#[derive(Debug)]
pub enum ASTError {
    ExpectedFound(Token, Token),
    UnexpectedEnd,
}


impl std::fmt::Display for ASTError {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        match self {
            ASTError::ExpectedFound(expected, found) => write!(f, "Expected '{}', found '{}'", expected, found),
            ASTError::UnexpectedEnd => write!(f, "Unexpected end of input"),
        }
    }
}

pub fn parse(tokens: Vec<Token>) -> Result<AST, ASTError> {
    let mut iter = tokens.iter().peekable();
    let ast = parse_expr(&mut iter)?;
    if iter.next().is_some() {
        Err(ASTError::UnexpectedEnd)
    } else {
        Ok(ast)
    }
}

fn parse_term(iter: &mut std::iter::Peekable<std::slice::Iter<Token>>) -> Result<AST, ASTError> {
    match iter.next() {
        Some(&Token::Number(num)) => Ok(AST::Number(num)),
        Some(&Token::LParen) => {
            let expr = parse_expr(iter)?;
            if let Some(&Token::RParen) = iter.next() {
                Ok(expr)
            } else {
                Err(ASTError::ExpectedFound(Token::RParen, Token::LParen))
            }
        },
        _ => Err(ASTError::UnexpectedEnd),
    }
}

fn parse_exponent(iter: &mut std::iter::Peekable<std::slice::Iter<Token>>) -> Result<AST, ASTError> {
    let mut lhs = parse_term(iter)?;

    while let Some(&token) = iter.peek() {
        match token {
            Token::Operator('^') => {
                iter.next();
                let rhs = parse_term(iter)?;
                lhs = AST::BinOp('^', Box::new(lhs), Box::new(rhs));
            },
            _ => break,
        }
    }

    Ok(lhs)
}

fn parse_factor(iter: &mut std::iter::Peekable<std::slice::Iter<Token>>) -> Result<AST, ASTError> {
    let mut lhs = if let Some(&Token::Operator('-')) = iter.peek() {
        iter.next();
        let expr = parse_exponent(iter)?;
        AST::Neg(Box::new(expr))
    } else {
        parse_exponent(iter)?
    };

    while let Some(&token) = iter.peek() {
        match token {
            Token::Operator(op @ ('*' | '/')) => {
                iter.next();
                let rhs = match iter.peek() {
                    Some(&Token::Operator('-')) => {
                        iter.next();
                        let expr = parse_exponent(iter)?;
                        AST::Neg(Box::new(expr))
                    }
                    _ => parse_exponent(iter)?,
                };
                lhs = AST::BinOp(*op, Box::new(lhs), Box::new(rhs));
            },
            _ => break,
        }
    }

    Ok(lhs)
}

fn parse_expr(iter: &mut std::iter::Peekable<std::slice::Iter<Token>>) -> Result<AST, ASTError> {
    let mut lhs = parse_factor(iter)?;

    while let Some(&token) = iter.peek() {
        match token {
            Token::Operator(op@ ('+' | '-')) => {
                iter.next();
                let rhs = parse_factor(iter)?;
                lhs = AST::BinOp(*op, Box::new(lhs), Box::new(rhs));
            },
            _ => break,
        }
    }

    Ok(lhs)
}

pub fn eval(ast: &AST) -> f64 {
    match ast {
        AST::Number(num) => *num,
        AST::BinOp(op, lhs, rhs) => {
            let lhs_val = eval(lhs);
            let rhs_val = eval(rhs);
            match op {
                '+' => lhs_val + rhs_val,
                '-' => lhs_val - rhs_val,
                '*' => lhs_val * rhs_val,
                '/' => lhs_val / rhs_val,
                '^' => lhs_val.powf(rhs_val),
                _ => unreachable!(),
            }
        },
        AST::Neg(expr) => -eval(expr),
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_tokenize_numbers() {
        let input = "42 3.14 7";
        let tokens = tokenize(input).unwrap();
        assert_eq!(
            tokens,
            vec![
                Token::Number(42.0),
                Token::Number(3.14),
                Token::Number(7.0)
            ]
        );
    }

    #[test]
    fn test_tokenize_operators() {
        let input = "+ - * /";
        let tokens = tokenize(input).unwrap();
        assert_eq!(
            tokens,
            vec![
                Token::Operator('+'),
                Token::Operator('-'),
                Token::Operator('*'),
                Token::Operator('/')
            ]
        );
    }

    #[test]
    fn test_tokenize_parentheses() {
        let input = "( )";
        let tokens = tokenize(input).unwrap();
        assert_eq!(tokens, vec![Token::LParen, Token::RParen]);
    }

    #[test]
    fn test_tokenize_invalid_character() {
        let input = "42 &";
        let result = tokenize(input);
        assert!(result.is_err());
    }

    #[test]
    fn test_eval_simple_expression() {
        let tokens = tokenize("3 + 4").unwrap();
        let ast = parse(tokens).unwrap();
        let result = eval(&ast);
        assert_eq!(result, 7.0);
    }

    #[test]
    fn test_eval_nested_expression() {
        let tokens = tokenize("(1 + 2) * 3").unwrap();
        let ast = parse(tokens).unwrap();
        let result = eval(&ast);
        assert_eq!(result, 9.0);
    }

    #[test]
    fn test_eval_negative_numbers() {
        let tokens = tokenize("-3 + 4").unwrap();
        let ast = parse(tokens).unwrap();
        let result = eval(&ast);
        assert_eq!(result, 1.0);
    }

    #[test]
    fn test_double_negative() {
        let tokens = tokenize("-(-3)").unwrap();
        let ast = parse(tokens).unwrap();
        let result = eval(&ast);
        assert_eq!(result, 3.0);
    }

    #[test]
    fn test_really_long_equation() {
        let tokens = tokenize("(15 + 6) * (3 - 1) + 4 * (10 / 2) - 8 + (5 * -2 - 3) * (6 / 2) + 12 - (4 + 1) * (3 - 2) + 9 * (8 / -4) - 7 + -5 + (2 - 9) * -3 + (12 / 4 - 5)").unwrap();
        let ast = parse(tokens).unwrap();
        let result = eval(&ast);
        assert_eq!(result, 11.0);   
    }

    #[test]
    fn pemdas1() {
        let tokens = tokenize("-24 + 3 + 8 - [(-1 + 13)^2 - (2 + 13)]").unwrap();
        let ast = parse(tokens).unwrap();
        let result = eval(&ast);
        assert_eq!(result, -142.0);
    }

    #[test]
    fn pemdas2() {
        let tokens = tokenize("3 + 4 * 2 / (1 - 5)^2").unwrap();
        let ast = parse(tokens).unwrap();
        let result = eval(&ast);
        assert_eq!(result, 3.5);
    }
}
