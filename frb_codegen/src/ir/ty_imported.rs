use crate::ir::*;
use convert_case::{Case, Casing};

#[derive(Debug, Clone)]
pub struct IrTypeImported {
	pub crate_id: String,
	pub name: String
}

impl IrTypeTrait for IrTypeImported {
    fn visit_children_types<F: FnMut(&IrType) -> bool>(&self, _f: &mut F, _ir_file: &IrFile) {}

    fn safe_ident(&self) -> String {
        self.dart_api_type().to_case(Case::Snake)
    }

    fn dart_api_type(&self) -> String {
		format!("{}_{}", self.crate_id.to_lowercase(), self.name)
    }

    fn dart_wire_type(&self) -> String {
        self.rust_wire_type()
    }

    fn rust_api_type(&self) -> String {
        self.name.to_string()
    }

    fn rust_wire_type(&self) -> String {
        format!("wire_{}", self.safe_ident())
    }
}