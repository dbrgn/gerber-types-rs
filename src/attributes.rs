//! Attributes.

use std::io::Write;

use chrono::{DateTime, Utc};
use uuid::Uuid;

use crate::errors::GerberResult;
use crate::traits::PartialGerberCode;

// FileAttribute

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum FileAttribute {
    Part(Part),
    FileFunction(FileFunction),
    FilePolarity(FilePolarity),
    GenerationSoftware(GenerationSoftware),
    CreationDate(DateTime<Utc>),
    ProjectId {
        id: String,
        guid: Uuid,
        revision: String,
    },
    Md5(String),
    UserDefined {
        name: String,
        value: Vec<String>,
    },
}

impl<W: Write> PartialGerberCode<W> for FileAttribute {
    fn serialize_partial(&self, writer: &mut W) -> GerberResult<()> {
        match *self {
            FileAttribute::Part(ref part) => {
                write!(writer, "Part,")?;
                part.serialize_partial(writer)?;
            }
            FileAttribute::FileFunction(ref function) => {
                write!(writer, "FileFunction,")?;
                match function {
                    &FileFunction::Copper {
                        ref layer,
                        ref pos,
                        ref copper_type,
                    } => {
                        write!(writer, "Copper,L{},", layer)?;
                        pos.serialize_partial(writer)?;
                        if let Some(ref t) = *copper_type {
                            write!(writer, ",")?;
                            t.serialize_partial(writer)?;
                        }
                    }
                    &FileFunction::Profile(ref plating) => {
                        write!(writer, "Profile,")?;
                        plating.serialize_partial(writer)?;
                    }
                    &FileFunction::Soldermask { ref pos, ref index } => {
                        write!(writer, "Soldermask,")?;
                        pos.serialize_partial(writer)?;
                        if let Some(ref i) = index {
                            write!(writer, ",{}", *i)?;
                        }
                    }
                    &FileFunction::Legend { ref pos, ref index } => {
                        write!(writer, "Legend,")?;
                        pos.serialize_partial(writer)?;
                        if let Some(ref i) = index {
                            write!(writer, ",{}", *i)?;
                        }
                    }
                    _ => unimplemented!(),
                }
            }
            FileAttribute::GenerationSoftware(ref gs) => {
                write!(writer, "GenerationSoftware,")?;
                gs.serialize_partial(writer)?;
            }
            FileAttribute::FilePolarity(ref p) => {
                write!(writer, "FilePolarity,")?;
                p.serialize_partial(writer)?;
            }
            FileAttribute::Md5(ref hash) => write!(writer, "MD5,{}", hash)?,
            _ => unimplemented!(),
        };
        Ok(())
    }
}

// ApertureAttribute

#[derive(Debug, Clone, PartialEq)]
pub enum ApertureAttribute {
    ApertureFunction(ApertureFunction),
    DrillTolerance { plus: f64, minus: f64 },
}

// Part

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum Part {
    /// Single PCB
    Single,
    /// A.k.a. customer panel, assembly panel, shipping panel, biscuit
    Array,
    /// A.k.a. working panel, production panel
    FabricationPanel,
    /// A test coupon
    Coupon,
    /// None of the above
    Other(String),
}

impl<W: Write> PartialGerberCode<W> for Part {
    fn serialize_partial(&self, writer: &mut W) -> GerberResult<()> {
        match *self {
            Part::Single => write!(writer, "Single")?,
            Part::Array => write!(writer, "Array")?,
            Part::FabricationPanel => write!(writer, "FabricationPanel")?,
            Part::Coupon => write!(writer, "Coupon")?,
            Part::Other(ref description) => write!(writer, "Other,{}", description)?,
        };
        Ok(())
    }
}

// Position

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum Position {
    Top,
    Bottom,
}

impl<W: Write> PartialGerberCode<W> for Position {
    fn serialize_partial(&self, writer: &mut W) -> GerberResult<()> {
        match *self {
            Position::Top => write!(writer, "Top")?,
            Position::Bottom => write!(writer, "Bot")?,
        };
        Ok(())
    }
}

// ExtendedPosition

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum ExtendedPosition {
    Top,
    Inner,
    Bottom,
}

impl<W: Write> PartialGerberCode<W> for ExtendedPosition {
    fn serialize_partial(&self, writer: &mut W) -> GerberResult<()> {
        match *self {
            ExtendedPosition::Top => write!(writer, "Top")?,
            ExtendedPosition::Inner => write!(writer, "Inr")?,
            ExtendedPosition::Bottom => write!(writer, "Bot")?,
        };
        Ok(())
    }
}

// CopperType

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum CopperType {
    Plane,
    Signal,
    Mixed,
    Hatched,
}

impl<W: Write> PartialGerberCode<W> for CopperType {
    fn serialize_partial(&self, writer: &mut W) -> GerberResult<()> {
        match *self {
            CopperType::Plane => write!(writer, "Plane")?,
            CopperType::Signal => write!(writer, "Signal")?,
            CopperType::Mixed => write!(writer, "Mixed")?,
            CopperType::Hatched => write!(writer, "Hatched")?,
        };
        Ok(())
    }
}

// Drill

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum Drill {
    ThroughHole,
    Blind,
    Buried,
}

// DrillRouteType

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum DrillRouteType {
    Drill,
    Route,
    Mixed,
}

// Profile

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum Profile {
    Plated,
    NonPlated,
}

impl<W: Write> PartialGerberCode<W> for Profile {
    fn serialize_partial(&self, writer: &mut W) -> GerberResult<()> {
        match *self {
            Profile::Plated => write!(writer, "P")?,
            Profile::NonPlated => write!(writer, "NP")?,
        };
        Ok(())
    }
}

// FileFunction

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum FileFunction {
    Copper {
        layer: i32,
        pos: ExtendedPosition,
        copper_type: Option<CopperType>,
    },
    Soldermask {
        pos: Position,
        index: Option<i32>,
    },
    Legend {
        pos: Position,
        index: Option<i32>,
    },
    Goldmask {
        pos: Position,
        index: Option<i32>,
    },
    Silvermask {
        pos: Position,
        index: Option<i32>,
    },
    Tinmask {
        pos: Position,
        index: Option<i32>,
    },
    Carbonmask {
        pos: Position,
        index: Option<i32>,
    },
    Peelablesoldermask {
        pos: Position,
        index: Option<i32>,
    },
    Glue {
        pos: Position,
        index: Option<i32>,
    },
    Viatenting(Position),
    Viafill,
    Heatsink(Position),
    Paste(Position),
    KeepOut(Position),
    Pads(Position),
    Scoring(Position),
    Plated {
        from_layer: i32,
        to_layer: i32,
        drill: Drill,
        label: Option<DrillRouteType>,
    },
    NonPlated {
        from_layer: i32,
        to_layer: i32,
        drill: Drill,
        label: Option<DrillRouteType>,
    },
    Profile(Profile),
    Drillmap,
    FabricationDrawing,
    ArrayDrawing,
    AssemblyDrawing(Position),
    Drawing(String),
    Other(String),
}

// FilePolarity

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum FilePolarity {
    Positive,
    Negative,
}

impl<W: Write> PartialGerberCode<W> for FilePolarity {
    fn serialize_partial(&self, writer: &mut W) -> GerberResult<()> {
        match *self {
            FilePolarity::Positive => write!(writer, "Positive")?,
            FilePolarity::Negative => write!(writer, "Negative")?,
        };
        Ok(())
    }
}

// GenerationSoftware

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct GenerationSoftware {
    pub vendor: String,
    pub application: String,
    pub version: Option<String>,
}

impl GenerationSoftware {
    pub fn new<S: Into<String>>(vendor: S, application: S, version: Option<S>) -> Self {
        GenerationSoftware {
            vendor: vendor.into(),
            application: application.into(),
            version: version.map(|s| s.into()),
        }
    }
}

impl<W: Write> PartialGerberCode<W> for GenerationSoftware {
    fn serialize_partial(&self, writer: &mut W) -> GerberResult<()> {
        match self.version {
            Some(ref v) => write!(writer, "{},{},{}", self.vendor, self.application, v)?,
            None => write!(writer, "{},{}", self.vendor, self.application)?,
        };
        Ok(())
    }
}

// ApertureFunction

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum ApertureFunction {
    // Only valid for layers with file function plated or non-plated
    ViaDrill,
    BackDrill,
    ComponentDrill {
        press_fit: Option<bool>, // TODO is this bool?
    },
    CastellatedDrill,
    MechanicalDrill {
        function: Option<DrillFunction>,
    },
    Slot,
    CutOut,
    Cavity,
    OtherDrill(String),

    // Only valid for layers with file function copper
    ComponentPad {
        press_fit: Option<bool>, // TODO is this bool?
    },
    SmdPad(SmdPadType),
    BgaPad(SmdPadType),
    ConnectorPad,
    HeatsinkPad,
    ViaPad,
    TestPad,
    CastellatedPad,
    FiducialPad(FiducialScope),
    ThermalReliefPad,
    WasherPad,
    AntiPad,
    OtherPad(String),
    Conductor,
    NonConductor,
    CopperBalancing,
    Border,
    OtherCopper(String),

    // All layers
    Profile,
    NonMaterial,
    Material,
    Other(String),
}

// DrillFunction

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum DrillFunction {
    BreakOut,
    Tooling,
    Other,
}

// SmdPadType

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum SmdPadType {
    CopperDefined,
    SoldermaskDefined,
}

// FiducialScope

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum FiducialScope {
    Global,
    Local,
}
