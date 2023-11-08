use tera::{Tera, Context};

use crate::manifest::ComponentDefinition;
use include_dir::{include_dir, Dir};

static TEMPLATE_DIR: Dir<'_> = include_dir!("$CARGO_MANIFEST_DIR/templates/code_serialization");
static MANIFEST_CODE_SERIALIZATION_TEMPLATE: &str = "manifest-code-serialization.tera";
static MACROS_TEMPLATE: &str = "macros.tera";

pub fn press_code_serialization_template(
    args: ComponentDefinition,
) -> String {
    let mut tera = Tera::default();

    tera.add_raw_template(MACROS_TEMPLATE, TEMPLATE_DIR
        .get_file(MACROS_TEMPLATE)
        .unwrap()
        .contents_utf8()
        .unwrap())
        .expect("Failed to add macros.tera");

    
    tera.add_raw_template(MANIFEST_CODE_SERIALIZATION_TEMPLATE, TEMPLATE_DIR
        .get_file(MANIFEST_CODE_SERIALIZATION_TEMPLATE)
        .unwrap()
        .contents_utf8()
        .unwrap())
        .expect("Failed to add manifest-code-serialization.tera");

    let context = Context::from_serialize(args).unwrap();

    tera.render(MANIFEST_CODE_SERIALIZATION_TEMPLATE, &context)
        .expect("Failed to render template")
}
