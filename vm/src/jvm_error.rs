#[derive(Debug)]
pub enum JVMError {
    IndexOutOfBounds {
        index: usize,
        max: usize,
    },
    TypeMismatch {
        expected: String,
        found: String,
    },
    StackUnderflow,
    StackOverflow,
    InvalidLocalVariable {
        index: usize,
    },
    NullReference,

    ConstantPoolIndexOutOfBounds {
        index: u16,
        max: usize,
    },
    InvalidConstantType {
        expected: &'static str,
        found: &'static str,
    },
    StringIndexInvalid(u16),
    ClassNameIndexInvalid(u16),
    ConstantPoolError(String),

    DivisionByZero,
    ArithmeticOverflow,
    InvalidOperandType {
        expected: &'static str,
        found: &'static str,
    },
    InsufficientOperands {
        required: usize,
        found: usize,
    },

    InvalidOffset(i32),

    MethodNotFound {
        class: String,
        name: String,
        descriptor: String,
    },
    IncompatibleClass {
        expected: String,
        found: String,
    },
    AbstractMethodCall {
        class: String,
        name: String,
        descriptor: String,
    },

    UncaughtException(String),

    ClassCastException(String),

    IllegalMonitorStateException(String),

    NoFrame,
    Other(String),
}
