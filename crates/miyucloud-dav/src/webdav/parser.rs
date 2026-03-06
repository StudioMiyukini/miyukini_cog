use crate::common::dav_error::DavError;
use crate::common::xml_security;

/// WebDAV request methods beyond standard HTTP
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum DavMethod {
    Propfind,
    Proppatch,
    Mkcol,
    Copy,
    Move,
    Lock,
    Unlock,
}

impl DavMethod {
    pub fn from_str(s: &str) -> Option<Self> {
        match s {
            "PROPFIND" => Some(Self::Propfind),
            "PROPPATCH" => Some(Self::Proppatch),
            "MKCOL" => Some(Self::Mkcol),
            "COPY" => Some(Self::Copy),
            "MOVE" => Some(Self::Move),
            "LOCK" => Some(Self::Lock),
            "UNLOCK" => Some(Self::Unlock),
            _ => None,
        }
    }
}

/// Depth header for PROPFIND requests
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Depth {
    Zero,
    One,
    Infinity,
}

impl Depth {
    pub fn from_header(value: Option<&str>) -> Self {
        match value {
            Some("0") => Self::Zero,
            Some("1") => Self::One,
            Some("infinity") | None => Self::Infinity,
            _ => Self::Infinity,
        }
    }
}

/// Parsed PROPFIND request body
#[derive(Debug)]
pub enum PropfindRequest {
    AllProp,
    PropName,
    Prop(Vec<PropRequest>),
}

#[derive(Debug, Clone)]
pub struct PropRequest {
    pub namespace: String,
    pub name: String,
}

/// Parse a PROPFIND XML request body
pub fn parse_propfind(body: &[u8]) -> Result<PropfindRequest, DavError> {
    if body.is_empty() {
        return Ok(PropfindRequest::AllProp);
    }

    xml_security::validate_xml(body)?;

    // Simplified parsing - full implementation in E3
    let body_str = std::str::from_utf8(body)
        .map_err(|e| DavError::XmlParse(format!("Invalid UTF-8: {e}")))?;

    if body_str.contains("allprop") {
        Ok(PropfindRequest::AllProp)
    } else if body_str.contains("propname") {
        Ok(PropfindRequest::PropName)
    } else {
        Ok(PropfindRequest::AllProp)
    }
}
