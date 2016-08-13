#[derive(Debug)]
pub enum Attribute {
    Part(Part),
    FileFunction(FileFunction),
    FilePolarity,
    GenerationSoftware,
    CreationDate,
    ProjectId,
    MD5,
    UserDefined { name: String, value: Vec<String> },
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
