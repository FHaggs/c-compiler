use super::ast;

#[derive(Debug)]
pub struct Program {
    pub function_definition: FunctionDefinition,
}

#[derive(Debug)]
pub struct FunctionDefinition {
    pub name: Identifier,
    pub instructions: Vec<Instruction>,
}

#[derive(Debug)]
pub enum Identifier {
    FunctionName(String),
}

#[derive(Debug)]
pub enum Instruction {
    Mov(Operand, Operand),
    Ret,
}

#[derive(Debug)]
pub enum Operand {
    Imm(i32),
    Register(Register),
}

#[derive(Debug)]
pub enum Register {
    EAX,
    // Later you can add EBX, ECX, EDX, etc.
}

pub fn codegen(c_program: ast::Program) -> Program {
    let function_def = codegen_function(c_program.function_definition);
    Program {
        function_definition: function_def,
    }
}
fn codegen_function(c_function: ast::FunctionDefinition) -> FunctionDefinition {
    let instructions = codegen_stmt(c_function.body);
    let name = match c_function.name {
        ast::Identifier::FunctionName(name) => Identifier::FunctionName(name),
        // other => panic!("Expected FunctionName found {:?}", other),
    };
    FunctionDefinition { name, instructions }
}

fn codegen_stmt(stmt: ast::Statement) -> Vec<Instruction> {
    match stmt {
        ast::Statement::Return(expr) => {
            let operand = codegen_expression(expr);
            vec![
                Instruction::Mov(operand, Operand::Register(Register::EAX)),
                Instruction::Ret,
            ]
        }
    }
}

fn codegen_expression(expr: ast::Expression) -> Operand {
    match expr {
        ast::Expression::Constant(n) => Operand::Imm(n),
    }
}
