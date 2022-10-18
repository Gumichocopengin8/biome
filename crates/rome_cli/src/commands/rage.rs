use rome_console::fmt::{Display, Formatter};
use rome_console::{fmt, markup, ConsoleExt, HorizontalLine, Markup};
use rome_diagnostics::termcolor;
use rome_diagnostics::termcolor::{ColorChoice, WriteColor};
use rome_fs::FileSystem;
use rome_service::workspace::{client, RageEntry, RageParams};
use rome_service::{load_config, DynRef, Workspace};
use std::{env, io, ops::Deref};
use tokio::runtime::Runtime;

use crate::{service, CliSession, Termination, VERSION};

/// Handler for the `rage` command
pub(crate) fn rage(mut session: CliSession) -> Result<(), Termination> {
    let terminal_supports_colors = termcolor::BufferWriter::stdout(ColorChoice::Auto)
        .buffer()
        .supports_color();

    session.app.console.log(markup!("CLI:\n"
    {KeyValuePair("Version", markup!({VERSION}))}
    {KeyValuePair("Color support", markup!({DebugDisplay(terminal_supports_colors)}))}

    {Section("Platform")}
    {KeyValuePair("CPU Architecture", markup!({std::env::consts::ARCH}))}
    {KeyValuePair("OS", markup!({std::env::consts::OS}))}

    {Section("Environment")}
    {EnvVarOs("NO_COLOR")}
    {EnvVarOs("TERM")}

    {RageConfiguration(&session.app.fs)}
    {WorkspaceRage(session.app.workspace.deref())}
    ));

    if session.app.workspace.server_info().is_none() {
        session
            .app
            .console
            .log(markup!("Discovering running Rome servers..."));
        session.app.console.log(markup!({ RunningRomeServer }));
    }

    Ok(())
}

struct WorkspaceRage<'a>(&'a dyn Workspace);

impl Display for WorkspaceRage<'_> {
    fn fmt(&self, fmt: &mut Formatter) -> io::Result<()> {
        let workspace = self.0;

        let rage_result = workspace.rage(RageParams {});

        match rage_result {
            Ok(result) => {
                for entry in result.entries {
                    match entry {
                        RageEntry::Section(title) => {
                            Section(&title).fmt(fmt)?;
                        }
                        RageEntry::Pair { name, value } => {
                            KeyValuePair(&name, markup!({ value })).fmt(fmt)?;
                        }
                        RageEntry::Markup(markup) => markup.fmt(fmt)?,
                    }
                }

                Ok(())
            }
            Err(err) => {
                writeln!(fmt)?;
                (markup! {<Error>"\u{2716} Workspace rage failed:"</Error>}).fmt(fmt)?;

                writeln!(fmt, " {err}")
            }
        }
    }
}

/// Prints information about other running rome server instances.
struct RunningRomeServer;

impl Display for RunningRomeServer {
    fn fmt(&self, f: &mut Formatter) -> io::Result<()> {
        let runtime = Runtime::new()?;

        match service::open_transport(runtime) {
            Ok(None) => {
                markup!(
                    {Section("Server")}
                    {KeyValuePair("Status", markup!(<Dim>"stopped"</Dim>))}
                )
                .fmt(f)?;
            }
            Ok(Some(transport)) => {
                markup!("\n"<Emphasis>"Running Rome Server:"</Emphasis>" "{HorizontalLine::new(78)}"

"<Info>"\u{2139} The client isn't connected to any server but rage discovered this running Rome server."</Info>"
")
                .fmt(f)?;

                match client(transport) {
                    Ok(client) => WorkspaceRage(client.deref()).fmt(f)?,
                    Err(err) => {
                        markup!(<Error>"\u{2716} Failed to connect: "</Error>).fmt(f)?;
                        writeln!(f, "{err}")?;
                    }
                }
            }
            Err(err) => {
                markup!("\n"<Error>"\u{2716} Failed to connect: "</Error>).fmt(f)?;
                writeln!(f, "{err}")?;
            }
        };

        Ok(())
    }
}

struct RageConfiguration<'a, 'app>(&'a DynRef<'app, dyn FileSystem>);

impl Display for RageConfiguration<'_, '_> {
    fn fmt(&self, fmt: &mut Formatter) -> io::Result<()> {
        Section("Rome Configuration").fmt(fmt)?;

        match load_config(self.0, None) {
            Ok(None) => KeyValuePair("Status", markup!(<Dim>"unset"</Dim>)).fmt(fmt)?,
            Ok(Some(configuration)) => {
                markup! (
                    {KeyValuePair("Status", markup!(<Dim>"loaded"</Dim>))}
                    {KeyValuePair("Formatter disabled", markup!({DebugDisplay(configuration.is_formatter_disabled())}))}
                    {KeyValuePair("Linter disabled", markup!({DebugDisplay(configuration.is_linter_disabled())}))}
                ).fmt(fmt)?
            }
            Err(err) => {
                markup! (
                    {KeyValuePair("Status", markup!(<Error>"Failed to load"</Error>))}
                    {KeyValuePair("Error", markup!({format!("{err:?}")}))}
                ).fmt(fmt)?
            }
        }

        Ok(())
    }
}

struct DebugDisplay<T>(T);

impl<T> Display for DebugDisplay<T>
where
    T: std::fmt::Debug,
{
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> io::Result<()> {
        write!(f, "{:?}", self.0)
    }
}

struct EnvVarOs(&'static str);

impl fmt::Display for EnvVarOs {
    fn fmt(&self, fmt: &mut Formatter) -> io::Result<()> {
        let name = self.0;
        match env::var_os(name) {
            None => KeyValuePair(name, markup! { <Dim>"unset"</Dim> }).fmt(fmt),
            Some(value) => KeyValuePair(name, markup! {{DebugDisplay(value)}}).fmt(fmt),
        }
    }
}

struct Section<'a>(&'a str);

impl Display for Section<'_> {
    fn fmt(&self, fmt: &mut Formatter) -> io::Result<()> {
        writeln!(fmt, "\n{}:", self.0)
    }
}

struct KeyValuePair<'a>(&'a str, Markup<'a>);

impl Display for KeyValuePair<'_> {
    fn fmt(&self, fmt: &mut Formatter) -> io::Result<()> {
        let KeyValuePair(key, value) = self;
        write!(fmt, "  {key}:")?;

        let padding_width = 22usize.saturating_sub(key.len() + 1);

        for _ in 0..padding_width {
            fmt.write_str(" ")?;
        }

        value.fmt(fmt)?;

        fmt.write_str("\n")
    }
}