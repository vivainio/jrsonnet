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
) -> Result<String> {
	let newline = newline.as_deref().unwrap_or("\n");
	let key_val_sep = key_val_sep.as_deref().unwrap_or(": ");
	value.manifest(JsonFormat::std_to_json(
		indent,
		newline,
		key_val_sep,
	))
}

#[builtin]
pub fn builtin_manifest_json(
	value: Val,
) -> Result<String> {
	builtin_manifest_json_ex(
		value,
		"    ".to_owned(),
		None,
		None,
	)
}

#[builtin]
pub fn builtin_manifest_json_minified(
	value: Val,
) -> Result<String> {
	value.manifest(JsonFormat::minify())
}

#[builtin]
pub fn builtin_manifest_yaml_doc(
	value: Val,
	#[default(false)] indent_array_in_object: bool,
	#[default(true)] quote_keys: bool,
) -> Result<String> {
	value.manifest(YamlFormat::std_to_yaml(
		indent_array_in_object,
		quote_keys,
	))
}

#[builtin]
#[allow(clippy::fn_params_excessive_bools)]
pub fn builtin_manifest_yaml_stream(
	value: Val,
	#[default(false)] indent_array_in_object: bool,
	#[default(true)] c_document_end: bool,
	#[default(true)] quote_keys: bool,
) -> Result<String> {
	value.manifest(YamlStreamFormat::std_yaml_stream(
		YamlFormat::std_to_yaml(
			indent_array_in_object,
			quote_keys,
		),
		c_document_end,
	))
}

#[builtin]
pub fn builtin_manifest_toml_ex(
	value: ObjValue,
	indent: String,
) -> Result<String> {
	Val::Obj(value).manifest(TomlFormat::std_to_toml(indent))
}

#[builtin]
pub fn builtin_manifest_toml(
	value: ObjValue,
) -> Result<String> {
	builtin_manifest_toml_ex(
		value,
		"  ".to_owned(),
	)
}

#[builtin]
pub fn builtin_to_string(a: Val) -> Result<IStr> {
	a.to_string()
}

#[builtin]
pub fn builtin_manifest_python(
	v: Val,
) -> Result<String> {
	v.manifest(PythonFormat::std())
}
#[builtin]
pub fn builtin_manifest_python_vars(
	conf: Val,
) -> Result<String> {
	conf.manifest(PythonVarsFormat::std())
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
) -> Result<String> {
	ini.manifest(IniFormat::std())
}
