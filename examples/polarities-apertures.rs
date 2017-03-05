//! Example from spec chapter 2.12.2

extern crate gerber_types;
extern crate conv;

use conv::TryFrom;

use gerber_types::*;
use gerber_types::MacroDecimal::Value;

const VERSION: &'static str = env!("CARGO_PKG_VERSION");

fn main() {
    let cf = CoordinateFormat::new(2, 6);
    let commands: Vec<Command> = vec![
        Command::FunctionCode(
            FunctionCode::GCode(
                GCode::Comment("Ucamco ex. 2: Shapes".to_string())
            )
        ),
        Command::ExtendedCode(ExtendedCode::CoordinateFormat(cf)),
        Command::ExtendedCode(ExtendedCode::Unit(Unit::Inches)),
        Command::ExtendedCode(
            ExtendedCode::FileAttribute(
                FileAttribute::GenerationSoftware(
                    GenerationSoftware::new("Rust Gerber", "gerber-types-rs", Some(VERSION))
                )
            )
        ),
        Command::ExtendedCode(
            ExtendedCode::FileAttribute(
                FileAttribute::Part(Part::Other("Only an example".to_string()))
            )
        ),
        Command::ExtendedCode(
            ExtendedCode::LoadPolarity(Polarity::Dark)
        ),
        Command::FunctionCode(
            FunctionCode::GCode(
                GCode::Comment("Define Apertures".to_string())
            )
        ),
        Command::ExtendedCode(
            ExtendedCode::ApertureMacro(
                ApertureMacro::new("TARGET125").add_primitive(
                    Primitive::Moire(
                        MoirePrimitive {
                            center: (Value(0.0), Value(0.0)),
                            diameter: Value(0.125),
                            ring_thickness: Value(0.01),
                            gap: Value(0.01),
                            max_rings: 3,
                            cross_hair_thickness: Value(0.003),
                            cross_hair_length: Value(0.150),
                            angle: Value(0.0),
                        }
                    )
                )
            )
        ),
        Command::ExtendedCode(
            ExtendedCode::ApertureMacro(
                ApertureMacro::new("THERMAL80").add_primitive(
                    Primitive::Thermal(
                        ThermalPrimitive {
                            center: (Value(0.0), Value(0.0)),
                            outer_diameter: Value(0.08),
                            inner_diameter: Value(0.055),
                            gap: Value(0.0125),
                            angle: Value(45.0),
                        }
                    )
                )
            )
        ),
        Command::ExtendedCode(
            ExtendedCode::ApertureDefinition(
                ApertureDefinition {
                    code: 10,
                    aperture: Aperture::Circle(Circle {
                        diameter: 0.01,
                        hole_diameter: None,
                    })
                }
            )
        ),
        Command::ExtendedCode(
            ExtendedCode::ApertureDefinition(
                ApertureDefinition {
                    code: 11,
                    aperture: Aperture::Circle(Circle {
                        diameter: 0.06,
                        hole_diameter: None,
                    })
                }
            )
        ),
        Command::ExtendedCode(
            ExtendedCode::ApertureDefinition(
                ApertureDefinition {
                    code: 12,
                    aperture: Aperture::Rectangle(Rectangular {
                        x: 0.06,
                        y: 0.06,
                        hole_diameter: None,
                    })
                }
            )
        ),
        Command::ExtendedCode(
            ExtendedCode::ApertureDefinition(
                ApertureDefinition {
                    code: 13,
                    aperture: Aperture::Rectangle(Rectangular {
                        x: 0.04,
                        y: 0.1,
                        hole_diameter: None,
                    })
                }
            )
        ),
        Command::ExtendedCode(
            ExtendedCode::ApertureDefinition(
                ApertureDefinition {
                    code: 14,
                    aperture: Aperture::Rectangle(Rectangular {
                        x: 0.1,
                        y: 0.04,
                        hole_diameter: None,
                    })
                }
            )
        ),
        Command::ExtendedCode(
            ExtendedCode::ApertureDefinition(
                ApertureDefinition {
                    code: 15,
                    aperture: Aperture::Obround(Rectangular {
                        x: 0.04,
                        y: 0.1,
                        hole_diameter: None,
                    })
                }
            )
        ),
        Command::ExtendedCode(
            ExtendedCode::ApertureDefinition(
                ApertureDefinition {
                    code: 16,
                    aperture: Aperture::Polygon(Polygon {
                        diameter: 0.1,
                        vertices: 3,
                        rotation: None,
                        hole_diameter: None,
                    })
                }
            )
        ),
        Command::ExtendedCode(
            ExtendedCode::ApertureDefinition(
                ApertureDefinition {
                    code: 18,
                    aperture: Aperture::Other("TARGET125".to_string())
                }
            )
        ),
        Command::ExtendedCode(
            ExtendedCode::ApertureDefinition(
                ApertureDefinition {
                    code: 19,
                    aperture: Aperture::Other("THERMAL80".to_string())
                }
            )
        ),
        Command::FunctionCode(
            FunctionCode::GCode(
                GCode::Comment("Start image generation".to_string())
            )
        ),
        Command::FunctionCode(
            FunctionCode::DCode(DCode::SelectAperture(10))
        ),
        Command::FunctionCode(
            FunctionCode::DCode(
                DCode::Operation(
                    Operation::Move(
                        Coordinates::new(0, CoordinateNumber::try_from(0.25).unwrap(), cf)
                    )
                )
            )
        ),
        Command::FunctionCode(
            FunctionCode::GCode(GCode::InterpolationMode(InterpolationMode::Linear))
        ),
        Command::FunctionCode(
            FunctionCode::DCode(
                DCode::Operation(
                    Operation::Interpolate(Coordinates::new(0, 0, cf), None)
                )
            )
        ),
        Command::FunctionCode(
            FunctionCode::DCode(
                DCode::Operation(
                    Operation::Interpolate(Coordinates::new(
                        CoordinateNumber::try_from(0.25).unwrap(),
                        0,
                        cf
                    ), None)
                )
            )
        ),
        Command::FunctionCode(
            FunctionCode::DCode(
                DCode::Operation(
                    Operation::Move(Coordinates::new(1, 1, cf))
                )
            )
        ),
        Command::FunctionCode(
            FunctionCode::DCode(
                DCode::Operation(
                    Operation::Interpolate(Coordinates::at_x(
                        CoordinateNumber::try_from(1.5).unwrap(),
                        cf
                    ), None)
                )
            )
        ),
        Command::FunctionCode(
            FunctionCode::DCode(
                DCode::Operation(
                    Operation::Interpolate(Coordinates::new(
                        2,
                        CoordinateNumber::try_from(1.5).unwrap(),
                        cf
                    ), None)
                )
            )
        ),
        Command::FunctionCode(
            FunctionCode::DCode(
                DCode::Operation(
                    Operation::Move(
                        Coordinates::at_x(CoordinateNumber::try_from(2.5).unwrap(), cf)
                    )
                )
            )
        ),
        Command::FunctionCode(
            FunctionCode::DCode(
                DCode::Operation(
                    Operation::Interpolate(Coordinates::at_y(1, cf), None)
                )
            )
        ),
        Command::FunctionCode(
            FunctionCode::DCode(DCode::SelectAperture(11))
        ),
        Command::FunctionCode(
            FunctionCode::DCode(
                DCode::Operation(
                    Operation::Flash(Coordinates::new(1, 1, cf))
                )
            )
        ),
        Command::FunctionCode(
            FunctionCode::DCode(
                DCode::Operation(
                    Operation::Flash(Coordinates::new(2, 1, cf))
                )
            )
        ),
        Command::FunctionCode(
            FunctionCode::DCode(
                DCode::Operation(
                    Operation::Flash(
                        Coordinates::new(CoordinateNumber::try_from(2.5).unwrap(), 1, cf)
                    )
                )
            )
        ),
        Command::FunctionCode(
            FunctionCode::DCode(
                DCode::Operation(
                    Operation::Flash(Coordinates::new(
                        CoordinateNumber::try_from(2.5).unwrap(),
                        CoordinateNumber::try_from(1.5).unwrap(),
                        cf
                    ))
                )
            )
        ),
        Command::FunctionCode(
            FunctionCode::DCode(
                DCode::Operation(
                    Operation::Flash(
                        Coordinates::new(2, CoordinateNumber::try_from(1.5).unwrap(), cf)
                    )
                )
            )
        ),
        Command::FunctionCode(
            FunctionCode::DCode(DCode::SelectAperture(12))
        ),
        Command::FunctionCode(
            FunctionCode::DCode(
                DCode::Operation(
                    Operation::Flash(
                        Coordinates::new(1, CoordinateNumber::try_from(1.5).unwrap(), cf)
                    )
                )
            )
        ),
        Command::FunctionCode(
            FunctionCode::DCode(DCode::SelectAperture(13))
        ),
        Command::FunctionCode(
            FunctionCode::DCode(
                DCode::Operation(
                    Operation::Flash(
                        Coordinates::new(3, CoordinateNumber::try_from(1.5).unwrap(), cf)
                    )
                )
            )
        ),
        Command::FunctionCode(
            FunctionCode::DCode(DCode::SelectAperture(14))
        ),
        Command::FunctionCode(
            FunctionCode::DCode(
                DCode::Operation(
                    Operation::Flash(
                        Coordinates::new(3, CoordinateNumber::try_from(1.25).unwrap(), cf)
                    )
                )
            )
        ),
        Command::FunctionCode(
            FunctionCode::DCode(DCode::SelectAperture(15))
        ),
        Command::FunctionCode(
            FunctionCode::DCode(
                DCode::Operation(
                    Operation::Flash(Coordinates::new(3, 1, cf))
                )
            )
        ),
        Command::FunctionCode(
            FunctionCode::DCode(DCode::SelectAperture(10))
        ),
        Command::FunctionCode(
            FunctionCode::DCode(
                DCode::Operation(
                    Operation::Move(
                        Coordinates::new(CoordinateNumber::try_from(3.75).unwrap(), 1, cf)
                    )
                )
            )
        ),
        Command::FunctionCode(
            FunctionCode::GCode(GCode::QuadrantMode(QuadrantMode::Multi))
        ),
        Command::FunctionCode(
            FunctionCode::GCode(GCode::InterpolationMode(InterpolationMode::CounterclockwiseCircular))
        ),
        Command::FunctionCode(
            FunctionCode::DCode(
                DCode::Operation(
                    Operation::Interpolate(
                        Coordinates::new(CoordinateNumber::try_from(3.75).unwrap(), 1, cf),
                        Some(CoordinateOffset::new(CoordinateNumber::try_from(0.25).unwrap(), 0, cf))
                    )
                )
            )
        ),
        Command::FunctionCode(
            FunctionCode::DCode(DCode::SelectAperture(16))
        ),
        Command::FunctionCode(
            FunctionCode::DCode(
                DCode::Operation(
                    Operation::Flash(
                        Coordinates::new(CoordinateNumber::try_from(3.4).unwrap(), 1, cf)
                    )
                )
            )
        ),
        Command::FunctionCode(
            FunctionCode::DCode(
                DCode::Operation(
                    Operation::Flash(Coordinates::new(
                        CoordinateNumber::try_from(3.5).unwrap(),
                        CoordinateNumber::try_from(0.9).unwrap(),
                        cf
                    ))
                )
            )
        ),
        Command::FunctionCode(
            FunctionCode::DCode(DCode::SelectAperture(10))
        ),
        Command::FunctionCode(
            FunctionCode::GCode(GCode::RegionMode(true))
        ),
        Command::FunctionCode(
            FunctionCode::DCode(
                DCode::Operation(
                    Operation::Move(
                        Coordinates::new(CoordinateNumber::try_from(0.5).unwrap(), 2, cf)
                    )
                )
            )
        ),
        Command::FunctionCode(
            FunctionCode::GCode(GCode::InterpolationMode(InterpolationMode::Linear))
        ),
        Command::FunctionCode(
            FunctionCode::DCode(
                DCode::Operation(
                    Operation::Interpolate(Coordinates::at_y(
                        CoordinateNumber::try_from(3.75).unwrap(),
                        cf
                    ), None)
                )
            )
        ),
        Command::FunctionCode(
            FunctionCode::DCode(
                DCode::Operation(
                    Operation::Interpolate(Coordinates::at_x(
                        CoordinateNumber::try_from(3.75).unwrap(),
                        cf
                    ), None)
                )
            )
        ),
        Command::FunctionCode(
            FunctionCode::DCode(
                DCode::Operation(
                    Operation::Interpolate(Coordinates::at_y(2, cf), None)
                )
            )
        ),
        Command::FunctionCode(
            FunctionCode::DCode(
                DCode::Operation(
                    Operation::Interpolate(Coordinates::at_x(
                        CoordinateNumber::try_from(0.5).unwrap(),
                        cf
                    ), None)
                )
            )
        ),
        Command::FunctionCode(
            FunctionCode::GCode(GCode::RegionMode(false))
        ),
        Command::FunctionCode(
            FunctionCode::DCode(DCode::SelectAperture(18))
        ),
        Command::FunctionCode(
            FunctionCode::DCode(
                DCode::Operation(
                    Operation::Flash(
                        Coordinates::new(0, CoordinateNumber::try_from(3.875).unwrap(), cf)
                    )
                )
            )
        ),
        Command::FunctionCode(
            FunctionCode::DCode(
                DCode::Operation(
                    Operation::Flash(
                        Coordinates::new(
                            CoordinateNumber::try_from(3.875).unwrap(),
                            CoordinateNumber::try_from(3.875).unwrap(),
                            cf
                        )
                    )
                )
            )
        ),
        Command::ExtendedCode(
            ExtendedCode::LoadPolarity(Polarity::Clear)
        ),
        Command::FunctionCode(
            FunctionCode::GCode(GCode::RegionMode(true))
        ),
        Command::FunctionCode(
            FunctionCode::DCode(
                DCode::Operation(
                    Operation::Move(
                        Coordinates::new(1, CoordinateNumber::try_from(2.5).unwrap(), cf)
                    )
                )
            )
        ),
        Command::FunctionCode(
            FunctionCode::DCode(
                DCode::Operation(
                    Operation::Interpolate(Coordinates::at_y(3, cf), None)
                )
            )
        ),
        Command::FunctionCode(
            FunctionCode::GCode(GCode::QuadrantMode(QuadrantMode::Single))
        ),
        Command::FunctionCode(
            FunctionCode::GCode(GCode::InterpolationMode(InterpolationMode::ClockwiseCircular))
        ),
        Command::FunctionCode(
            FunctionCode::DCode(
                DCode::Operation(
                    Operation::Interpolate(
                        Coordinates::new(
                            CoordinateNumber::try_from(1.25).unwrap(),
                            CoordinateNumber::try_from(3.25).unwrap(),
                            cf
                        ),
                        Some(CoordinateOffset::new(CoordinateNumber::try_from(0.25).unwrap(), 0, cf))
                    )
                )
            )
        ),
        Command::FunctionCode(
            FunctionCode::GCode(GCode::InterpolationMode(InterpolationMode::Linear))
        ),
        Command::FunctionCode(
            FunctionCode::DCode(
                DCode::Operation(
                    Operation::Interpolate(Coordinates::at_x(3, cf), None)
                )
            )
        ),
        Command::FunctionCode(
            FunctionCode::GCode(GCode::QuadrantMode(QuadrantMode::Multi))
        ),
        Command::FunctionCode(
            FunctionCode::GCode(GCode::InterpolationMode(InterpolationMode::ClockwiseCircular))
        ),
        Command::FunctionCode(
            FunctionCode::DCode(
                DCode::Operation(
                    Operation::Interpolate(
                        Coordinates::new(3, CoordinateNumber::try_from(2.5).unwrap(), cf),
                        Some(CoordinateOffset::new(0, CoordinateNumber::try_from(0.375).unwrap(), cf))
                    )
                )
            )
        ),
        Command::FunctionCode(
            FunctionCode::GCode(GCode::InterpolationMode(InterpolationMode::Linear))
        ),
        Command::FunctionCode(
            FunctionCode::DCode(
                DCode::Operation(
                    Operation::Interpolate(Coordinates::at_x(1, cf), None)
                )
            )
        ),
        Command::FunctionCode(
            FunctionCode::GCode(GCode::RegionMode(false))
        ),
        Command::ExtendedCode(
            ExtendedCode::LoadPolarity(Polarity::Dark)
        ),
        Command::FunctionCode(
            FunctionCode::DCode(DCode::SelectAperture(10))
        ),
        Command::FunctionCode(
            FunctionCode::DCode(
                DCode::Operation(
                    Operation::Move(
                        Coordinates::new(
                            CoordinateNumber::try_from(1.5).unwrap(),
                            CoordinateNumber::try_from(2.875).unwrap(),
                            cf
                        )
                    )
                )
            )
        ),
        Command::FunctionCode(
            FunctionCode::DCode(
                DCode::Operation(
                    Operation::Interpolate(Coordinates::at_x(2, cf), None)
                )
            )
        ),
        Command::FunctionCode(
            FunctionCode::DCode(DCode::SelectAperture(11))
        ),
        Command::FunctionCode(
            FunctionCode::DCode(
                DCode::Operation(
                    Operation::Flash(
                        Coordinates::new(
                            CoordinateNumber::try_from(1.5).unwrap(),
                            CoordinateNumber::try_from(2.875).unwrap(),
                            cf
                        )
                    )
                )
            )
        ),
        Command::FunctionCode(
            FunctionCode::DCode(
                DCode::Operation(
                    Operation::Flash(Coordinates::at_x(2, cf))
                )
            )
        ),
        Command::FunctionCode(
            FunctionCode::DCode(DCode::SelectAperture(19))
        ),
        Command::FunctionCode(
            FunctionCode::DCode(
                DCode::Operation(
                    Operation::Flash(
                        Coordinates::new(
                            CoordinateNumber::try_from(2.875).unwrap(),
                            CoordinateNumber::try_from(2.875).unwrap(),
                            cf
                        )
                    )
                )
            )
        ),
        Command::ExtendedCode(
            ExtendedCode::FileAttribute(
                FileAttribute::Md5("6ab9e892830469cdff7e3e346331d404".to_string())
            )
        ),
        Command::FunctionCode(
            FunctionCode::MCode(MCode::EndOfFile)
        ),
    ];
    println!("{}", commands.to_code().unwrap());
}
