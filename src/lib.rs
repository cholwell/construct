use anyhow::Result;
use console::Term;

/// A `View` is an encapsulated piece of ui that can be written to the terminal.
///
/// To display a `View`, call `Construct.view(..)`.
///
/// ```
/// struct Foo;
///
/// impl View for Foo {
///     fn title(&self) -> String {
///         String::from("Title")
///     }
///
///     fn content(&self, terminal: &Term, _: &Construct) -> Result<()> {
///         terminal.write_line("hello world")
///     }
/// }
/// ```
pub trait View {
    fn title(&self) -> String;
    fn content(self, terminal: &Term, construct: &Construct) -> Result<()>;
}

/// `Construct` provides simple view based routing for the terminal.
///
/// Just create a new instance, and then pass a `View` to `Construct.view(..)`
///
/// ```
/// let construct = Construct::new();
/// construct.view(Foo)
/// ```
pub struct Construct {
    terminal: Term,
    logo: Option<String>,
}

impl Construct {
    /// Create a `Construct` with default values
    pub fn new() -> Self {
        Self {
            terminal: Term::stdout(),
            logo: None,
        }
    }

    /// Create a `ConstructBuilder`
    pub fn builder() -> ConstructBuilder {
        ConstructBuilder {
            terminal: None,
            logo: None,
        }
    }

    /// Display a `View` in the terminal
    ///
    /// ```
    /// let foo: View = Foo::new();
    /// construct.view(foo)
    /// ```
    pub fn view(&self, view: impl View) -> Result<()> {
        self.terminal.clear()?;
        if let Some(l) = &self.logo {
            self.terminal.write_line(&l)?;
        }
        self.terminal.write_line(&view.title())?;
        view.content(&self.terminal, self)
    }
}

/// Builder for `Construct`
///
/// ```
/// let construct = Construct::builder()
///     .with_terminal(Term::stdout())
///     .with_logo(String::from("Logo"))
///     .build();
/// ```
pub struct ConstructBuilder {
    terminal: Option<Term>,
    logo: Option<String>,
}

impl ConstructBuilder {
    /// Add specific terminal
    pub fn with_terminal(mut self, terminal: Term) -> ConstructBuilder {
        self.terminal = Some(terminal);
        self
    }

    /// Add a logo
    pub fn with_logo(mut self, logo: String) -> ConstructBuilder {
        self.logo = Some(logo);
        self
    }

    /// Build a new `Construct`
    pub fn build(self) -> Construct {
        let terminal = match self.terminal {
            Some(t) => t,
            None => Term::stdout(),
        };
        Construct {
            terminal,
            logo: self.logo,
        }
    }
}

pub trait WriteLineBreak {
    /// Write an empty line to the terminal
    ///
    /// ```
    /// terminal.write_line_break()?;
    /// ```
    fn write_line_break(&self) -> Result<()>;
}

impl WriteLineBreak for Term {
    fn write_line_break(&self) -> Result<()> {
        self.write_str("\n")?;
        Ok(())
    }
}

pub trait Clear {
    /// Clear the terminal
    ///
    /// ```
    /// terminal.clear()?;
    /// ```
    fn clear(&self) -> Result<()>;
}

impl Clear for Term {
    fn clear(&self) -> Result<()> {
        self.clear_last_lines(999)?;
        Ok(())
    }
}
