pub enum DataType {
    NoneType,
    UserDefinedType,
    PrimitiveType,
    PointerType,
    ArrayType,
}

pub struct TypeInfo {
    name: String,
    size: usize,
    fields: Vec<FieldInfo>,
}

pub struct FieldInfo {
    field_name: String,
    offset: usize,
    field_type: FieldType,
}

impl TypeInfo {
    pub fn new(name: String, size: usize, fields: Vec<FieldInfo>) -> Self {
        Self {
            name,
            size,
            fields,
        }
    }
}

impl FieldInfo {
    pub fn new(field_name: String, offset: usize, field_type: FieldType) -> Self {
        Self {
            field_name,
            offset,
            field_type,
        }
    }
}

enum FieldType {
    Array(ArrayField),
    UDT(UDTField),
}

struct ArrayField {
    type_size: usize,
    len: usize,
}

struct UDTField {
    size: usize,
}