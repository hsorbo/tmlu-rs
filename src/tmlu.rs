use std::option::Option;
use xml::{
    common::XmlVersion,
    writer::{EmitterConfig, XmlEvent},
    EventWriter,
};

#[derive(Debug, Clone)]
pub struct Style {
    pub dash_scale: String,
    pub fill_color_string: String,
    pub line_type: String,
    pub line_type_scale: String,
    pub opacity: String,
    pub size_mode: String,
    pub stroke_color_string: String,
    pub stroke_thickness: String,
}

#[derive(Debug, Clone)]
struct LayerList {
    constant: String,
    locked: String,
    name: String,
    style: Style,
    visible: String,
}

impl LayerList {
    fn new(name: &str) -> LayerList {
        LayerList {
            constant: "true".to_string(),
            locked: "false".to_string(),
            name: name.to_string(),
            style: Style::default(),
            visible: "true".to_string(),
        }
    }
}

impl Default for Style {
    fn default() -> Style {
        Style {
            dash_scale: "1.0".to_string(),
            fill_color_string: "0x00000000".to_string(),
            line_type: "STANDARD".to_string(),
            line_type_scale: "1.0".to_string(),
            opacity: "100.0".to_string(),
            size_mode: "SWITCHABLE".to_string(),
            stroke_color_string: "0x000000ff".to_string(),
            stroke_thickness: "1.0".to_string(),
        }
    }
}

#[derive(Debug, Clone)]
pub struct RadiusVector {
    pub angle: String,
    pub length: String,
    pub tension_corridor: String,
    pub tension_profile: String,
}

impl RadiusVector {
    pub fn default(angle: String) -> RadiusVector {
        RadiusVector {
            angle: angle,
            length: "0.0".to_string(),
            tension_corridor: "1.0".to_string(),
            tension_profile: "1.0".to_string(),
        }
    }
}
#[derive(Debug, Clone)]
pub struct Shape {
    pub has_profile_azimut: String,
    pub has_profile_tilt: String,
    pub profile_azimut: String,
    pub profile_tilt: String,
    pub radius_collection: Vec<RadiusVector>,
}
impl Default for Shape {
    fn default() -> Shape {
        Shape {
            has_profile_azimut: "false".to_string(),
            has_profile_tilt: "false".to_string(),
            profile_azimut: "0.0".to_string(),
            profile_tilt: "0.0".to_string(),
            radius_collection: vec![
                RadiusVector::default("0.0".to_string()),
                RadiusVector::default("180.0".to_string()),
                RadiusVector::default("90.0".to_string()),
                RadiusVector::default("270.0".to_string()),
            ],
        }
    }
}
#[derive(Debug, Clone)]
pub struct SurveyData {
    pub id: i32,
    pub azimuth: String,
    pub closure_to_id: i32,
    pub color: String,
    pub comment: Option<String>,
    pub date: String,
    pub depth: String,
    pub depth_in: String,
    pub down: String,
    pub excluded: String,
    pub explorer: Option<String>,
    pub from_id: i32,
    pub inclination: String,
    pub latitude: String,
    pub left: String,
    pub length: String,
    pub locked: String,
    pub longitude: String,
    pub name: Option<String>,
    pub profile_type: String,
    pub right: String,
    pub section: Option<String>,
    pub shape: Shape,
    pub station_type: String,
    pub up: String,
}
impl Default for SurveyData {
    fn default() -> Self {
        SurveyData {
            id: 0,
            azimuth: "0.0".to_string(),
            closure_to_id: 0,
            color: "0".to_string(),
            comment: None,
            date: "2021-01-01".to_string(),
            depth: "0.0".to_string(),
            depth_in: "0.0".to_string(),
            down: "0.0".to_string(),
            excluded: "false".to_string(),
            explorer: None,
            from_id: 0,
            inclination: "0.0".to_string(),
            latitude: "0.0".to_string(),
            left: "0.0".to_string(),
            length: "0.0".to_string(),
            locked: "false".to_string(),
            longitude: "0.0".to_string(),
            name: None,
            profile_type: "0".to_string(),
            right: "0.0".to_string(),
            section: None,
            shape: Shape::default(),
            station_type: "0".to_string(),
            up: "0.0".to_string(),
        }
    }
}

impl SurveyData {
    fn update(&mut self, tag: &Vec<u8>, e: &quick_xml::events::BytesText) {
        let val = e.unescape().unwrap().into_owned();
        match tag.as_slice() {
            b"ID" => self.id = val.parse::<i32>().ok().unwrap(),
            b"AZ" => self.azimuth = val,
            b"CID" => self.closure_to_id = val.parse::<i32>().ok().unwrap(),
            b"CL" => self.color = val,
            b"CM" => self.comment = Some(val),
            b"DT" => self.date = val,
            b"DP" => self.depth = val,
            b"DPI" => self.depth_in = val,
            b"D" => self.down = val,
            b"EXC" => self.excluded = val,
            b"EX" => self.explorer = Some(val),
            b"FRID" => self.from_id = val.parse::<i32>().ok().unwrap(),
            b"INC" => self.inclination = val,
            b"LT" => self.latitude = val,
            b"L" => self.left = val,
            b"LG" => self.length = val,
            b"LK" => self.locked = val,
            b"LGT" => self.longitude = val,
            b"NM" => self.name = Some(val),
            b"PRTY" => self.profile_type = val,
            b"R" => self.right = val,
            b"SC" => self.section = Some(val),
            b"TY" => self.station_type = val,
            b"U" => self.up = val,
            _ => (),
        }
    }
}

#[derive(Debug, Clone)]
pub struct CaveFileInfo {
    pub cave_name: String,
    pub first_start_absolute_elevation: String,
    pub geo_coding: String,
    //list_annotation: String,
    //data: String,
    pub unit: String,
    pub use_magnetic_azimuth: String,
    //constraints: String,
    //carto_line: String,
    // carto_page: String,
    // carto_rectangle: String,
    // carto_selection: String,
    // carto_ellipse: String,
    // carto_spline: String,
    // layers: String,
    // carto_overlay: String,
    // carto_linked_surface: String,
}

impl Default for CaveFileInfo {
    fn default() -> CaveFileInfo {
        CaveFileInfo {
            cave_name: "".to_string(),
            first_start_absolute_elevation: "0.0".to_string(),
            geo_coding: "".to_string(),
            //list_annotation: "".to_string(),
            //data: "".to_string(),
            unit: "m".to_string(),
            use_magnetic_azimuth: "true".to_string(),
            //constraints: "".to_string(),
            //carto_line: "".to_string(),
            // carto_page: "".to_string(),
            // carto_rectangle: "".to_string(),
            // carto_selection: "".to_string(),
            // carto_ellipse: "".to_string(),
            // carto_spline: "".to_string(),
            // layers: "".to_string(),
            // carto_overlay: "".to_string(),
            // carto_linked_surface: "".to_string(),
        }
    }
}
fn opt_str(input: Option<String>) -> String {
    match input {
        Some(s) => s,
        None => String::new(),
    }
}

fn write_element<W: std::io::Write>(
    writer: &mut EventWriter<W>,
    name: &str,
    val: &str,
) -> std::result::Result<(), xml::writer::Error> {
    writer.write(XmlEvent::start_element(name))?;
    let a = xml::escape::escape_str_pcdata(val);
    if val.contains('>') {
        writer.write(XmlEvent::characters(&a.replace('>', "&gt;")))?;
    } else {
        writer.write(XmlEvent::characters(&a))?;
    }
    writer.write(XmlEvent::end_element())?;
    Ok(())
}
fn write_element_fast<W: std::io::Write>(
    writer: &mut EventWriter<W>,
    name: &str,
    val: &str,
) -> std::result::Result<(), xml::writer::Error> {
    writer.write(XmlEvent::start_element(name))?;
    writer.write(XmlEvent::characters(&val))?;
    writer.write(XmlEvent::end_element())?;
    Ok(())
}

pub fn write_cavefile<W: std::io::Write, I: IntoIterator<Item = SurveyData>>(
    output: W,
    survey_data: I,
    info: CaveFileInfo,
) -> std::result::Result<(), xml::writer::Error> {
    let mut config = EmitterConfig::new()
        .perform_indent(true)
        .indent_string("")
        .pad_self_closing(false);
    config.perform_escaping = false;

    let mut writer = config.create_writer(output);

    let _ = writer.write(XmlEvent::StartDocument {
        version: XmlVersion::Version10,
        encoding: Some("UTF-8"),
        standalone: Some(true),
    });

    writer.write(XmlEvent::start_element("CaveFile"))?;
    write_element(&mut writer, "caveName", &info.cave_name)?;
    write_element(
        &mut writer,
        "firstStartAbsoluteElevation",
        &info.first_start_absolute_elevation,
    )?;
    write_element(&mut writer, "geoCoding", &info.geo_coding)?;
    writer.write(XmlEvent::start_element("ListAnnotation"))?;
    writer.write(XmlEvent::end_element())?;
    writer.write(XmlEvent::start_element("Data"))?;
    for srvd in survey_data {
        writer.write(XmlEvent::start_element("SRVD"))?;
        write_element_fast(&mut writer, "AZ", &srvd.azimuth)?;
        write_element_fast(&mut writer, "CID", &srvd.closure_to_id.to_string())?;
        write_element_fast(&mut writer, "CL", &srvd.color)?;
        write_element(&mut writer, "CM", opt_str(srvd.comment).as_str())?;
        write_element_fast(&mut writer, "DT", &srvd.date)?;
        write_element_fast(&mut writer, "DP", &srvd.depth)?;
        write_element_fast(&mut writer, "DPI", &srvd.depth_in)?;
        write_element(&mut writer, "D", &srvd.down)?;
        write_element(&mut writer, "EXC", &srvd.excluded)?;
        write_element(&mut writer, "EX", opt_str(srvd.explorer).as_str())?;
        write_element_fast(&mut writer, "FRID", &srvd.from_id.to_string())?;
        write_element_fast(&mut writer, "ID", &srvd.id.to_string())?;
        write_element_fast(&mut writer, "INC", &srvd.inclination)?;
        write_element_fast(&mut writer, "LT", &srvd.latitude)?;
        write_element_fast(&mut writer, "L", &srvd.left)?;
        write_element_fast(&mut writer, "LG", &srvd.length)?;
        write_element_fast(&mut writer, "LK", &srvd.locked)?;
        write_element_fast(&mut writer, "LGT", &srvd.longitude)?;
        write_element(&mut writer, "NM", opt_str(srvd.name).as_str())?;
        write_element_fast(&mut writer, "PRTY", &srvd.profile_type)?;
        write_element_fast(&mut writer, "R", &srvd.right)?;
        write_element(&mut writer, "SC", opt_str(srvd.section).as_str())?;
        writer.write(XmlEvent::start_element("SH"))?;
        write_element_fast(&mut writer, "HPRA", &srvd.shape.has_profile_azimut)?;
        write_element_fast(&mut writer, "HPRT", &srvd.shape.has_profile_tilt)?;
        write_element_fast(&mut writer, "PRAZ", &srvd.shape.profile_azimut)?;
        write_element_fast(&mut writer, "PRT", &srvd.shape.profile_tilt)?;
        writer.write(XmlEvent::start_element("RC"))?;
        for rv in &srvd.shape.radius_collection {
            writer.write(XmlEvent::start_element("RV"))?;
            write_element_fast(&mut writer, "ag", &rv.angle)?;
            write_element_fast(&mut writer, "lg", &rv.length)?;
            write_element_fast(&mut writer, "tc", &rv.tension_corridor)?;
            write_element_fast(&mut writer, "tp", &rv.tension_profile)?;
            writer.write(XmlEvent::end_element())?;
        }
        writer.write(XmlEvent::end_element())?;
        writer.write(XmlEvent::end_element())?;
        write_element_fast(&mut writer, "TY", &srvd.station_type)?;
        write_element_fast(&mut writer, "U", &srvd.up)?;
        writer.write(XmlEvent::end_element())?;
    }
    writer.write(XmlEvent::end_element())?;
    write_element(&mut writer, "unit", &info.unit)?;
    write_element(
        &mut writer,
        "useMagneticAzimuth",
        &info.use_magnetic_azimuth,
    )?;
    writer.write(XmlEvent::start_element("Constraints"))?;
    writer.write(XmlEvent::end_element())?;
    writer.write(XmlEvent::start_element("CartoLine"))?;
    writer.write(XmlEvent::end_element())?;
    writer.write(XmlEvent::start_element("CartoPage"))?;
    writer.write(XmlEvent::end_element())?;
    writer.write(XmlEvent::start_element("CartoRectangle"))?;
    writer.write(XmlEvent::end_element())?;
    writer.write(XmlEvent::start_element("CartoSelection"))?;
    writer.write(XmlEvent::end_element())?;
    writer.write(XmlEvent::start_element("CartoEllipse"))?;
    writer.write(XmlEvent::end_element())?;
    writer.write(XmlEvent::start_element("CartoSpline"))?;
    writer.write(XmlEvent::end_element())?;
    writer.write(XmlEvent::start_element("Layers"))?;
    for layer in [LayerList::new("Overlay"), LayerList::new("Default")].iter() {
        writer.write(XmlEvent::start_element("layerList"))?;
        write_element_fast(&mut writer, "constant", &layer.constant)?;
        write_element_fast(&mut writer, "locked", &layer.locked)?;
        write_element_fast(&mut writer, "name", &layer.name)?;
        let style = &layer.style;
        writer.write(XmlEvent::start_element("style"))?;
        write_element_fast(&mut writer, "dashScale", &style.dash_scale)?;
        write_element_fast(&mut writer, "fillColorString", &style.fill_color_string)?;
        write_element_fast(&mut writer, "lineType", &style.line_type)?;
        write_element_fast(&mut writer, "lineTypeScale", &style.line_type_scale)?;
        write_element_fast(&mut writer, "opacity", &style.opacity)?;
        write_element_fast(&mut writer, "sizeMode", &style.size_mode)?;
        write_element_fast(&mut writer, "strokeColorString", &style.stroke_color_string)?;
        write_element_fast(&mut writer, "strokeThickness", &style.stroke_thickness)?;
        writer.write(XmlEvent::end_element())?;
        write_element_fast(&mut writer, "visible", &layer.visible)?;
        writer.write(XmlEvent::end_element())?;
    }
    writer.write(XmlEvent::end_element())?;
    writer.write(XmlEvent::start_element("CartoOverlay"))?;
    writer.write(XmlEvent::end_element())?;
    writer.write(XmlEvent::start_element("CartoLinkedSurface"))?;
    writer.write(XmlEvent::end_element())?;
    writer.write(XmlEvent::end_element())?;
    Ok(())
}

//https://docs.rs/quick-xml/latest/quick_xml/de/fn.from_reader.html#
#[derive(Debug)]
pub struct CaveFile {
    pub info: CaveFileInfo,
    pub data: Vec<SurveyData>,
}

pub fn read_cavefile<R: std::io::BufRead>(input: R) -> CaveFile {
    use quick_xml::events::Event;
    use quick_xml::reader::Reader;
    let mut reader = Reader::from_reader(input);
    reader.trim_text(true);

    let mut buf = Vec::new();
    let mut is_srvd = false;
    let mut current_tag = Vec::new();

    let mut cave = CaveFile {
        data: Vec::new(),
        info: CaveFileInfo::default(),
    };

    let mut current_srvd = SurveyData::default();

    let mut path: Vec<Vec<u8>> = Vec::new();
    let mut path_string: Vec<String> = Vec::new(); //TODO: remove this if performance is an issue

    loop {
        match reader.read_event_into(&mut buf) {
            Err(e) => panic!("Error at position {}: {:?}", reader.buffer_position(), e),
            Ok(Event::Eof) => break,
            Ok(Event::Start(e)) => {
                path.push(e.name().as_ref().to_owned());
                path_string.push(String::from_utf8_lossy(e.name().as_ref()).to_string());
                //println!("path_string: {:?}\n", path_string);
                current_tag = e.name().as_ref().to_owned();
                if e.name().as_ref() == b"SRVD" {
                    is_srvd = true;
                }
            }
            Ok(Event::End(e)) => {
                path.pop();
                path_string.pop();
                if e.name().as_ref() == b"SRVD" {
                    let a = current_srvd.clone();
                    cave.data.push(a);
                    is_srvd = false;
                    //TODO: avoid clearing the whole struct
                    //There will be no text-events on empty tags..
                    current_srvd = SurveyData::default();
                }
            }
            Ok(Event::Text(e)) => {
                //println!("S {:?}", path.last());
                let k = e.unescape().unwrap().into_owned();
                if is_srvd {
                    current_srvd.update(&current_tag, &e);
                } else if current_tag == b"caveName" {
                    cave.info.cave_name = k;
                } else if current_tag == b"firstStartAbsoluteElevation" {
                    cave.info.first_start_absolute_elevation = k;
                } else if current_tag == b"geoCoding" {
                    cave.info.geo_coding = k;
                } else if current_tag == b"unit" {
                    cave.info.unit = k;
                } else if current_tag == b"useMagneticAzimuth" {
                    cave.info.use_magnetic_azimuth = k;
                }
            }
            _ => (),
        }
        buf.clear();
    }
    cave
}
