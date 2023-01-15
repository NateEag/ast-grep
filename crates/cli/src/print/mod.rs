mod colored_print;
mod interactive_print;
mod json_print;

use ast_grep_config::RuleConfig;
use ast_grep_core::{Matcher, NodeMatch, Pattern};
use ast_grep_language::SupportLang;

use anyhow::Result;
pub use codespan_reporting::{files::SimpleFile, term::ColorArg};

use std::borrow::Cow;
use std::path::Path;

pub use codespan_reporting::term::termcolor::ColorChoice;
pub use colored_print::{print_diff, ColoredPrinter, Heading, PrintStyles, ReportStyle};
pub use interactive_print::InteractivePrinter;
pub use json_print::JSONPrinter;

// add this macro because neither trait_alias nor type_alias_impl is supported.
macro_rules! Matches {
  ($lt: lifetime) => { impl Iterator<Item = NodeMatch<$lt, SupportLang>> };
}
macro_rules! Diffs {
  ($lt: lifetime) => { impl Iterator<Item = Diff<$lt>> };
}

pub trait Printer {
  fn print_rule<'a>(
    &self,
    matches: Matches!('a),
    file: SimpleFile<Cow<str>, &String>,
    rule: &RuleConfig<SupportLang>,
  ) -> Result<()>;
  fn print_matches<'a>(&self, matches: Matches!('a), path: &Path) -> Result<()>;
  fn print_diffs<'a>(&self, diffs: Diffs!('a), path: &Path) -> Result<()>;
  fn print_rule_diffs<'a>(
    &self,
    diffs: Diffs!('a),
    path: &Path,
    rule: &RuleConfig<SupportLang>,
  ) -> Result<()>;
  #[inline]
  fn before_print(&self) -> Result<()> {
    Ok(())
  }
  #[inline]
  fn after_print(&self) -> Result<()> {
    Ok(())
  }
}

#[derive(Clone)]
pub struct Diff<'n> {
  /// the matched node
  pub node_match: NodeMatch<'n, SupportLang>,
  /// string content for the replacement
  pub replacement: Cow<'n, str>,
}

impl<'n> Diff<'n> {
  pub fn generate(
    node_match: NodeMatch<'n, SupportLang>,
    matcher: &impl Matcher<SupportLang>,
    rewrite: &Pattern<SupportLang>,
  ) -> Self {
    let replacement = Cow::Owned(
      node_match
        .replace(matcher, rewrite)
        .expect("edit must exist")
        .inserted_text,
    );

    Self {
      node_match,
      replacement,
    }
  }
}
