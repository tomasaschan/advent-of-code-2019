use super::Debugger;
use rustyline::completion::{Completer, FilenameCompleter, Pair};
use rustyline::error::ReadlineError;
use rustyline::highlight::Highlighter;
use rustyline::hint::Hinter;
use rustyline::{Config, Context, Editor};
use rustyline_derive::Helper;

impl Debugger {
    pub fn readline(&mut self, prompt: &str, add_to_history: bool) -> Option<String> {
        match self.editor.readline(prompt) {
            Ok(cmd) => {
                if add_to_history {
                    self.editor.add_history_entry(&cmd);
                    if self.editor.save_history(".intcode_hist").is_err() {}
                }

                Some(cmd)
            }
            // Err(ReadlineError::Interrupted) => {
            //     println!("^C");
            //     break;
            // }
            Err(ReadlineError::Eof) => {
                println!("Goodbye!");
                None
            }
            Err(err) => {
                println!("Error: {:?}", err);
                None
            }
        }
    }
}

#[derive(Helper)]
pub struct Help {
    completer: FilenameCompleter,
}
impl Help {
    pub fn new() -> Help {
        Help {
            completer: FilenameCompleter::new(),
        }
    }
}
impl Highlighter for Help {}
impl Hinter for Help {}
impl Completer for Help {
    type Candidate = Pair;
    fn complete(
        &self,
        line: &str,
        pos: usize,
        ctx: &Context<'_>,
    ) -> Result<(usize, Vec<Pair>), ReadlineError> {
        self.completer.complete(line, pos, ctx)
    }
}

pub fn create_editor() -> Editor<Help> {
    let config = Config::builder()
        .history_ignore_space(true)
        .history_ignore_dups(true)
        .auto_add_history(true)
        .build();
    let mut editor = Editor::with_config(config);
    editor.set_helper(Some(Help::new()));
    editor
}
