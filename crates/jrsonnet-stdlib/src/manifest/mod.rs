mod ini;
mod python;
mod toml;
mod xml;
mod yaml;

pub use ini::IniFormat;
use jrsonnet_evaluator::{
	IStr, ObjValue, Result, Val,
	function::builtin,
	manifest::{JsonFormat, YamlStreamFormat, escape_string_json},
};
pub use python::{PythonFormat, PythonVarsFormat};
pub use toml::TomlFormat;
pub use xml::XmlJsonmlFormat;
pub use yaml::YamlFormat;

#[builtin]
pub fn builtin_escape_string_json(str_: IStr) -> Result<String> {
	Ok(escape_string_json(&str_))
}

#[builtin]
pub fn builtin_escape_string_python(str: IStr) -> Result<String> {
	Ok(escape_string_json(&str))
}

#[builtin]
pub fn builtin_manifest_json_ex(
	value: Val,
	indent: String,
	newline: Option<IStr>,
	key_val_sep: Option<IStr>,

	#[default(false)]
	preserve_order: bool,
) -> Result<String> {
	let newline = newline.as_deref().unwrap_or("\n");
	let key_val_sep = key_val_sep.as_deref().unwrap_or(": ");
	value.manifest(JsonFormat::std_to_json(
		indent,
		newline,
		key_val_sep,
		preserve_order,
	))
}

#[builtin]
pub fn builtin_manifest_json(
	value: Val,

	#[default(false)]
	preserve_order: bool,
) -> Result<String> {
	builtin_manifest_json_ex(
		value,
		"    ".to_owned(),
		None,
		None,
		preserve_order,
	)
}

#[builtin]
pub fn builtin_manifest_json_minified(
	value: Val,

	#[default(false)]
	preserve_order: bool,
) -> Result<String> {
	value.manifest(JsonFormat::minify(
		preserve_order,
	))
}

#[builtin]
pub fn builtin_manifest_yaml_doc(
	value: Val,
	#[default(false)] indent_array_in_object: bool,
	#[default(true)] quote_keys: bool,

	#[default(false)]
	preserve_order: bool,
) -> Result<String> {
	value.manifest(YamlFormat::std_to_yaml(
		indent_array_in_object,
		quote_keys,
		preserve_order,
	))
}

#[builtin]
#[allow(clippy::fn_params_excessive_bools)]
pub fn builtin_manifest_yaml_stream(
	value: Val,
	#[default(false)] indent_array_in_object: bool,
	#[default(true)] c_document_end: bool,
	#[default(true)] quote_keys: bool,

	#[default(false)]
	preserve_order: bool,
) -> Result<String> {
	value.manifest(YamlStreamFormat::std_yaml_stream(
		YamlFormat::std_to_yaml(
			indent_array_in_object,
			quote_keys,
			preserve_order,
		),
		c_document_end,
	))
}

#[builtin]
pub fn builtin_manifest_toml_ex(
	value: ObjValue,
	indent: String,

	#[default(false)]
	preserve_order: bool,
) -> Result<String> {
	Val::Obj(value).manifest(TomlFormat::std_to_toml(
		indent,
		preserve_order,
	))
}

#[builtin]
pub fn builtin_manifest_toml(
	value: ObjValue,

	#[default(false)]
	preserve_order: bool,
) -> Result<String> {
	builtin_manifest_toml_ex(
		value,
		"  ".to_owned(),
		preserve_order,
	)
}

#[builtin]
pub fn builtin_to_string(a: Val) -> Result<IStr> {
	a.to_string()
}

#[builtin]
pub fn builtin_manifest_python(
	v: Val,

	#[default(false)]
	preserve_order: bool,
) -> Result<String> {
	v.manifest(PythonFormat::std(
		preserve_order,
	))
}
#[builtin]
pub fn builtin_manifest_python_vars(
	conf: Val,

	#[default(false)]
	preserve_order: bool,
) -> Result<String> {
	conf.manifest(PythonVarsFormat::std(
		preserve_order,
	))
}

#[builtin]
pub fn builtin_escape_string_xml(str_: String) -> String {
	xml::escape_string_xml(str_.as_str())
}

#[builtin]
pub fn builtin_manifest_xml_jsonml(value: Val) -> Result<String> {
	value.manifest(XmlJsonmlFormat::std_to_xml())
}

#[builtin]
pub fn builtin_manifest_ini(
	ini: Val,

	#[default(false)]
	preserve_order: bool,
) -> Result<String> {
	ini.manifest(IniFormat::std(
		preserve_order,
	))
}
