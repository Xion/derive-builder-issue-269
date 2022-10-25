use derive_builder::Builder;


#[derive(Builder, Clone, Debug, PartialEq)]
pub struct Structure {
    #[builder(default = "DEFAULT_SIZE", field(type = "f32"))]
    size: f32,
}

const DEFAULT_SIZE: f32 = 42.0;


fn main() {
    let structure = StructureBuilder::default().build().unwrap();
    assert_eq!(DEFAULT_SIZE, structure.size);
}
