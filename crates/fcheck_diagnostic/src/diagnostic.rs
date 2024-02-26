/// Core "diagnostics" type used by the reporting system.
#[derive(Debug, PartialEq, Eq, Clone)]
pub struct CodeLocation {
    /// The row of the code where the diagnostic is located.
    pub line: usize,
    /// The column of the code where the diagnostic is located.
    pub column: usize,
}

#[derive(Debug, PartialEq, Eq, Clone)]
pub struct DiagnosticKind {
    /// The identifier of the diagnostic, used to align the diagnostic with a rule.
    pub name: String,
    /// The message body to display to the user, to explain the diagnostic.
    pub body: String,
}

#[derive(Debug, PartialEq, Eq, Clone)]
pub struct Diagnostic {
    pub kind: DiagnosticKind,
    pub location: CodeLocation,
}
