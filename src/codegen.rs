use grammar::*;
use grammar::OperatorType::*;
use grammar::Statement::*;
use grammar::Expression::*;

trait Codegen {
    fn stringify(&self, minify: bool) -> String;
}

impl Codegen for OperatorType {
    fn stringify(&self, _: bool) -> String {
        match *self {
            FatArrow         => "=>",
            Accessor         => ".",
            New              => "new",
            Increment        => "++",
            Decrement        => "--",
            LogicalNot       => "!",
            BitwiseNot       => "~",
            Typeof           => "typeof",
            Void             => "void",
            Delete           => "delete",
            Multiplication   => "*",
            Division         => "/",
            Remainder        => "%",
            Exponent         => "**",
            Addition         => "+",
            Substraction     => "-",
            BitShiftLeft     => "<<",
            BitShiftRight    => ">>",
            UBitShiftRight   => ">>>",
            Lesser           => "<",
            LesserEquals     => "<=",
            Greater          => ">",
            GreaterEquals    => ">=",
            Instanceof       => "instanceof",
            In               => "in",
            StrictEquality   => "===",
            StrictInequality => "!==",
            Equality         => "==",
            Inequality       => "!=",
            BitwiseAnd       => "&",
            BitwiseXor       => "^",
            BitwiseOr        => "|",
            LogicalAnd       => "&&",
            LogicalOr        => "||",
            Conditional      => "?",
            Assign           => "=",
            Spread           => "...",
        }.to_string()
    }
}

impl Codegen for LiteralValue {
    fn stringify(&self, minify: bool) -> String {
        match *self {
            LiteralUndefined          => {
                if minify { "void 0" } else { "undefined" }.to_string()
            },
            LiteralNull               => "null".to_string(),
            LiteralTrue               => {
                if minify { "!0" } else { "true" }.to_string()
            },
            LiteralFalse              => {
                if minify { "!1" } else { "false" }.to_string()
            },
            LiteralInteger(ref num)   => num.to_string(),
            LiteralFloat(ref num)     => num.to_string(),
            LiteralString(ref string) => format!("{:?}", string)
        }
    }
}

impl Codegen for ObjectMember {
    fn stringify(&self, minify: bool) -> String {
        match *self {
            ObjectMember::Shorthand {
                ref key
            } => key.clone(),

            ObjectMember::Literal {
                ref key,
                ref value,
            } => format!("{}: {}",
                key,
                value.stringify(minify)
            ),

            ObjectMember::Computed {
                ref key,
                ref value,
            } => format!("[{}]: {}",
                key.stringify(minify),
                value.stringify(minify)
            )
        }
    }
}

impl Codegen for MemberKey {
    fn stringify(&self, minify: bool) -> String {
        match *self {
            MemberKey::Literal(ref string) => string.clone(),
            MemberKey::Computed(ref expr)  => expr.stringify(minify),
        }
    }
}

impl Codegen for Parameter {
    fn stringify(&self, minify: bool) -> String {
        let Parameter { ref name } = *self;
        name.clone()
    }
}

impl Codegen for Expression {
    fn stringify(&self, minify: bool) -> String {
        match *self {

            IdentifierExpression(ref ident) => ident.clone(),

            LiteralExpression(ref literal)  => literal.stringify(minify),

            ArrayExpression(ref items) => {
                format!("[{}]", items.into_iter()
                    .map(|item| item.stringify(minify))
                    .collect::<Vec<String>>()
                    .join(", ")
                )
            },

            ObjectExpression(ref members) => {
                format!("{{{}}}", members.into_iter()
                    .map(|member| member.stringify(minify))
                    .collect::<Vec<String>>()
                    .join(", ")
                )
            },

            MemberExpression {
                ref object,
                ref property,
            } => {
                format!("{}.{}",
                    object.stringify(minify),
                    property.stringify(minify),
                )
            },

            CallExpression {
                ref callee,
                ref arguments,
            } => {
                format!("{}({})",
                    callee.stringify(minify),
                    arguments.into_iter()
                        .map(|argument| argument.stringify(minify))
                        .collect::<Vec<String>>()
                        .join(", ")
                )
            },

            BinaryExpression {
                ref left,
                ref operator,
                ref right,
            } => {
                format!("{} {} {}",
                    left.stringify(minify),
                    operator.stringify(minify),
                    right.stringify(minify)
                )
            },

            PrefixExpression {
                ref operator,
                ref operand,
            } => {
                format!("{}{}",
                    operator.stringify(minify),
                    operand.stringify(minify)
                )
            },

            ConditionalExpression {
                ref test,
                ref consequent,
                ref alternate,
            } => {
                format!("{} ? {} : {}",
                    test.stringify(minify),
                    consequent.stringify(minify),
                    alternate.stringify(minify),
                )
            },

            ArrowFunctionExpression {
                ref params,
                ref body,
            } => {
                let params = if params.len() == 1 {
                    params[0].stringify(minify)
                } else {
                    format!("({})", params.into_iter()
                        .map(|parameter| parameter.stringify(minify))
                        .collect::<Vec<String>>()
                        .join(", ")
                    )
                };

                format!("{} => {}", params, body.stringify(minify))
            },

            PostfixExpression {
                ref operator,
                ref operand,
            } => {
                format!("{}{}",
                    operand.stringify(minify),
                    operator.stringify(minify)
                )
            },

            _ => '💀'.to_string(),
        }
    }
}

impl Codegen for VariableDeclarationKind {
    fn stringify(&self, _: bool) -> String {
        match *self {
            VariableDeclarationKind::Var   => "var",
            VariableDeclarationKind::Let   => "let",
            VariableDeclarationKind::Const => "const",
        }.to_string()
    }
}

impl Codegen for ClassMember {
    fn stringify(&self, minify: bool) -> String {
        match *self {

            ClassMember::ClassConstructor {
                ref params,
                ref body,
            } => {
                let mut code = format!("constructor({}) {{", params.into_iter()
                    .map(|parameter| parameter.stringify(minify))
                    .collect::<Vec<String>>()
                    .join(", ")
                );
                if !minify {
                    code.push('\n');
                }
                statements(&mut code, &body, minify);
                code.push('}');
                code
            },

            ClassMember::ClassMethod {
                is_static,
                ref name,
                ref params,
                ref body,
            } => {
                let mut code = if is_static {
                    "static ".to_string()
                } else {
                    String::new()
                };

                code.push_str(name);
                code.push_str(&format!("({}) {{", params.into_iter()
                    .map(|parameter| parameter.stringify(minify))
                    .collect::<Vec<String>>()
                    .join(", ")
                ));
                if !minify {
                    code.push('\n');
                }
                statements(&mut code, &body, minify);
                code.push('}');
                code
            },

            ClassMember::ClassProperty {
                is_static,
                ref name,
                ref value,
            } => {
                let mut code = if is_static {
                    "static ".to_string()
                } else {
                    String::new()
                };

                code.push_str(&format!("{} = {};", name, value.stringify(minify)));
                code
            }
        }
    }
}

impl Codegen for Statement {
    fn stringify(&self, minify: bool) -> String {
        match *self {

            ExpressionStatement(ref expr) => expr.stringify(minify),

            ReturnStatement(ref expr) => {
                format!("return {}", expr.stringify(minify))
            },

            VariableDeclarationStatement {
                ref kind,
                ref declarations,
            } => {
                format!("{} {};", kind.stringify(minify), declarations.into_iter()
                    .map(|&(ref name, ref value)| {
                        format!("{} = {}", name, value.stringify(minify))
                    })
                    .collect::<Vec<String>>()
                    .join(", ")
                )
            },

            FunctionStatement {
                ref name,
                ref params,
                ref body,
            } => {
                let params = params.into_iter()
                    .map(|parameter| parameter.stringify(minify))
                    .collect::<Vec<String>>()
                    .join(", ");

                let mut code = format!("function {}({}) {{", name, params);
                if !minify {
                    code.push('\n');
                }
                statements(&mut code, &body, minify);
                code.push('}');
                code
            },

            IfStatement {
                ref test,
                ref consequent,
                ref alternate,
            } => {
                let mut code = format!("if ({}) {}",
                    test.stringify(minify),
                    consequent.stringify(minify)
                );

                if let &Some(ref alternate) = alternate {
                    code.push_str(
                        &format!(" else {}", alternate.stringify(minify))
                    );
                };

                code
            },

            WhileStatement {
                ref test,
                ref body,
            } => {
                format!("while ({}) {}",
                    test.stringify(minify),
                    body.stringify(minify)
                )
            },

            BlockStatement {
                ref body
            } => {
                let mut code = '{'.to_string();
                if !minify {
                    code.push('\n');
                }

                statements(&mut code, body, minify);
                code.push('}');
                code
            },

            ClassStatement {
                ref name,
                ref extends,
                ref body,
            } => {
                let mut code = format!("class {} ", name);
                if let &Some(ref super_class) = extends {
                    code.push_str(&format!("extends {} ", super_class));
                }
                code.push_str("{\n");
                for member in body {
                    code.push_str(&member.stringify(minify));
                    code.push('\n');
                }
                code.push('}');
                code
            },
        }
    }
}

#[inline(always)]
fn statements(code: &mut String, body: &Vec<Statement>, minify: bool) {
    for statement in body {
        code.push_str(&statement.stringify(minify));
        if !minify {
            code.push('\n');
        }
    }
}

pub fn generate_code(program: Program, minify: bool) -> String {
    let mut code = String::new();

    statements(&mut code, &program.body, minify);

    code
}