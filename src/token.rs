#[derive(Debug, PartialEq)]
pub enum Token {
    EOF,
    AdditionOperator,
    SubtractionOperator,
    MultiplicationOperator,
    DivisionOperator,
    NegationOperator,
    GreaterThanOperator,
    LessThanOperator,
    GreaterThanOrEqualToOperator,
    LessThanOrEqualToOperator,
    LogicalOrOperator,
    LogicalAndOperator,
    LogicalNotOperator,
    LogicalEqualOperator,
    Identifier {
        value: String,
    },
    Integer {
        i: i64,
    },
    Term {
        elements: Vec<Token>,
    },
    Expression {
        elements: Vec<Token>,
    },
    FunctionName {
        name: String,
    },
    FunctionArgs {
        args: Vec<String>,
    },
    FunctionBody {
        expressions: Vec<Token>,
    },
    Function {
        name: Option<Box<Token>>,
        args: Box<Token>,
        body: Box<Token>,
    },
    FunctionCall {
        name: String,
        parameters: Box<Token>,
    },
    Assignment {
        id: Box<Token>,
        expression: Box<Token>,
    },
    Program {
        expressions: Vec<Token>,
    },
}

impl Token {
    pub fn identifier(self) -> String {
        match self {
            Token::Identifier { value } => value,
            _ => panic!("are you sure {:?} is an identifier?", self),
        }
    }
}
