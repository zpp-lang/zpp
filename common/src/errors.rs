use codespan::{ByteIndex, Span};
use codespan_reporting::diagnostic::{Diagnostic, Label, LabelStyle, Severity};
use codespan_reporting::files::SimpleFile;
use codespan_reporting::term::{Config, emit};
use codespan_reporting::term::termcolor::{ColorChoice, StandardStream};

/// A helper for generating and emitting diagnostics for error reporting.
#[derive(Clone)]
pub struct Reporting<'a> {
    file: SimpleFile<&'a str, &'a str>
}

pub fn merge_span(span1: &Span, span2: &Span) -> Span {
    Span::new(
        ByteIndex::from(span1.start().0),
        ByteIndex::from(span2.end().0)
    )
}

pub fn create_span(log: logos::Span) -> Span {
    Span::new(
        ByteIndex::from(log.start as u32),
        ByteIndex::from(log.end as u32)
    )
}

impl<'a> Reporting<'a> {
    /// Constructs a new Reporting instance with the given file.
    ///
    /// # Arguments
    ///
    /// * `file` - The name of the file that is being reported
    /// * 'content' - The content of the file that is being reported
    ///
    /// # Returns
    ///
    /// Returns a `Reporting` instance.
    pub fn new(file: &'a str, content: &'a str) -> Self {
        Reporting { file: SimpleFile::new(file, content) }
    }

    /// Constructs a diagnostic with the specified severity, message, span, notes, and code.
    ///
    /// # Arguments
    ///
    /// * `severity` - The severity of the diagnostic (error, warning, info, note).
    /// * `message` - The message describing the diagnostic.
    /// * `span` - The span of the source code associated with the diagnostic.
    /// * `notes` - Additional notes to include with the diagnostic.
    /// * `code` - A code associated with the diagnostic.
    ///
    /// # Returns
    ///
    /// Returns a `Diagnostic<()>` instance.
    fn make_diagnostic(
        &self,
        severity: Severity,
        message: String,
        span: Span,
        notes: Vec<String>,
        code: String
    ) -> Diagnostic<()> {
        Diagnostic::new(severity)
            .with_code(code)
            .with_labels(vec![
                Label::new(
                    LabelStyle::Primary,
                    (),
                    span.start().0 as usize .. span.end().0 as usize
                )
            ])
            .with_notes(notes)
            .with_message(message)
    }

    /// Emits the given diagnostic.
    ///
    /// # Arguments
    ///
    /// * `diagnostic` - The diagnostic to be emitted.
    ///
    /// # Returns
    ///
    /// Returns None.
    fn emit_diagnostic(
        &self,
        diagnostic: &Diagnostic<()>
    ) {
        let writer = StandardStream::stdout(ColorChoice::Auto);
        let config = Config::default();
        emit(&mut writer.lock(), &config, &self.file, diagnostic)
            .expect("Failed to emit diagnostic");
    }

    /// Emits an error diagnostic with the specified message, span, notes, and code.
    ///
    /// # Arguments
    ///
    /// * `message` - The error message.
    /// * `span` - The span of the source code associated with the error.
    /// * `notes` - Additional notes to include with the error.
    /// * `code` - A code associated with the error.
    ///
    /// # Returns
    ///
    /// Returns a `Diagnostic<()>` instance representing the emitted error.
    pub fn emit_error(
        &self,
        message: String,
        span: Span,
        notes: Vec<String>,
        code: String,
    ) -> Diagnostic<()> {
        let diagnostic = self.make_diagnostic(
            Severity::Error,
            message,
            span,
            notes,
            code,
        );

        self.emit_diagnostic(&diagnostic);
        diagnostic
    }

    /// Emits a warning diagnostic with the specified message, span, notes, and code.
    ///
    /// # Arguments
    ///
    /// * `message` - The warning message.
    /// * `span` - The span of the source code associated with the warning.
    /// * `notes` - Additional notes to include with the warning.
    /// * `code` - A code associated with the warning.
    ///
    /// # Returns
    ///
    /// Returns a `Diagnostic<()>` instance representing the emitted warning.
    pub fn emit_warning(
        &self,
        message: String,
        span: Span,
        notes: Vec<String>,
        code: String,
    ) -> Diagnostic<()> {
        let diagnostic = self.make_diagnostic(
            Severity::Warning,
            message,
            span,
            notes,
            code,
        );

        self.emit_diagnostic(&diagnostic);
        diagnostic
    }

    /// Emits an info diagnostic with the specified message, span, notes, and code.
    ///
    /// # Arguments
    ///
    /// * `message` - The info message.
    /// * `span` - The span of the source code associated with the info message.
    /// * `notes` - Additional notes to include with the info message.
    /// * `code` - A code associated with the info message.
    ///
    /// # Returns
    ///
    /// Returns a `Diagnostic<()>` instance representing the emitted info message.
    pub fn emit_info(
        &self,
        message: String,
        span: Span,
        notes: Vec<String>,
        code: String,
    ) -> Diagnostic<()> {
        let diagnostic = self.make_diagnostic(
            Severity::Help,
            message,
            span,
            notes,
            code,
        );

        self.emit_diagnostic(&diagnostic);
        diagnostic
    }

    /// Emits a note diagnostic with the specified message, span, notes, and code.
    ///
    /// # Arguments
    ///
    /// * `message` - The note message.
    /// * `span` - The span of the source code associated with the note.
    /// * `notes` - Additional notes to include with the note.
    /// * `code` - A code associated with the note.
    ///
    /// # Returns
    ///
    /// Returns a `Diagnostic<()>` instance representing the emitted note.
    pub fn emit_note(
        &self,
        message: String,
        span: Span,
        notes: Vec<String>,
        code: String,
    ) -> Diagnostic<()> {
        let diagnostic = self.make_diagnostic(
            Severity::Note,
            message,
            span,
            notes,
            code,
        );

        self.emit_diagnostic(&diagnostic);
        diagnostic
    }
}