use crate::prelude::*;
use crate::editor::annotatedstring::AnnotationType;

// clippy::struct_field_names: naming the field `type` is disallowed due to type being a keyword.
#[derive(Copy, Clone, Debug)]
#[allow(clippy::struct_field_names)]
pub struct Annotation {
    pub annotation_type: AnnotationType,
    pub start: ByteIdx,
    pub end: ByteIdx,
}