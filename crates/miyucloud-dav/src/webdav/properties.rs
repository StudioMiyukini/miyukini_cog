use crate::common::dav_error::DavError;
use crate::webdav::response::DavProperty;

/// Standard WebDAV properties
pub fn resource_properties(
    display_name: &str,
    content_type: &str,
    content_length: u64,
    creation_date: &str,
    last_modified: &str,
    etag: &str,
    is_collection: bool,
) -> Vec<DavProperty> {
    let mut props = vec![
        DavProperty {
            namespace: "DAV:".into(),
            name: "displayname".into(),
            value: Some(display_name.into()),
        },
        DavProperty {
            namespace: "DAV:".into(),
            name: "getcontentlength".into(),
            value: Some(content_length.to_string()),
        },
        DavProperty {
            namespace: "DAV:".into(),
            name: "getcontenttype".into(),
            value: Some(content_type.into()),
        },
        DavProperty {
            namespace: "DAV:".into(),
            name: "creationdate".into(),
            value: Some(creation_date.into()),
        },
        DavProperty {
            namespace: "DAV:".into(),
            name: "getlastmodified".into(),
            value: Some(last_modified.into()),
        },
        DavProperty {
            namespace: "DAV:".into(),
            name: "getetag".into(),
            value: Some(format!("\"{etag}\"")),
        },
    ];

    if is_collection {
        props.push(DavProperty {
            namespace: "DAV:".into(),
            name: "resourcetype".into(),
            value: Some("<D:collection/>".into()),
        });
    } else {
        props.push(DavProperty {
            namespace: "DAV:".into(),
            name: "resourcetype".into(),
            value: None,
        });
    }

    props
}

/// Dead property storage (user-defined properties)
pub struct DeadPropertyStore {
    // Will be backed by SQLite in E3
}

impl DeadPropertyStore {
    pub fn new() -> Self {
        Self {}
    }

    pub fn get(&self, _href: &str, _ns: &str, _name: &str) -> Result<Option<String>, DavError> {
        Ok(None)
    }

    pub fn set(
        &self,
        _href: &str,
        _ns: &str,
        _name: &str,
        _value: &str,
    ) -> Result<(), DavError> {
        Ok(())
    }

    pub fn remove(&self, _href: &str, _ns: &str, _name: &str) -> Result<(), DavError> {
        Ok(())
    }
}
