use miette::Diagnostic;

#[derive(thiserror::Error, Diagnostic, Debug)]
pub enum StructureErrors {
    #[error("File `{0}` is missing a state section.")]
    #[help("Try adding a state section:\nState {\n\t...\n}")]
    MissingState(String),
}

#[derive(thiserror::Error, Diagnostic, Debug)]
#[error("This error has not been properly documented yet.")]
pub struct UnimplementedError {}
