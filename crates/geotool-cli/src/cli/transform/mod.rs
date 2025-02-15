mod context;
mod options;
mod output_fn;
mod record;
use bpaf::Bpaf;
use context::ContextTransform;
pub use options::*;
use record::Record;
#[derive(Bpaf, Clone, Debug)]
pub enum TransformCommands {
    #[bpaf(command, adjacent, fallback_to_usage)]
    /// Crypto coordinates between `BD09`, `GCJ02` and `WGS84`.
    Crypto {
        #[bpaf(short, long)]
        from: CryptoSpace,
        #[bpaf(short, long)]
        to: CryptoSpace,
    },
    #[bpaf(command, adjacent, fallback_to_usage)]
    /// Converts projected XY coordinates from the height compensation plane to the sea level plane.
    DatumCompense {
        #[bpaf(long)]
        /// Elevation of the height compensation plane (in meters).
        hb: f64,
        #[bpaf(short, long)]
        /// Radius of the Earth (in meters).
        radius: f64,
        #[bpaf(long)]
        /// X coordinate system origin (in meters).
        x0: f64,
        #[bpaf(long)]
        /// Y coordinate system origin (in meters).
        y0: f64,
    },
    #[bpaf(command, adjacent, fallback_to_usage)]
    /// Converts geodetic coordinates (longitude/L, latitude/B, height/H) to Cartesian coordinates (X, Y, Z).
    Lbh2xyz {
        #[bpaf(short('a'), long)]
        /// Semimajor radius of the ellipsoid axis
        major_radius: f64,
        #[bpaf(long("invf"))]
        /// Inverse flattening of the ellipsoid.
        inverse_flattening: f64,
    },
    #[bpaf(command, adjacent, fallback_to_usage)]
    /// Normalize.
    Normalize,
    #[bpaf(command, adjacent, fallback_to_usage)]
    /// Transform coordinate from one known coordinate reference systems to another.
    ///
    ///  The `from` and `to` can be:
    ///  - an "AUTHORITY:CODE", like "EPSG:25832".
    ///  - a PROJ string, like "+proj=longlat +datum=WGS84". When using that syntax, the unit is expected to be degrees.
    ///  - the name of a CRS as found in the PROJ database, e.g "WGS84", "NAD27", etc.
    Proj {
        #[bpaf(short, long, argument("PROJ"))]
        from: String,
        #[bpaf(short, long, argument("PROJ"))]
        to: String,
    },
    #[bpaf(command, adjacent, fallback_to_usage)]
    /// Rotate Coordinate.
    Rotate {
        #[bpaf(short, long)]
        value: f64,
        #[bpaf(short, long)]
        axis: RotateAxis,
        #[bpaf(short, long)]
        unit: options::RotateUnit,
    },
    #[bpaf(command, adjacent, fallback_to_usage)]
    /// Scale Coordinate.
    Scale {
        #[bpaf(short('x'), long)]
        x_scale: f64,
        #[bpaf(short('y'), long)]
        y_scale: f64,
        #[bpaf(short('z'), long)]
        z_scale: f64,
    },
    #[bpaf(command, adjacent, fallback_to_usage)]
    /// Transforms coordinates between Cartesian, cylindrical, and spherical coordinate systems.
    Space {
        #[bpaf(short, long)]
        from: CoordSpace,
        #[bpaf(short, long)]
        to: CoordSpace,
    },
    #[bpaf(command, adjacent, fallback_to_usage)]
    /// Translate Coordinate.
    Translate {
        #[bpaf(short('x'), long)]
        x_translate: f64,
        #[bpaf(short('y'), long)]
        y_translate: f64,
        #[bpaf(short('z'), long)]
        z_translate: f64,
    },
    #[bpaf(command, adjacent, fallback_to_usage)]
    /// Converts Cartesian coordinates (X, Y, Z) to geodetic coordinates (Longitude, Latitude, Height).
    Xyz2lbh {
        #[bpaf(short('a'), long)]
        /// Semimajor radius of the ellipsoid axis
        major_radius: f64,
        #[bpaf(long("invf"))]
        /// Inverse flattening of the ellipsoid.
        inverse_flattening: f64,
    },
}
pub fn execute(
    name: &str,
    x: f64,
    y: f64,
    z: f64,
    output_format: OutputFormat,
    cmds: Vec<TransformCommands>,
) {
    let mut ctx = ContextTransform { x, y, z };
    let mut records: Vec<Record> = vec![Record {
        idx: 0,
        method: "input".to_string(),
        parameter: serde_json::json!({}),
        output_x: ctx.x,
        output_y: ctx.y,
        output_z: ctx.z,
        output_x_name: "x".to_string(),
        output_y_name: "y".to_string(),
        output_z_name: "z".to_string(),
    }];
    for (i, cmd) in cmds.iter().enumerate() {
        match cmd {
            TransformCommands::Crypto { from, to } => {
                ctx.crypto(*from, *to);
                let record = Record {
                    idx: (i + 1) as u8,
                    method: "crypto".to_string(),
                    parameter: serde_json::json!({
                        "from": from.to_string(),
                        "to": to.to_string()
                    }),
                    output_x: ctx.x,
                    output_y: ctx.y,
                    output_z: ctx.z,
                    output_x_name: "longitude".to_string(),
                    output_y_name: "latitude".to_string(),
                    output_z_name: "elevation".to_string(),
                };
                records.push(record);
            }
            TransformCommands::DatumCompense {
                hb,
                radius: r,
                x0,
                y0,
            } => {
                ctx.datum_compense(*hb, *r, *x0, *y0);
                let record = Record {
                    idx: (i + 1) as u8,
                    method: "datum compense".to_string(),
                    parameter: serde_json::json!({
                        "hb": hb,
                        "r": r,
                        "x0":x0,
                        "y0":y0
                    }),
                    output_x: ctx.x,
                    output_y: ctx.y,
                    output_z: ctx.z,
                    output_x_name: "x".to_string(),
                    output_y_name: "y".to_string(),
                    output_z_name: "elevation".to_string(),
                };
                records.push(record);
            }
            TransformCommands::Lbh2xyz {
                major_radius: semi_major_axis,
                inverse_flattening,
            } => {
                ctx.lbh2xyz(*semi_major_axis, *inverse_flattening);
                let record = Record {
                    idx: (i + 1) as u8,
                    method: "lbh2xyz".to_string(),
                    parameter: serde_json::json!({
                        "semi_major_axis": semi_major_axis,
                        "inverse_flattening": inverse_flattening
                    }),
                    output_x: ctx.x,
                    output_y: ctx.y,
                    output_z: ctx.z,
                    output_x_name: "x".to_string(),
                    output_y_name: "y".to_string(),
                    output_z_name: "z".to_string(),
                };
                records.push(record);
            }
            TransformCommands::Normalize => {
                ctx.normalize();
                let record = Record {
                    idx: (i + 1) as u8,
                    method: "normalize".to_string(),
                    parameter: serde_json::json!({}),
                    output_x: ctx.x,
                    output_y: ctx.y,
                    output_z: ctx.z,
                    output_x_name: "x".to_string(),
                    output_y_name: "y".to_string(),
                    output_z_name: "z".to_string(),
                };
                records.push(record);
            }
            TransformCommands::Proj { from, to } => {
                tracing::warn!("Proj currently support 2D convert only.");
                ctx.proj(from.as_str(), to.as_str()).unwrap();
                let record = Record {
                    idx: (i + 1) as u8,
                    method: "proj".to_string(),
                    parameter: serde_json::json!({
                        "from": from.to_string(),
                        "to": to.to_string()
                    }),
                    output_x: ctx.x,
                    output_y: ctx.y,
                    output_z: ctx.z,
                    output_x_name: "x".to_string(),
                    output_y_name: "y".to_string(),
                    output_z_name: "z".to_string(),
                };
                records.push(record);
            }
            TransformCommands::Rotate { value, axis, unit } => {
                ctx.rotate(*value, *axis, *unit);
                let record = Record {
                    idx: (i + 1) as u8,
                    method: "rotate".to_string(),
                    parameter: serde_json::json!({
                        "value": value,
                        "axis": axis.to_string(),
                        "unit": unit.to_string()
                    }),
                    output_x: ctx.x,
                    output_y: ctx.y,
                    output_z: ctx.z,
                    output_x_name: "x".to_string(),
                    output_y_name: "y".to_string(),
                    output_z_name: "z".to_string(),
                };
                records.push(record);
            }
            TransformCommands::Scale {
                x_scale,
                y_scale,
                z_scale,
            } => {
                ctx.scale(*x_scale, *y_scale, *z_scale);
                let record = Record {
                    idx: (i + 1) as u8,
                    method: "scale".to_string(),
                    parameter: serde_json::json!({
                        "x_scale": x_scale,
                        "y_scale": y_scale,
                        "z_scale": z_scale
                    }),
                    output_x: ctx.x,
                    output_y: ctx.y,
                    output_z: ctx.z,
                    output_x_name: "x".to_string(),
                    output_y_name: "y".to_string(),
                    output_z_name: "z".to_string(),
                };
                records.push(record);
            }
            TransformCommands::Space { from, to } => {
                ctx.space(*from, *to);
                let record = Record {
                    idx: (i + 1) as u8,
                    method: "space".to_string(),
                    parameter: serde_json::json!({
                        "from": from.to_string(),
                        "to": to.to_string()
                    }),
                    output_x: ctx.x,
                    output_y: ctx.y,
                    output_z: ctx.z,
                    output_x_name: "longitude".to_string(),
                    output_y_name: "latitude".to_string(),
                    output_z_name: "elevation".to_string(),
                };
                records.push(record);
            }
            TransformCommands::Translate {
                x_translate,
                y_translate,
                z_translate,
            } => {
                ctx.translate(*x_translate, *y_translate, *z_translate);
                let record = Record {
                    idx: (i + 1) as u8,
                    method: "scale".to_string(),
                    parameter: serde_json::json!({
                        "x_translate": x_translate,
                        "y_translate": y_translate,
                        "z_translate": z_translate
                    }),
                    output_x: ctx.x,
                    output_y: ctx.y,
                    output_z: ctx.z,
                    output_x_name: "x".to_string(),
                    output_y_name: "y".to_string(),
                    output_z_name: "z".to_string(),
                };
                records.push(record);
            }
            TransformCommands::Xyz2lbh {
                major_radius: semi_major_axis,
                inverse_flattening,
            } => {
                ctx.xyz2lbh(*semi_major_axis, *inverse_flattening);
                let record = Record {
                    idx: (i + 1) as u8,
                    method: "xyz2lbh".to_string(),
                    parameter: serde_json::json!({
                        "semi_major_axis": semi_major_axis,
                        "inverse_flattening": inverse_flattening
                    }),
                    output_x: ctx.x,
                    output_y: ctx.y,
                    output_z: ctx.z,
                    output_x_name: "longitude".to_string(),
                    output_y_name: "latitude".to_string(),
                    output_z_name: "elevation".to_string(),
                };
                records.push(record);
            }
        }
    }
    // output
    match output_format {
        OutputFormat::Simple => output_fn::output_simple(records.last().unwrap()),
        OutputFormat::Plain => output_fn::output_plain(name, &records),
        OutputFormat::Json => output_fn::output_json(name, &records),
    }
}
