use chrono::{DateTime, UTC};
use uuid::Uuid;

#[derive(Debug)]
pub enum FileAttribute {
    Part(Part),
    FileFunction(FileFunction),
    FilePolarity(FilePolarity),
    GenerationSoftware(GenerationSoftware),
    CreationDate(DateTime<UTC>),
    ProjectId {
        id: String,
        guid: Uuid,
        revision: String,
    },
    Md5(String),
    UserDefined { name: String, value: Vec<String> },
}

#[derive(Debug)]
pub enum ApertureAttribute {
    ApertureFunction(ApertureFunction),
    DrillTolerance {
        plus: f64,
        minus: f64,
    },
}

#[derive(Debug)]
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

#[derive(Debug)]
pub enum Position {
    Top,
    Bottom,
}

#[derive(Debug)]
pub enum ExtendedPosition {
    Top,
    Inner,
    Bottom,
}

#[derive(Debug)]
pub enum CopperType {
    Plane,
    Signal,
    Mixed,
    Hatched,
}

#[derive(Debug)]
pub enum Drill {
    ThroughHole,
    Blind,
    Buried,
}

#[derive(Debug)]
pub enum DrillRouteType {
    Drill,
    Route,
    Mixed,
}

#[derive(Debug)]
pub enum Profile {
    Plated,
    NonPlated,
}

#[derive(Debug)]
pub enum FileFunction {
    Copper { layer: i32, pos: ExtendedPosition, copper_type: Option<CopperType> },
    Soldermask { pos: Position, index: Option<i32> },
    Legend { pos: Position, index: Option<i32> },
    Goldmask { pos: Position, index: Option<i32> },
    Silvermask { pos: Position, index: Option<i32> },
    Tinmask { pos: Position, index: Option<i32> },
    Carbonmask { pos: Position, index: Option<i32> },
    Peelablesoldermask { pos: Position, index: Option<i32> },
    Glue { pos: Position, index: Option<i32> },
    Viatenting(Position),
    Viafill,
    Heatsink(Position),
    Paste(Position),
    KeepOut(Position),
    Pads(Position),
    Scoring(Position),
    Plated { from_layer: i32, to_layer: i32, drill: Drill, label: Option<DrillRouteType> },
    NonPlated { from_layer: i32, to_layer: i32, drill: Drill, label: Option<DrillRouteType> },
    Profile(Profile),
    Drillmap,
    FabricationDrawing,
    ArrayDrawing,
    AssemblyDrawing(Position),
    Drawing(String),
    Other(String),
}

#[derive(Debug)]
pub enum FilePolarity {
    Positive,
    Negative,
}

#[derive(Debug)]
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

#[derive(Debug)]
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

#[derive(Debug)]
pub enum DrillFunction {
    BreakOut,
    Tooling,
    Other,
}

#[derive(Debug)]
pub enum SmdPadType {
    CopperDefined,
    SoldermaskDefined,
}

#[derive(Debug)]
pub enum FiducialScope{
    Global,
    Local,
}
