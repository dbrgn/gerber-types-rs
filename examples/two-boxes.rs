//! Example from spec chapter 2.12.1

extern crate gerber_types;

use std::io::{stdout, Write};

use gerber_types::{Command};
use gerber_types::{ExtendedCode, Unit, FileAttribute, GenerationSoftware, Part, Polarity,
                   ApertureDefinition, Aperture, Circle, CoordinateFormat};
use gerber_types::{FunctionCode};
use gerber_types::{DCode, Operation, Coordinates};
use gerber_types::{GCode, InterpolationMode};
use gerber_types::{MCode};
use gerber_types::GerberCode;

const VERSION: &'static str = env!("CARGO_PKG_VERSION");

fn main() {
    let cf = CoordinateFormat::new(2, 5);
    let commands: Vec<Command> = vec![
        Command::FunctionCode(
            FunctionCode::GCode(
                GCode::Comment("Ucamco ex. 1: Two square boxes".to_string())
            )
        ),
        Command::ExtendedCode(ExtendedCode::CoordinateFormat(cf)),
        Command::ExtendedCode(ExtendedCode::Unit(Unit::Millimeters)),
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
        Command::ExtendedCode(
            ExtendedCode::ApertureDefinition(
                ApertureDefinition {
                    code: 10,
                    aperture: Aperture::Circle(Circle { diameter: 0.01, hole_diameter: None }),
                }
            )
        ),
        Command::FunctionCode(
            FunctionCode::DCode(
                DCode::SelectAperture(10)
            )
        ),
        Command::FunctionCode(
            FunctionCode::DCode(
                DCode::Operation(
                    Operation::Move(Coordinates::new(0, 0, cf))
                )
            )
        ),
        Command::FunctionCode(
            FunctionCode::GCode(
                GCode::InterpolationMode(InterpolationMode::Linear)
            )
        ),
        // TODO: The interpolate representation needs to take the coordinate
        // format into account!
        Command::FunctionCode(
            FunctionCode::DCode(
                DCode::Operation(
                    Operation::Interpolate(Coordinates::new(5, 0, cf), None)
                )
            )
        ),
        Command::FunctionCode(
            FunctionCode::DCode(
                DCode::Operation(
                    Operation::Interpolate(Coordinates::at_y(5, cf), None)
                )
            )
        ),
        Command::FunctionCode(
            FunctionCode::DCode(
                DCode::Operation(
                    Operation::Interpolate(Coordinates::at_x(0, cf), None)
                )
            )
        ),
        Command::FunctionCode(
            FunctionCode::DCode(
                DCode::Operation(
                    Operation::Interpolate(Coordinates::at_y(0, cf), None)
                )
            )
        ),
        Command::FunctionCode(
            FunctionCode::DCode(
                DCode::Operation(
                    Operation::Move(Coordinates::at_x(6, cf))
                )
            )
        ),
        Command::FunctionCode(
            FunctionCode::DCode(
                DCode::Operation(
                    Operation::Interpolate(Coordinates::at_x(11, cf), None)
                )
            )
        ),
        Command::FunctionCode(
            FunctionCode::DCode(
                DCode::Operation(
                    Operation::Interpolate(Coordinates::at_y(5, cf), None)
                )
            )
        ),
        Command::FunctionCode(
            FunctionCode::DCode(
                DCode::Operation(
                    Operation::Interpolate(Coordinates::at_x(6, cf), None)
                )
            )
        ),
        Command::FunctionCode(
            FunctionCode::DCode(
                DCode::Operation(
                    Operation::Interpolate(Coordinates::at_y(0, cf), None)
                )
            )
        ),
        Command::FunctionCode(FunctionCode::MCode(MCode::EndOfFile)),
    ];
    let mut stdout = stdout();
    commands.to_code(&mut stdout).unwrap();
    writeln!(stdout).unwrap();
}
