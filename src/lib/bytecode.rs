//! The bytecode form and component structures

use std::{
  mem::{ transmute, },
};



/// Interface trait for encoding a bytecode value into a byte buffer
pub trait Encode {
  /// Encodes a Bytecode value into a byte buffer
  fn encode (&self, buff: &mut Vec<u8>);
}

/// Interface trait for decoding a bytecode value from a byte buffer
pub trait Decode: Sized {
  /// Decodes a bytecode value from a byte buffer
  fn decode (buff: &mut &[u8]) -> Result<Self, DecodeError>;
}

/// An error resulting from attempting to decode a bytecode value from an improperly formed byte buffer
#[repr(u8)]
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub enum DecodeError {
  /// The decoder hit the end of the byte buffer without getting enough data to complete a value
  EOF,
  /// The decoder read a badly encoded String
  InvalidString,
  /// The decoder encountered an unexpected value
  UnexpectedValue,
}



/// The main container of the in-memory representation of bytecode for a compilation
#[derive(Debug, Clone, PartialEq)]
pub struct Module {
  /// The uniquely identifying name of a Module
  pub name: String,
  /// The semantic Version number of a Module
  pub version: Version,

  /// All Type definitions used by a Module
  pub types:     Vec<Type>,
  /// All Modules a Module depends on
  pub imports:   Vec<ImportModule>,
  /// All Global definitions of a Module, in initialization order
  pub globals:   Vec<Global>,
  /// All Function defintions of a Module
  pub functions: Vec<Function>,
  /// All Exports a Module exposes
  pub exports:   Vec<Export>,
}

impl Module {
  /// Create a new, empty `Module`
  pub fn empty (name: String, version: Version) -> Self {
    Self {
      name,
      version,

      types: Vec::default(),
      imports: Vec::default(),
      globals: Vec::default(),
      functions: Vec::default(),
      exports: Vec::default(),
    }
  }
}

impl Encode for Module {
  fn encode (&self, buff: &mut Vec<u8>) {
    self.name.encode(buff);
    self.version.encode(buff);
    self.types.encode(buff);
    self.imports.encode(buff);
    self.globals.encode(buff);
    self.functions.encode(buff);
    self.exports.encode(buff);
  }
}

impl Decode for Module {
  fn decode (buff: &mut &[u8]) -> Result<Module, DecodeError> {
    Ok(Module {
      name: String::decode(buff)?,
      version: Version::decode(buff)?,
      types: Vec::decode(buff)?,
      imports: Vec::decode(buff)?,
      globals: Vec::decode(buff)?,
      functions: Vec::decode(buff)?,
      exports: Vec::decode(buff)?,
    })
  }
}



/// Represents a `Module`'s semver2 version number
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Default)]
pub struct Version {
  /// Bumped for incompatible API changes
  pub major: u8,
  /// Bumped for added functionality in a backwards compatible manner
  pub minor: u8,
  /// Bumped for backwards compatible bug fixes
  pub patch: u8,
}

impl Version {
  /// Create a new `Version` and initialize its values
  pub fn new (major: u8, minor: u8, patch: u8) -> Self {
    Self { major, minor, patch }
  }
}

impl Encode for Version {
  fn encode (&self, buff: &mut Vec<u8>) {
    self.major.encode(buff);
    self.minor.encode(buff);
    self.patch.encode(buff);
  }
}

impl Decode for Version {
  fn decode (buff: &mut &[u8]) -> Result<Version, DecodeError> {
    Ok(Version {
      major: u8::decode(buff)?,
      minor: u8::decode(buff)?,
      patch: u8::decode(buff)?,
    })
  }
}



/// A unique (per-item-kind, per-`Module`) id for an item in a `Module`
/// 
/// This is a generic version of the type safe indices defined later
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Default)]
pub struct ID(pub u64);

impl From<u64> for ID { fn from (i: u64) -> Self { Self(i) } }
impl From<ID> for u64 { fn from (i: ID) -> Self { i.0 } }

impl From<TypeID> for ID { fn from (i: TypeID) -> Self { Self(i.0) } }
impl From<GlobalID> for ID { fn from (i: GlobalID) -> Self { Self(i.0) } }
impl From<FunctionID> for ID { fn from (i: FunctionID) -> Self { Self(i.0) } }
impl From<LocalID> for ID { fn from (i: LocalID) -> Self { Self(i.0) } }
impl From<ElementID> for ID { fn from (i: ElementID) -> Self { Self(i.0) } }

impl Encode for ID { fn encode (&self, buff: &mut Vec<u8>) { self.0.encode(buff) } }

impl Decode for ID { fn decode (buff: &mut &[u8]) -> Result<ID, DecodeError> { Ok(Self(u64::decode(buff)?)) }}



/// A unique (per-`Module`) id for a `Type` in a `Module`
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Default)]
pub struct TypeID(pub u64);

/// A unique (per-`Module`) id for a `Global` in a `Module`
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Default)]
pub struct GlobalID(pub u64);

/// A unique (per-`Module`) id for a `Function` in a `Module`
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Default)]
pub struct FunctionID(pub u64);

/// A unique (per-`Function`) id for a local variable in a `Function`
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Default)]
pub struct LocalID(pub u64);

/// A unique (per-struct) id for an element in a `Type`
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Default)]
pub struct ElementID(pub u64);

impl From<u64> for TypeID { fn from (i: u64) -> Self { Self(i) } }
impl From<u64> for GlobalID { fn from (i: u64) -> Self { Self(i) } }
impl From<u64> for FunctionID { fn from (i: u64) -> Self { Self(i) } }
impl From<u64> for LocalID { fn from (i: u64) -> Self { Self(i) } }
impl From<u64> for ElementID { fn from (i: u64) -> Self { Self(i) } }

impl From<TypeID> for u64 { fn from (i: TypeID) -> Self { i.0 } }
impl From<GlobalID> for u64 { fn from (i: GlobalID) -> Self { i.0 } }
impl From<FunctionID> for u64 { fn from (i: FunctionID) -> Self { i.0 } }
impl From<LocalID> for u64 { fn from (i: LocalID) -> Self { i.0 } }
impl From<ElementID> for u64 { fn from (i: ElementID) -> Self { i.0 } }

impl From<ID> for TypeID { fn from (i: ID) -> Self { Self(i.0) } }
impl From<ID> for GlobalID { fn from (i: ID) -> Self { Self(i.0) } }
impl From<ID> for FunctionID { fn from (i: ID) -> Self { Self(i.0) } }
impl From<ID> for LocalID { fn from (i: ID) -> Self { Self(i.0) } }
impl From<ID> for ElementID { fn from (i: ID) -> Self { Self(i.0) } }

impl Encode for TypeID { fn encode (&self, buff: &mut Vec<u8>) { self.0.encode(buff) } }
impl Encode for GlobalID { fn encode (&self, buff: &mut Vec<u8>) { self.0.encode(buff) } }
impl Encode for FunctionID { fn encode (&self, buff: &mut Vec<u8>) { self.0.encode(buff) } }
impl Encode for LocalID { fn encode (&self, buff: &mut Vec<u8>) { self.0.encode(buff) } }
impl Encode for ElementID { fn encode (&self, buff: &mut Vec<u8>) { self.0.encode(buff) } }

impl Decode for TypeID { fn decode (buff: &mut &[u8]) -> Result<TypeID, DecodeError> { Ok(Self(u64::decode(buff)?)) }}
impl Decode for GlobalID { fn decode (buff: &mut &[u8]) -> Result<GlobalID, DecodeError> { Ok(Self(u64::decode(buff)?)) }}
impl Decode for FunctionID { fn decode (buff: &mut &[u8]) -> Result<FunctionID, DecodeError> { Ok(Self(u64::decode(buff)?)) }}
impl Decode for LocalID { fn decode (buff: &mut &[u8]) -> Result<LocalID, DecodeError> { Ok(Self(u64::decode(buff)?)) }}
impl Decode for ElementID { fn decode (buff: &mut &[u8]) -> Result<ElementID, DecodeError> { Ok(Self(u64::decode(buff)?)) }}



/// Represents a type definition in a `Module`
#[derive(Debug, Clone, PartialEq)]
pub struct Type {
  /// The id of a `Type` in a `Module`'s types list
  pub id: TypeID,
  /// The variant data associated with a `Type`
  pub data: TypeData,
}

impl Type {
  /// Create a new `Type`
  pub fn new (id: TypeID, data: TypeData) -> Self {
    Self { id, data }
  }
}

impl Encode for Type {
  fn encode (&self, buff: &mut Vec<u8>) {
    self.id.encode(buff);
    self.data.encode(buff);
  }
}

impl Decode for Type {
  fn decode (buff: &mut &[u8]) -> Result<Type, DecodeError> {
    Ok(Type {
      id: TypeID::decode(buff)?,
      data: TypeData::decode(buff)?,
    })
  }
}



/// Variant data associated with a `Type`
#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub enum TypeData {
  /// A built in type defined by the compiler
  Intrinsic(IntrinsicType),
  /// The address of a value of another type
  Pointer(TypeID),
  /// An aggregate containing a list of values of other types
  Struct(Vec<TypeID>),
  /// A functional interface signature
  Function {
    /// Types of values provided to a function
    parameters: Vec<TypeID>,
    /// Type of value returned by a function
    result: Option<TypeID>
  },
}

impl TypeData {
  /// Get the `TypeDataKind` of a `TypeData`
  pub fn get_kind (&self) -> TypeDataKind {
    match self {
      TypeData::Intrinsic { .. } => TypeDataKind::Intrinsic,
      TypeData::Pointer   { .. } => TypeDataKind::Pointer,
      TypeData::Struct    { .. } => TypeDataKind::Struct,
      TypeData::Function  { .. } => TypeDataKind::Function,
    }
  }
}

impl Default for TypeData { fn default () -> Self { Self::Intrinsic(IntrinsicType::default()) } }

impl Encode for TypeData {
  fn encode (&self, buff: &mut Vec<u8>) {
    self.get_kind().encode(buff);

    use TypeData::*;

    match self {
      Intrinsic(ity) => ity.encode(buff),
      Pointer(idx) => idx.encode(buff),
      Struct(fields) => fields.encode(buff),
      Function { parameters, result } => {
        parameters.encode(buff);
        result.encode(buff);
      },
    }
  }
}

impl Decode for TypeData {
  fn decode (buff: &mut &[u8]) -> Result<TypeData, DecodeError> {
    Ok(match TypeDataKind::decode(buff)? {
      TypeDataKind::Intrinsic => TypeData::Intrinsic(IntrinsicType::decode(buff)?),
      TypeDataKind::Pointer => TypeData::Pointer(TypeID::decode(buff)?),
      TypeDataKind::Struct => TypeData::Struct(Vec::decode(buff)?),
      TypeDataKind::Function => TypeData::Function {
        parameters: Vec::decode(buff)?,
        result: Option::decode(buff)?,
      },
    })
  }
}



/// A data-less variant only version of `TypeData`
/// 
/// See `TypeData` for docs on each variant
#[repr(u8)]
#[allow(missing_docs)]
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub enum TypeDataKind {
  Intrinsic,
  Pointer,
  Struct,
  Function,
}

impl Encode for TypeDataKind {
  fn encode (&self, buff: &mut Vec<u8>) {
    buff.push(*self as _)
  }
}

impl Decode for TypeDataKind {
  fn decode (buff: &mut &[u8]) -> Result<TypeDataKind, DecodeError> {
    let byte = u8::decode(buff)?;
    
    if byte >= TypeDataKind::Intrinsic as _
    && byte <= TypeDataKind::Function  as _ {
      Ok(unsafe { transmute(byte) })
    } else {
      Err(DecodeError::UnexpectedValue)
    }
  }
}



/// Represents a type defined by the compiler
#[repr(u8)]
#[derive(Debug, Clone, Copy, PartialEq, PartialOrd, Ord, Eq, Hash)]
pub enum IntrinsicType {
  /// An empty type, with no associated values
  Void,
  /// A binary state
  Bool,
  /// 8 bit unsigned integer
  U8,
  /// 16 bit unsigned integer
  U16,
  /// 32 bit unsigned integer
  U32,
  /// 64 bit unsigned integer
  U64,
  /// 8 bit signed integer
  S8,
  /// 16 bit signed integer
  S16,
  /// 32 bit signed integer
  S32,
  /// 64 bit signed integer
  S64,
  /// 32 bit real
  F32,
  /// 64 bit real
  F64,
}

impl Default for IntrinsicType { fn default () -> Self { Self::Void } }

impl Encode for IntrinsicType {
  fn encode (&self, buff: &mut Vec<u8>) {
    buff.push(*self as _)
  }
}

impl Decode for IntrinsicType {
  fn decode (buff: &mut &[u8]) -> Result<IntrinsicType, DecodeError> {
    let byte = u8::decode(buff)?;
    
    if byte >= IntrinsicType::Void as _
    && byte <= IntrinsicType::F64  as _ {
      Ok(unsafe { transmute(byte) })
    } else {
      Err(DecodeError::UnexpectedValue)
    }
  }
}



/// Binds another module imported by a `Module`
#[derive(Debug, Clone, PartialEq)]
pub struct ImportModule {
  /// The uniquely identifying name of an `ImportModule`
  pub name: String,
  /// The semantic `Version` number of an `ImportModule`
  pub version: Version,
  /// List of items bound from the `ImportModule`
  pub items: Vec<Import>,
}

impl ImportModule {
  /// Create a new, empty `ImportModule`
  pub fn empty (name: String, version: Version) -> Self {
    Self {
      name,
      version,
      items: Vec::default(),
    }
  }
}

impl Encode for ImportModule {
  fn encode (&self, buff: &mut Vec<u8>) {
    self.name.encode(buff);
    self.version.encode(buff);
    self.items.encode(buff);
  }
}

impl Decode for ImportModule {
  fn decode (buff: &mut &[u8]) -> Result<ImportModule, DecodeError> {
    Ok(ImportModule {
      name: String::decode(buff)?,
      version: Version::decode(buff)?,
      items: Vec::decode(buff)?
    })
  }
}



/// Binds an item from an imported module to a unique id in a `Module`
#[derive(Debug, Clone, PartialEq)]
pub struct Import {
  /// The uniquely identifying name of an `Import` binding
  pub name: String,
  /// The variant data associated with an `Import` binding
  pub data: ImportData,
}

impl Import {
  /// Create a new `Import`
  pub fn new (name: String, data: ImportData) -> Self {
    Self { name, data }
  }
}

impl Encode for Import {
  fn encode (&self, buff: &mut Vec<u8>) {
    self.name.encode(buff);
    self.data.encode(buff);
  }
}

impl Decode for Import {
  fn decode (buff: &mut &[u8]) -> Result<Import, DecodeError> {
    Ok(Import {
      name: String::decode(buff)?,
      data: ImportData::decode(buff)?,
    })
  }
}



/// Variant data associated with an `Import`
#[derive(Debug, Clone, PartialEq)]
pub enum ImportData {
  /// An imported namespace; contains other `Import` bindings
  Namespace(Vec<Import>),
  /// An imported `Global`; contains a `GlobalID` and a `TypeID`
  Global(GlobalID, TypeID),
  /// An imported `Function`; contains a `FunctionID` and a `TypeID`
  Function(FunctionID, TypeID),
}

impl ImportData {
  /// Get the `AliasDataKind` of an `ImportData`
  pub fn get_kind (&self) -> AliasDataKind {
    match self {
      ImportData::Namespace { .. } => AliasDataKind::Namespace,
      ImportData::Global { .. } => AliasDataKind::Global,
      ImportData::Function { .. } => AliasDataKind::Function,
    }
  }
}

impl Encode for ImportData {
  fn encode (&self, buff: &mut Vec<u8>) {
    self.get_kind().encode(buff);

    use ImportData::*;

    match self {
      Namespace(items) => {
        items.encode(buff);
      },

      Global(g_idx, t_idx) => {
        g_idx.encode(buff);
        t_idx.encode(buff);
      },

      Function(f_idx, t_idx) => {
        f_idx.encode(buff);
        t_idx.encode(buff);
      },
    }
  }
}

impl Decode for ImportData {
  fn decode (buff: &mut &[u8]) -> Result<ImportData, DecodeError> {
    Ok(match AliasDataKind::decode(buff)? {
      AliasDataKind::Namespace => ImportData::Namespace(Vec::decode(buff)?),
      AliasDataKind::Global    => ImportData::Global(GlobalID::decode(buff)?, TypeID::decode(buff)?),
      AliasDataKind::Function  => ImportData::Function(FunctionID::decode(buff)?, TypeID::decode(buff)?),
    })
  }
}



/// Represents a global variable definition in a `Module`
#[derive(Debug, Clone, PartialEq)]
pub struct Global {
  /// The id of a `Global` in a `Module`'s globals list
  pub id: GlobalID,
  /// The id of a `Global`'s `Type`
  pub ty: TypeID,
  /// The instructions used to initialize a `Global`
  pub initializer: Vec<Instruction>,
}

impl Global {
  /// Create a new, empty `Global` with no initializer
  pub fn empty (id: GlobalID, ty: TypeID) -> Self {
    Self {
      id,
      ty,
      initializer: Vec::default(),
    }
  }
}

impl Encode for Global {
  fn encode (&self, buff: &mut Vec<u8>) {
    self.id.encode(buff);
    self.ty.encode(buff);
    self.initializer.encode(buff);
  }
}

impl Decode for Global {
  fn decode (buff: &mut &[u8]) -> Result<Global, DecodeError> {
    Ok(Global {
      id: GlobalID::decode(buff)?,
      ty: TypeID::decode(buff)?,
      initializer: Vec::decode(buff)?,
    })
  }
}



/// Represents a function definition in a `Module`
#[derive(Debug, Clone, PartialEq)]
pub struct Function {
  /// The id of a `Function` in a `Module`'s functions list
  pub id: FunctionID,
  /// The id of a `Function`'s `Type`
  pub ty: TypeID,
  /// The instructions used to execute a `Function`
  pub body: Vec<Instruction>,
}

impl Function {
  /// Create a new, empty `Function` with no body
  pub fn empty (id: FunctionID, ty: TypeID) -> Self {
    Self {
      id,
      ty,
      body: Vec::default(),
    }
  }
}

impl Encode for Function {
  fn encode (&self, buff: &mut Vec<u8>) {
    self.id.encode(buff);
    self.ty.encode(buff);
    self.body.encode(buff);
  }
}

impl Decode for Function {
  fn decode (buff: &mut &[u8]) -> Result<Function, DecodeError> {
    Ok(Function {
      id: FunctionID::decode(buff)?,
      ty: TypeID::decode(buff)?,
      body: Vec::decode(buff)?,
    })
  }
}



/// Binds an item exported by a `Module`
#[derive(Debug, Clone, PartialEq)]
pub struct Export {
  /// The uniquely identifying name of an `Export` binding
  pub name: String,
  /// The variant data associated with an `Export` binding
  pub data: ExportData,
}

impl Export {
  /// Create a new `Export`
  pub fn new (name: String, data: ExportData) -> Self {
    Self { name, data }
  }
}

impl Encode for Export {
  fn encode (&self, buff: &mut Vec<u8>) {
    self.name.encode(buff);
    self.data.encode(buff);
  }
}

impl Decode for Export {
  fn decode (buff: &mut &[u8]) -> Result<Export, DecodeError> {
    Ok(Export {
      name: String::decode(buff)?,
      data: ExportData::decode(buff)?,
    })
  }
}



/// Variant data associated with an `Export`
#[derive(Debug, Clone, PartialEq)]
pub enum ExportData {
  /// An exported namespace; contains other `Export` bindings
  Namespace(Vec<Export>),
  /// An exported `Global`; contains a `GlobalID`
  Global(GlobalID),
  /// An exported `Function`; contains a `FunctionID`
  Function(FunctionID),
}

impl ExportData {
  /// Get the `AliasDataKind` of an `ExportData`
  pub fn get_kind (&self) -> AliasDataKind {
    match self {
      ExportData::Namespace { .. } => AliasDataKind::Namespace,
      ExportData::Global { .. } => AliasDataKind::Global,
      ExportData::Function { .. } => AliasDataKind::Function,
    }
  }
}

impl Encode for ExportData {
  fn encode (&self, buff: &mut Vec<u8>) {
    self.get_kind().encode(buff);
    
    use ExportData::*;

    match self {
      Namespace(items) => {
        items.encode(buff);
      },

      Global(g_idx) => g_idx.encode(buff),
      Function(f_idx) => f_idx.encode(buff),
    }
  }
}

impl Decode for ExportData {
  fn decode (buff: &mut &[u8]) -> Result<ExportData, DecodeError> {
    Ok(match AliasDataKind::decode(buff)? {
      AliasDataKind::Namespace => ExportData::Namespace(Vec::decode(buff)?),
      AliasDataKind::Global    => ExportData::Global(GlobalID::decode(buff)?),
      AliasDataKind::Function  => ExportData::Function(FunctionID::decode(buff)?),
    })
  }
}



/// A data-less variant only version of `ImportData`/`ExportData`
/// 
/// See `ImportData` and `ExportData` for docs on each variant
#[repr(u8)]
#[allow(missing_docs)]
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub enum AliasDataKind {
  Namespace,
  Global,
  Function,
}

impl Encode for AliasDataKind {
  fn encode (&self, buff: &mut Vec<u8>) {
    buff.push(*self as _);
  }
}

impl Decode for AliasDataKind {
  fn decode (buff: &mut &[u8]) -> Result<AliasDataKind, DecodeError> {
    let byte = u8::decode(buff)?;
    
    if byte >= AliasDataKind::Namespace as _
    && byte <= AliasDataKind::Function  as _ {
      Ok(unsafe { transmute(byte) })
    } else {
      Err(DecodeError::UnexpectedValue)
    }
  }
}



/// Bytecode instructions used by `Global`'s initializers and `Function`'s bodies
#[derive(Debug, Clone, PartialEq, PartialOrd)]
pub enum Instruction {
  /// Does nothing, filler data
  NoOp,

  /// Pushes a constant value on the stack
  ImmediateValue(ImmediateValue),

  /// Creates a local variable in the current stack frame, of the type given by an id
  CreateLocal(TypeID),

  /// Gets the address of a local variable,
  /// then pushes it on the stack
  LocalAddress(LocalID),
  /// Gets the address of a `Global` variable,
  /// then pushes it on the stack
  GlobalAddress(GlobalID),
  /// Gets the address of a `Function`,
  /// then pushes it on the stack
  FunctionAddress(FunctionID),
  
  /// Pops an address off the stack and offsets it to the specified struct element id,
  /// then pushes it back on the stack with the type of the struct element
  GetElement(ElementID),

  /// Pops a value off the stack and casts it to the type given by an id,
  /// then pushes the newly typed value back on the stack
  Cast(TypeID),
  
  /// Pops a value off the stack and uses it as an address for a dereference,
  /// then pushes the loaded data back on the stack
  Load,
  /// Pops an address and a value off the stack `(Addr, Val)`,
  /// and stores the value to the address
  Store,

  /// Pops a value off the stack and discards it
  Discard,

  /// Pops two values off the stack `(A, B)`, performs addition `(A + B)`,
  /// and pushes the result back on the stack
  Add,
  /// Pops two values off the stack `(A, B)`, performs subtraction `(A - B)`,
  /// and pushes the result back on the stack
  Sub,
  /// Pops two values off the stack `(A, B)`, performs multiplication `(A * B)`,
  /// and pushes the result back on the stack
  Mul,
  /// Pops two values off the stack `(A, B)`, performs division `(A / B)`,
  /// and pushes the result back on the stack
  Div,
  /// Pops two values off the stack `(A, B)`, performs remainder division `(A % B)`,
  /// and pushes the result back on the stack
  Rem,
  /// Pops a single value off the stack and negates its sign (-Val),
  /// and pushes the result back on the stack
  Neg,

  /// Pops two values off the stack `(A, B)`, performs bitwise AND `(A & B)`,
  /// and pushes the result back on the stack
  And,
  /// Pops two values off the stack `(A, B)`, performs bitwise OR `(A | B)`,
  /// and pushes the result back on the stack
  Or,
  /// Pops two values off the stack `(A, B)`, performs bitwise XOR `(A ~ B)`,
  /// and pushes the result back on the stack
  Xor,
  /// Pops two values off the stack `(A, B)`, performs bitwise LSHIFT `(A << B)`,
  /// and pushes the result back on the stack
  LShift,
  /// Pops two values off the stack `(A, B)`, performs bitwise RSHIFT `(A >> B)`,
  /// and pushes the result back on the stack
  RShift,
  /// Pops a single value off the stack, performs bitwise NOT,
  /// and pushes the result back on the stack
  Not,

  /// Pops two values off the stack `(A, B)`, performs equality comparison `(A == B)`,
  /// and pushes the result back on the stack
  EQ,
  /// Pops two values off the stack `(A, B)`, performs inequality comparison `(A != B)`,
  /// and pushes the result back on the stack
  NEQ,
  /// Pops two values off the stack `(A, B)`, performs less than comparison `(A < B)`,
  /// and pushes the result back on the stack
  LT,
  /// Pops two values off the stack `(A, B)`, performs greater than comparison `(A > B)`,
  /// and pushes the result back on the stack
  GT,
  /// Pops two values off the stack `(A, B)`, performs less than or equal comparison `(A <= B)`,
  /// and pushes the result back on the stack
  LEQ,
  /// Pops two values off the stack `(A, B)`, performs greater than or equal comparison `(A >= B)`,
  /// and pushes the result back on the stack
  GEQ,

  /// Calls a `Module`-local `Function` by id,
  /// after popping a type-dependant amount of arguments off the stack.
  /// After the call is complete, pushes the function's return value back on the stack (if one was given)
  CallDirect(FunctionID),

  /// Pops a functional address off the stack,
  /// then pops a type-dependant amount of arguments off the stack.
  /// After the call is complete, pushes the function's return value back on the stack (if one was given)
  CallIndirect,

  /// Pops a boolean value off the stack and uses it as a branch predicate,
  /// either executing its then branch or else branch depending on the boolean value
  /// 
  /// Conceptually contains other instructions,
  /// but in serialized form it is a sequence
  IfBlock(Vec<Instruction>, Vec<Instruction>),

  /// Continues repeatedly executing its branch until a `Break` `Return`, or `ReturnVoid` instruction is reached
  /// 
  /// `Continue` instruction jumps to the top of this block
  /// 
  /// Conceptually contains other instructions,
  /// but in serialized form it is a sequence
  LoopBlock(Vec<Instruction>),

  /// Jumps to the end of a LoopBlock and ends the loop
  Break,
  /// Jumps back to the entry point of a LoopBlock and continues the loop
  Continue,

  /// Pops a value off the stack, and returns it from a function
  Return,
  /// Returns from a function
  ReturnVoid,
}

impl Instruction {
  /// Get the InstructionKind of an Instruction
  pub fn get_kind (&self) -> InstructionKind {
    match self {
      Instruction::NoOp { .. } => InstructionKind::NoOp,
      Instruction::ImmediateValue { .. } => InstructionKind::ImmediateValue,
      Instruction::CreateLocal { .. } => InstructionKind::CreateLocal,
      Instruction::LocalAddress { .. } => InstructionKind::LocalAddress,
      Instruction::GlobalAddress { .. } => InstructionKind::GlobalAddress,
      Instruction::FunctionAddress { .. } => InstructionKind::FunctionAddress,
      Instruction::GetElement { .. } => InstructionKind::GetElement,
      Instruction::Cast { .. } => InstructionKind::Cast,
      Instruction::Load { .. } => InstructionKind::Load,
      Instruction::Store { .. } => InstructionKind::Store,
      Instruction::Discard { .. } => InstructionKind::Discard,
      Instruction::Add { .. } => InstructionKind::Add,
      Instruction::Sub { .. } => InstructionKind::Sub,
      Instruction::Mul { .. } => InstructionKind::Mul,
      Instruction::Div { .. } => InstructionKind::Div,
      Instruction::Rem { .. } => InstructionKind::Rem,
      Instruction::Neg { .. } => InstructionKind::Neg,
      Instruction::And { .. } => InstructionKind::And,
      Instruction::Or { .. } => InstructionKind::Or,
      Instruction::Xor { .. } => InstructionKind::Xor,
      Instruction::LShift { .. } => InstructionKind::LShift,
      Instruction::RShift { .. } => InstructionKind::RShift,
      Instruction::Not { .. } => InstructionKind::Not,
      Instruction::EQ { .. } => InstructionKind::EQ,
      Instruction::NEQ { .. } => InstructionKind::NEQ,
      Instruction::LT { .. } => InstructionKind::LT,
      Instruction::GT { .. } => InstructionKind::GT,
      Instruction::LEQ { .. } => InstructionKind::LEQ,
      Instruction::GEQ { .. } => InstructionKind::GEQ,
      Instruction::CallDirect { .. } => InstructionKind::CallDirect,
      Instruction::CallIndirect { .. } => InstructionKind::CallIndirect,
      Instruction::IfBlock { .. } => InstructionKind::IfBlock,
      Instruction::LoopBlock { .. } => InstructionKind::LoopBlock,
      Instruction::Break { .. } => InstructionKind::Break,
      Instruction::Continue { .. } => InstructionKind::Continue,
      Instruction::Return { .. } => InstructionKind::Return,
      Instruction::ReturnVoid { .. } => InstructionKind::ReturnVoid,
    }
  }
}

impl Encode for Instruction {
  fn encode (&self, buff: &mut Vec<u8>) {
    use Instruction::*;

    self.get_kind().encode(buff);

    match self {
      | NoOp
      | Load
      | Store
      | Discard
      | Add
      | Sub
      | Mul
      | Div
      | Rem
      | Neg
      | And
      | Or
      | Xor
      | LShift
      | RShift
      | Not
      | EQ
      | NEQ
      | LT
      | GT
      | LEQ
      | GEQ
      | CallIndirect
      | Break
      | Continue
      | Return
      | ReturnVoid
      => { },

      ImmediateValue(imm) => imm.encode(buff),

      CreateLocal(t_idx) => t_idx.encode(buff),
      LocalAddress(l_idx) => l_idx.encode(buff),
      GlobalAddress(g_idx) => g_idx.encode(buff),
      FunctionAddress(f_idx) => f_idx.encode(buff),
      GetElement(e_idx) => e_idx.encode(buff),
      Cast(t_idx) => t_idx.encode(buff),
      CallDirect(f_idx) => f_idx.encode(buff),

      IfBlock(then_instrs, else_instrs) => {
        then_instrs.encode(buff);
        else_instrs.encode(buff);
      },

      LoopBlock(loop_instrs) => loop_instrs.encode(buff),
    }
  }
}

impl Decode for Instruction {
  fn decode (buff: &mut &[u8]) -> Result<Instruction, DecodeError> {
    Ok(match InstructionKind::decode(buff)? {
      InstructionKind::NoOp => Instruction::NoOp,
      InstructionKind::Load => Instruction::Load,
      InstructionKind::Store => Instruction::Store,
      InstructionKind::Discard => Instruction::Discard,
      InstructionKind::Add => Instruction::Add,
      InstructionKind::Sub => Instruction::Sub,
      InstructionKind::Mul => Instruction::Mul,
      InstructionKind::Div => Instruction::Div,
      InstructionKind::Rem => Instruction::Rem,
      InstructionKind::Neg => Instruction::Neg,
      InstructionKind::And => Instruction::And,
      InstructionKind::Or => Instruction::Or,
      InstructionKind::Xor => Instruction::Xor,
      InstructionKind::LShift => Instruction::LShift,
      InstructionKind::RShift => Instruction::RShift,
      InstructionKind::Not => Instruction::Not,
      InstructionKind::EQ => Instruction::EQ,
      InstructionKind::NEQ => Instruction::NEQ,
      InstructionKind::LT => Instruction::LT,
      InstructionKind::GT => Instruction::GT,
      InstructionKind::LEQ => Instruction::LEQ,
      InstructionKind::GEQ => Instruction::GEQ,
      InstructionKind::CallIndirect => Instruction::CallIndirect,
      InstructionKind::Break => Instruction::Break,
      InstructionKind::Continue => Instruction::Continue,
      InstructionKind::Return => Instruction::Return,
      InstructionKind::ReturnVoid => Instruction::ReturnVoid,

      InstructionKind::ImmediateValue => Instruction::ImmediateValue(ImmediateValue::decode(buff)?),

      InstructionKind::CreateLocal => Instruction::CreateLocal(TypeID::decode(buff)?),
      InstructionKind::LocalAddress => Instruction::LocalAddress(LocalID::decode(buff)?),
      InstructionKind::GlobalAddress => Instruction::GlobalAddress(GlobalID::decode(buff)?),
      InstructionKind::FunctionAddress => Instruction::FunctionAddress(FunctionID::decode(buff)?),
      InstructionKind::GetElement => Instruction::GetElement(ElementID::decode(buff)?),
      InstructionKind::Cast => Instruction::Cast(TypeID::decode(buff)?),
      InstructionKind::CallDirect => Instruction::CallDirect(FunctionID::decode(buff)?),

      InstructionKind::IfBlock => Instruction::IfBlock(Vec::decode(buff)?, Vec::decode(buff)?),
      InstructionKind::LoopBlock => Instruction::LoopBlock(Vec::decode(buff)?),
    })
  }
}



/// A data-less variant only version of Instruction
/// 
/// See Instruction for docs on each variant
#[repr(u8)]
#[allow(missing_docs)]
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub enum InstructionKind {
  NoOp,

  ImmediateValue,

  CreateLocal,

  LocalAddress,
  GlobalAddress,
  FunctionAddress,
  
  GetElement,

  Cast,
  
  Load,
  Store,

  Discard,

  Add,
  Sub,
  Mul,
  Div,
  Rem,
  Neg,

  And,
  Or,
  Xor,
  LShift,
  RShift,
  Not,

  EQ,
  NEQ,
  LT,
  GT,
  LEQ,
  GEQ,

  CallDirect,

  CallIndirect,

  IfBlock,

  LoopBlock,

  Break,
  Continue,

  Return,
  ReturnVoid,
}

impl Default for InstructionKind { fn default () -> Self { Self::NoOp } }

impl Encode for InstructionKind {
  fn encode (&self, buff: &mut Vec<u8>) {
    buff.push(*self as _)
  }
}

impl Decode for InstructionKind {
  fn decode (buff: &mut &[u8]) -> Result<InstructionKind, DecodeError> {
    let byte = u8::decode(buff)?;
    
    if byte >= InstructionKind::NoOp as _
    && byte <= InstructionKind::ReturnVoid  as _ {
      Ok(unsafe { transmute(byte) })
    } else {
      Err(DecodeError::UnexpectedValue)
    }
  }
}



/// A literal value encoded directly into a bytecode instruction
#[derive(Debug, Clone, Copy, PartialEq, PartialOrd)]
pub enum ImmediateValue {
  /// A binary state
  Bool(bool),
  /// 8 bit unsigned integer
  U8(u8),
  /// 16 bit unsigned integer
  U16(u16),
  /// 32 bit unsigned integer
  U32(u32),
  /// 64 bit unsigned integer
  U64(u64),
  /// 8 bit signed integer
  S8(i8),
  /// 16 bit signed integer
  S16(i16),
  /// 32 bit signed integer
  S32(i32),
  /// 64 bit signed integer
  S64(i64),
  /// 32 bit real
  F32(f32),
  /// 64 bit real
  F64(f64),
}

impl ImmediateValue {
  /// Get the IntrinsicType of an ImmediateValue
  pub fn get_intrinsic_type (&self) -> IntrinsicType {
    match self {
      Self::Bool(_) => IntrinsicType::Bool,
      Self::U8(_) => IntrinsicType::U8,
      Self::U16(_) => IntrinsicType::U16,
      Self::U32(_) => IntrinsicType::U32,
      Self::U64(_) => IntrinsicType::U64,
      Self::S8(_) => IntrinsicType::S8,
      Self::S16(_) => IntrinsicType::S16,
      Self::S32(_) => IntrinsicType::S32,
      Self::S64(_) => IntrinsicType::S64,
      Self::F32(_) => IntrinsicType::F32,
      Self::F64(_) => IntrinsicType::F64,
    }
  }
}

impl Encode for ImmediateValue {
  fn encode (&self, buff: &mut Vec<u8>) {
    use ImmediateValue::*;

    self.get_intrinsic_type().encode(buff);

    match self {
      &Bool(x) => x.encode(buff),

      &U8(x) => x.encode(buff),
      U16(x) => x.encode(buff),
      U32(x) => x.encode(buff),
      U64(x) => x.encode(buff),
      &S8(x) => x.encode(buff),
      S16(x) => x.encode(buff),
      S32(x) => x.encode(buff),
      S64(x) => x.encode(buff),
      F32(x) => x.encode(buff),
      F64(x) => x.encode(buff),
    }
  }
}

impl Decode for ImmediateValue {
  fn decode (buff: &mut &[u8]) -> Result<ImmediateValue, DecodeError> {
    Ok(match IntrinsicType::decode(buff)? {
      IntrinsicType::Bool => bool::decode(buff)?.into(),

      IntrinsicType::U8  => u8 ::decode(buff)?.into(),
      IntrinsicType::U16 => u16::decode(buff)?.into(),
      IntrinsicType::U32 => u32::decode(buff)?.into(),
      IntrinsicType::U64 => u64::decode(buff)?.into(),
      IntrinsicType::S8  => i8 ::decode(buff)?.into(),
      IntrinsicType::S16 => i16::decode(buff)?.into(),
      IntrinsicType::S32 => i32::decode(buff)?.into(),
      IntrinsicType::S64 => i64::decode(buff)?.into(),
      IntrinsicType::F32 => f32::decode(buff)?.into(),
      IntrinsicType::F64 => f64::decode(buff)?.into(),
      
      IntrinsicType::Void => return Err(DecodeError::UnexpectedValue),
    })
  }
}

impl From<bool> for ImmediateValue { fn from (u: bool) -> Self { Self::Bool (u) } }

impl From<u8>  for ImmediateValue { fn from (u: u8)  -> Self { Self::U8  (u) } }
impl From<u16> for ImmediateValue { fn from (u: u16) -> Self { Self::U16 (u) } }
impl From<u32> for ImmediateValue { fn from (u: u32) -> Self { Self::U32 (u) } }
impl From<u64> for ImmediateValue { fn from (u: u64) -> Self { Self::U64 (u) } }
impl From<i8>  for ImmediateValue { fn from (s: i8)  -> Self { Self::S8  (s) } }
impl From<i16> for ImmediateValue { fn from (s: i16) -> Self { Self::S16 (s) } }
impl From<i32> for ImmediateValue { fn from (s: i32) -> Self { Self::S32 (s) } }
impl From<i64> for ImmediateValue { fn from (s: i64) -> Self { Self::S64 (s) } }
impl From<f32> for ImmediateValue { fn from (f: f32) -> Self { Self::F32 (f) } }
impl From<f64> for ImmediateValue { fn from (f: f64) -> Self { Self::F64 (f) } }

impl From<usize> for ImmediateValue { fn from (u: usize) -> Self { Self::U64 (u as _) } }
impl From<isize> for ImmediateValue { fn from (s: isize) -> Self { Self::S64 (s as _) } }



impl Encode for str {
  fn encode (&self, buff: &mut Vec<u8>) {
    self.len().encode(buff);
    self.as_bytes().encode(buff);
  }
}

impl Encode for bool { fn encode (&self, buff: &mut Vec<u8>) { buff.push(*self as _); } }
impl Encode for u8   { fn encode (&self, buff: &mut Vec<u8>) { buff.push(*self as _); } }
impl Encode for i8   { fn encode (&self, buff: &mut Vec<u8>) { buff.push(*self as _); } }

impl Encode for u16 { fn encode (&self, buff: &mut Vec<u8>) { self.to_le_bytes().encode(buff) } }
impl Encode for u32 { fn encode (&self, buff: &mut Vec<u8>) { self.to_le_bytes().encode(buff) } }
impl Encode for u64 { fn encode (&self, buff: &mut Vec<u8>) { self.to_le_bytes().encode(buff) } }
impl Encode for i16 { fn encode (&self, buff: &mut Vec<u8>) { self.to_le_bytes().encode(buff) } }
impl Encode for i32 { fn encode (&self, buff: &mut Vec<u8>) { self.to_le_bytes().encode(buff) } }
impl Encode for i64 { fn encode (&self, buff: &mut Vec<u8>) { self.to_le_bytes().encode(buff) } }
impl Encode for f32 { fn encode (&self, buff: &mut Vec<u8>) { self.to_le_bytes().encode(buff) } }
impl Encode for f64 { fn encode (&self, buff: &mut Vec<u8>) { self.to_le_bytes().encode(buff) } }

impl Encode for usize { fn encode (&self, buff: &mut Vec<u8>) { (*self as u64).encode(buff) } }
impl Encode for isize { fn encode (&self, buff: &mut Vec<u8>) { (*self as i64).encode(buff) } }

impl<E: Encode> Encode for [E] {
  fn encode (&self, buff: &mut Vec<u8>) {
    self.iter().for_each(|e| e.encode(buff))
  }
}

impl<E: Encode> Encode for Vec<E> {
  fn encode (&self, buff: &mut Vec<u8>) {
    self.len().encode(buff);
    self.iter().for_each(|e| e.encode(buff))
  }
}

impl<E: Encode> Encode for Option<E> {
  fn encode (&self, buff: &mut Vec<u8>) {
    match self {
      Some(e) => {
        true.encode(buff);
        e.encode(buff);
      },
      None => false.encode(buff)
    }
  }
}


impl Decode for u8 {
  fn decode (buff: &mut &[u8]) -> Result<u8, DecodeError> {
    match buff.get(0) {
      Some(&b) => { *buff = &buff[1..]; Ok(b) },
      None =>  Err(DecodeError::EOF)
    }
  }
}


impl Decode for bool { fn decode (buff: &mut &[u8]) -> Result<bool, DecodeError> { Ok(u8::decode(buff)? == 1 ) } }
impl Decode for i8   { fn decode (buff: &mut &[u8]) -> Result<i8,   DecodeError> { Ok(u8::decode(buff)? as i8) } }

type BytePair  = [u8; 2];

impl Decode for BytePair  {
  fn decode (buff: &mut &[u8]) -> Result<BytePair,  DecodeError> {
    if buff.len() >= 2 {
      let arr = unsafe { * (buff.as_ptr() as *const BytePair) };
      *buff = &buff[2..];
      Ok(arr)
    } else {
      Err(DecodeError::EOF)
    }
  }
}

type ByteQuad  = [u8; 4];

impl Decode for ByteQuad  {
  fn decode (buff: &mut &[u8]) -> Result<ByteQuad,  DecodeError> {
    if buff.len() >= 4 {
      let arr = unsafe { * (buff.as_ptr() as *const ByteQuad) };
      *buff = &buff[4..];
      Ok(arr)
    } else {
      Err(DecodeError::EOF)
    }
  }
}

type ByteOctet = [u8; 8];

impl Decode for ByteOctet  {
  fn decode (buff: &mut &[u8]) -> Result<ByteOctet,  DecodeError> {
    if buff.len() >= 8 {
      let arr = unsafe { *(buff.as_ptr() as *const ByteOctet) };
      *buff = &buff[8..];
      Ok(arr)
    } else {
      Err(DecodeError::EOF)
    }
  }
}

impl Decode for u16 { fn decode (buff: &mut &[u8]) -> Result<u16, DecodeError> { Ok(u16::from_le_bytes( BytePair::decode(buff)?)) }}
impl Decode for u32 { fn decode (buff: &mut &[u8]) -> Result<u32, DecodeError> { Ok(u32::from_le_bytes( ByteQuad::decode(buff)?)) }}
impl Decode for u64 { fn decode (buff: &mut &[u8]) -> Result<u64, DecodeError> { Ok(u64::from_le_bytes(ByteOctet::decode(buff)?)) }}

impl Decode for i16 { fn decode (buff: &mut &[u8]) -> Result<i16, DecodeError> { Ok(i16::from_le_bytes( BytePair::decode(buff)?)) }}
impl Decode for i32 { fn decode (buff: &mut &[u8]) -> Result<i32, DecodeError> { Ok(i32::from_le_bytes( ByteQuad::decode(buff)?)) }}
impl Decode for i64 { fn decode (buff: &mut &[u8]) -> Result<i64, DecodeError> { Ok(i64::from_le_bytes(ByteOctet::decode(buff)?)) }}

impl Decode for f32 { fn decode (buff: &mut &[u8]) -> Result<f32, DecodeError> { Ok(f32::from_le_bytes( ByteQuad::decode(buff)?)) }}
impl Decode for f64 { fn decode (buff: &mut &[u8]) -> Result<f64, DecodeError> { Ok(f64::from_le_bytes(ByteOctet::decode(buff)?)) }}

impl Decode for usize { fn decode (buff: &mut &[u8]) -> Result<usize, DecodeError> { Ok(u64::decode(buff)? as _) } }
impl Decode for isize { fn decode (buff: &mut &[u8]) -> Result<isize, DecodeError> { Ok(i64::decode(buff)? as _) } }


impl Decode for String {
  fn decode (buff: &mut &[u8]) -> Result<String, DecodeError> {
    let length = u64::decode(buff)? as usize;

    if buff.len() < length { return Err(DecodeError::EOF) }

    match std::str::from_utf8(&buff[..length]) {
      Ok(str) => {
        let string = str.to_string();

        *buff = &buff[length..];
        
        Ok(string)
      },
      Err(_) => Err(DecodeError::InvalidString)
    }
  }
}

impl<D: Decode> Decode for Vec<D> {
  fn decode (buff: &mut &[u8]) -> Result<Vec<D>, DecodeError> {
    let length = u64::decode(buff)? as usize;

    let mut vec = Vec::with_capacity(length);

    for _ in 0..length {
      vec.push(D::decode(buff)?)
    }

    Ok(vec)
  }
}

impl<D: Decode> Decode for Option<D> {
  fn decode (buff: &mut &[u8]) -> Result<Option<D>, DecodeError> {
    let is_some = bool::decode(buff)?;
    if is_some {
      Ok(Some(D::decode(buff)?))
    } else {
      Ok(None)
    }
  }
}



#[cfg(test)]
mod test {
  use super::*;

  #[test]
  fn test_bytecode_encode_decode () {
    let module = Module {
      name: "test_module".to_owned(),
      version: Version::new(0, 0, 1),
      types: vec! [
        Type::new(0.into(), TypeData::Intrinsic(IntrinsicType::S64)),
        Type::new(1.into(), TypeData::Function { parameters: vec! [ 0.into(), 0.into() ], result: Some(0.into()) }),
        Type::new(2.into(), TypeData::Function { parameters: vec! [ ], result: Some(0.into()) }),
      ],
      imports: vec! [
        ImportModule {
          name: "test_import_module".to_owned(),
          version: Version::new(1, 2, 0),
          items: vec! [
            Import::new("test_import_namespace".to_owned(), ImportData::Namespace(vec! [
              Import::new("test_import_global".to_owned(), ImportData::Global(0.into(), 0.into()))
            ])),
            Import::new("test_import_function".to_owned(), ImportData::Function(0.into(), 1.into())),
          ]
        }
      ],
      globals: vec! [
        Global {
          id: 1.into(),
          ty: 0.into(),
          initializer: vec! [
            Instruction::ImmediateValue(ImmediateValue::S64(99))
          ]
        },
        Global {
          id: 2.into(),
          ty: 0.into(),
          initializer: vec! [
            Instruction::CallDirect(1.into())
          ]
        }
      ],
      functions: vec! [
        Function {
          id: 1.into(),
          ty: 2.into(),
          body: vec! [
            Instruction::GlobalAddress(1.into()),
            Instruction::Load,
            Instruction::ImmediateValue(ImmediateValue::S64(1)),
            Instruction::CallDirect(2.into()),
            Instruction::Return,
          ]
        },
        Function {
          id: 2.into(),
          ty: 1.into(),
          body: vec! [
            Instruction::LocalAddress(0.into()),
            Instruction::LocalAddress(1.into()),
            Instruction::Sub,
            Instruction::Return,
          ]
        }
      ],
      exports: vec! [
        Export::new("test_export_namespace".to_owned(), ExportData::Namespace(vec! [
          Export::new("test_export_function".to_owned(), ExportData::Function(1.into())),
        ])),
        Export::new("test_export_global".to_owned(), ExportData::Global(1.into())),
        Export::new("test_reexport".to_owned(), ExportData::Global(0.into())),
      ]
    };

    let mut encoded = Vec::default();
    module.encode(&mut encoded);

    let mut decoder = encoded.as_slice();
    let decoded = Module::decode(&mut decoder).expect("Failed to decode module");

    assert_eq!(module, decoded)
  }

  #[test]
  fn test_all_instruction_encode_decode () {
    use Instruction::*;
    
    let instructions = vec! [
      NoOp,

      ImmediateValue(99i32.into()),

      CreateLocal(64.into()),

      LocalAddress(12.into()),
      GlobalAddress(13.into()),
      FunctionAddress(14.into()),
      
      GetElement(55.into()),

      Cast(11.into()),
      
      Load,
      Store,

      Discard,

      Add,
      Sub,
      Mul,
      Div,
      Rem,
      Neg,

      And,
      Or,
      Xor,
      LShift,
      RShift,
      Not,

      EQ,
      NEQ,
      LT,
      GT,
      LEQ,
      GEQ,

      CallDirect(4.into()),

      CallIndirect,

      IfBlock(vec! [ // random instructions
        GetElement(55.into()),
        Cast(11.into()),
        Return,
        ReturnVoid,
        Load,
        Store,
        IfBlock(vec! [ //nesting
          Div,
          Rem,
          Return,
          ReturnVoid,
          NoOp,
          Neg,
        ], vec! [
          EQ,
          NoOp,
          Return,
          ReturnVoid,
          ImmediateValue(99i32.into()),
          NEQ,
          LT,
        ])
      ], vec![
        LShift,
        RShift,
        Not,
        FunctionAddress(14.into()),
        Return,
        ReturnVoid,
        GetElement(55.into()),
        Cast(11.into()),
        LoopBlock(vec! [ //nesting
          NoOp,
          Return,
          ReturnVoid,
          ImmediateValue(99i32.into()),
          CreateLocal(64.into()),
          IfBlock(vec! [
            Div,
            Rem,
            Neg,
          ], vec! [
            EQ,
            NEQ,
            LT,
          ])
        ]),
      ]),

      LoopBlock(vec! [ //random instructions
        LShift,
        RShift,
        Not,
        FunctionAddress(14.into()),
        IfBlock(vec! [ //nesting
          Div,
          Neg,
          Rem,
        ], vec! [
          NEQ,
          EQ,
          Break,
          LT,
        ]),
        Break,
      ]),

      Break,
      Continue,

      Return,
      ReturnVoid,
    ];

    let mut encoded = Vec::default();
    instructions.encode(&mut encoded);

    let mut decoder = encoded.as_slice();
    let decoded = Vec::decode(&mut decoder).expect("Failed to decode instructions");

    assert_eq!(instructions, decoded)
  }

  #[test]
  fn test_enum_encode_decode () {
    let bad_data = vec! [ 255 ];


    let type_data_kinds = vec! [
      TypeDataKind::Function,
      TypeDataKind::Intrinsic,
      TypeDataKind::Pointer,
      TypeDataKind::Struct,
    ];

    let mut encoded = Vec::default();
    type_data_kinds.encode(&mut encoded);

    let mut decoder = encoded.as_slice();
    let decoded = Vec::decode(&mut decoder).expect("Failed to decode type_data_kinds");

    assert_eq!(type_data_kinds, decoded);

    let mut decoder = bad_data.as_slice();
    TypeDataKind::decode(&mut decoder).expect_err("TypeDataKind decoder failed to reject out of range value");


    let intrinsic_types = vec! [
      IntrinsicType::Void,
      IntrinsicType::Bool,
      IntrinsicType::U8,
      IntrinsicType::U16,
      IntrinsicType::U32,
      IntrinsicType::U64,
      IntrinsicType::S8,
      IntrinsicType::S16,
      IntrinsicType::S32,
      IntrinsicType::S64,
      IntrinsicType::F32,
      IntrinsicType::F64,
    ];

    let mut encoded = Vec::default();
    intrinsic_types.encode(&mut encoded);
    
    let mut decoder = encoded.as_slice();
    let decoded = Vec::decode(&mut decoder).expect("Failed to decode intrinsic_types");

    assert_eq!(intrinsic_types, decoded);

    let mut decoder = bad_data.as_slice();
    IntrinsicType::decode(&mut decoder).expect_err("IntrinsicType decoder failed to reject out of range value");


    let alias_data_kinds = vec! [
      AliasDataKind::Function,
      AliasDataKind::Global,
      AliasDataKind::Namespace,
    ];

    let mut encoded = Vec::default();
    alias_data_kinds.encode(&mut encoded);
    
    let mut decoder = encoded.as_slice();
    let decoded = Vec::decode(&mut decoder).expect("Failed to decode alias_data_kinds");

    assert_eq!(alias_data_kinds, decoded);

    let mut decoder = bad_data.as_slice();
    AliasDataKind::decode(&mut decoder).expect_err("AliasDataKind decoder failed to reject out of range value");


    let instruction_kinds = vec! [
      InstructionKind::NoOp,
      InstructionKind::ImmediateValue,
      InstructionKind::CreateLocal,
      InstructionKind::LocalAddress,
      InstructionKind::GlobalAddress,
      InstructionKind::FunctionAddress,
      InstructionKind::GetElement,
      InstructionKind::Cast,
      InstructionKind::Load,
      InstructionKind::Store,
      InstructionKind::Discard,
      InstructionKind::Add,
      InstructionKind::Sub,
      InstructionKind::Mul,
      InstructionKind::Div,
      InstructionKind::Rem,
      InstructionKind::Neg,
      InstructionKind::And,
      InstructionKind::Or,
      InstructionKind::Xor,
      InstructionKind::LShift,
      InstructionKind::RShift,
      InstructionKind::Not,
      InstructionKind::EQ,
      InstructionKind::NEQ,
      InstructionKind::LT,
      InstructionKind::GT,
      InstructionKind::LEQ,
      InstructionKind::GEQ,
      InstructionKind::CallDirect,
      InstructionKind::CallIndirect,
      InstructionKind::IfBlock,
      InstructionKind::LoopBlock,
      InstructionKind::Break,
      InstructionKind::Continue,
      InstructionKind::Return,
      InstructionKind::ReturnVoid,
    ];

    let mut encoded = Vec::default();
    instruction_kinds.encode(&mut encoded);
    
    let mut decoder = encoded.as_slice();
    let decoded = Vec::decode(&mut decoder).expect("Failed to decode instruction_kinds");

    assert_eq!(instruction_kinds, decoded);

    let mut decoder = bad_data.as_slice();
    InstructionKind::decode(&mut decoder).expect_err("InstructionKind decoder failed to reject out of range value");
  }
}