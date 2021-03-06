//! Types used in resolution of Pseudonymes: Aliass, Exports and (TODO) Typepseudonyms

use std::{
  fmt::{ Display, Debug, Formatter, Result as FMTResult, },
};

use mod_common::{ Identifier, };

use crate::{
  source::{ SourceRegion, },
  ast::{ Path, TypeExpression, },
  ctx::{ ContextKey, },
};

/// Variant data for an Pseudonym
#[repr(u8)]
#[allow(missing_docs)]
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub enum PseudonymKind {
  Alias,
  Export,
}

impl Display for PseudonymKind {
  fn fmt (&self, f: &mut Formatter) -> FMTResult {
    write!(f, "{}", match self { PseudonymKind::Alias => "Alias", PseudonymKind::Export => "Export" })
  }
}

/// Variant data for an Pseudonym
#[repr(u8)]
#[allow(missing_docs)]
#[derive(Debug, Clone)]
pub enum PseudonymPayload {
  Path(Path),
  TypeExpression(TypeExpression),
}
  
/// A placeholder structure for delayed evaluation of imports and exports
#[allow(missing_docs)]
#[derive(Debug, Clone)]
pub struct Pseudonym {
  pub destination_namespace: ContextKey,
  pub kind: PseudonymKind,
  pub payload: PseudonymPayload,
  pub new_name: Identifier,
  pub relative_to: ContextKey,
  pub origin: SourceRegion,
}

/// Defines an expectation of some aspect of an analysis action
#[repr(u8)]
#[allow(missing_docs)]
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub enum Expect {
  Require,
  Allow,
  Deny,
}

impl Default for Expect { #[inline] fn default () -> Self { Self::Allow } }

/// The result type given by `ty_helpers::ty_meet_n`
pub enum TyMeetResult {
  /// There was a single, viable type which all types could coerce to
  Ok(ContextKey),
  /// There were multiple viable types which all types could coerce to,
  /// which is unresolvable in the current type system
  Unresolvable,
  /// There was no viable type which all types could coerce to
  None,
}
