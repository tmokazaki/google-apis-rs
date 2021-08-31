// DO NOT EDIT !
// This file was generated automatically from 'src/mako/cli/main.rs.mako'
// DO NOT EDIT !
#![allow(unused_variables, unused_imports, dead_code, unused_mut)]

extern crate tokio;

#[macro_use]
extern crate clap;
extern crate yup_oauth2 as oauth2;

use std::env;
use std::io::{self, Write};
use clap::{App, SubCommand, Arg};

use google_containeranalysis1_beta1::{api, Error};

mod client;

use client::{InvalidOptionsError, CLIError, arg_from_str, writer_from_opts, parse_kv_arg,
          input_file_from_opts, input_mime_from_opts, FieldCursor, FieldError, CallType, UploadProtocol,
          calltype_from_str, remove_json_null_values, ComplexType, JsonType, JsonTypeInfo};

use std::default::Default;
use std::str::FromStr;

use serde_json as json;
use clap::ArgMatches;

enum DoitError {
    IoError(String, io::Error),
    ApiError(Error),
}

struct Engine<'n> {
    opt: ArgMatches<'n>,
    hub: api::ContainerAnalysis,
    gp: Vec<&'static str>,
    gpm: Vec<(&'static str, &'static str)>,
}


impl<'n> Engine<'n> {
    async fn _projects_notes_batch_create(&self, opt: &ArgMatches<'n>, dry_run: bool, err: &mut InvalidOptionsError)
                                                    -> Result<(), DoitError> {
        
        let mut field_cursor = FieldCursor::default();
        let mut object = json::value::Value::Object(Default::default());
        
        for kvarg in opt.values_of("kv").map(|i|i.collect()).unwrap_or(Vec::new()).iter() {
            let last_errc = err.issues.len();
            let (key, value) = parse_kv_arg(&*kvarg, err, false);
            let mut temp_cursor = field_cursor.clone();
            if let Err(field_err) = temp_cursor.set(&*key) {
                err.issues.push(field_err);
            }
            if value.is_none() {
                field_cursor = temp_cursor.clone();
                if err.issues.len() > last_errc {
                    err.issues.remove(last_errc);
                }
                continue;
            }
        
            let type_info: Option<(&'static str, JsonTypeInfo)> =
                match &temp_cursor.to_string()[..] {
                    _ => {
                        let suggestion = FieldCursor::did_you_mean(key, &vec![]);
                        err.issues.push(CLIError::Field(FieldError::Unknown(temp_cursor.to_string(), suggestion, value.map(|v| v.to_string()))));
                        None
                    }
                };
            if let Some((field_cursor_str, type_info)) = type_info {
                FieldCursor::from(field_cursor_str).set_json_value(&mut object, value.unwrap(), type_info, err, &temp_cursor);
            }
        }
        let mut request: api::BatchCreateNotesRequest = json::value::from_value(object).unwrap();
        let mut call = self.hub.projects().notes_batch_create(request, opt.value_of("parent").unwrap_or(""));
        for parg in opt.values_of("v").map(|i|i.collect()).unwrap_or(Vec::new()).iter() {
            let (key, value) = parse_kv_arg(&*parg, err, false);
            match key {
                _ => {
                    let mut found = false;
                    for param in &self.gp {
                        if key == *param {
                            found = true;
                            call = call.param(self.gpm.iter().find(|t| t.0 == key).unwrap_or(&("", key)).1, value.unwrap_or("unset"));
                            break;
                        }
                    }
                    if !found {
                        err.issues.push(CLIError::UnknownParameter(key.to_string(),
                                                                  {let mut v = Vec::new();
                                                                           v.extend(self.gp.iter().map(|v|*v));
                                                                           v } ));
                    }
                }
            }
        }
        let protocol = CallType::Standard;
        if dry_run {
            Ok(())
        } else {
            assert!(err.issues.len() == 0);
            for scope in self.opt.values_of("url").map(|i|i.collect()).unwrap_or(Vec::new()).iter() {
                call = call.add_scope(scope);
            }
            let mut ostream = match writer_from_opts(opt.value_of("out")) {
                Ok(mut f) => f,
                Err(io_err) => return Err(DoitError::IoError(opt.value_of("out").unwrap_or("-").to_string(), io_err)),
            };
            match match protocol {
                CallType::Standard => call.doit().await,
                _ => unreachable!()
            } {
                Err(api_err) => Err(DoitError::ApiError(api_err)),
                Ok((mut response, output_schema)) => {
                    let mut value = json::value::to_value(&output_schema).expect("serde to work");
                    remove_json_null_values(&mut value);
                    json::to_writer_pretty(&mut ostream, &value).unwrap();
                    ostream.flush().unwrap();
                    Ok(())
                }
            }
        }
    }

    async fn _projects_notes_create(&self, opt: &ArgMatches<'n>, dry_run: bool, err: &mut InvalidOptionsError)
                                                    -> Result<(), DoitError> {
        
        let mut field_cursor = FieldCursor::default();
        let mut object = json::value::Value::Object(Default::default());
        
        for kvarg in opt.values_of("kv").map(|i|i.collect()).unwrap_or(Vec::new()).iter() {
            let last_errc = err.issues.len();
            let (key, value) = parse_kv_arg(&*kvarg, err, false);
            let mut temp_cursor = field_cursor.clone();
            if let Err(field_err) = temp_cursor.set(&*key) {
                err.issues.push(field_err);
            }
            if value.is_none() {
                field_cursor = temp_cursor.clone();
                if err.issues.len() > last_errc {
                    err.issues.remove(last_errc);
                }
                continue;
            }
        
            let type_info: Option<(&'static str, JsonTypeInfo)> =
                match &temp_cursor.to_string()[..] {
                    "attestation-authority.hint.human-readable-name" => Some(("attestationAuthority.hint.humanReadableName", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "base-image.fingerprint.v1-name" => Some(("baseImage.fingerprint.v1Name", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "base-image.fingerprint.v2-blob" => Some(("baseImage.fingerprint.v2Blob", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Vec })),
                    "base-image.fingerprint.v2-name" => Some(("baseImage.fingerprint.v2Name", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "base-image.resource-url" => Some(("baseImage.resourceUrl", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "build.builder-version" => Some(("build.builderVersion", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "build.signature.key-id" => Some(("build.signature.keyId", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "build.signature.key-type" => Some(("build.signature.keyType", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "build.signature.public-key" => Some(("build.signature.publicKey", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "build.signature.signature" => Some(("build.signature.signature", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "create-time" => Some(("createTime", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "deployable.resource-uri" => Some(("deployable.resourceUri", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Vec })),
                    "discovery.analysis-kind" => Some(("discovery.analysisKind", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "expiration-time" => Some(("expirationTime", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "intoto.expected-command" => Some(("intoto.expectedCommand", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Vec })),
                    "intoto.step-name" => Some(("intoto.stepName", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "intoto.threshold" => Some(("intoto.threshold", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "kind" => Some(("kind", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "long-description" => Some(("longDescription", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "name" => Some(("name", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "package.name" => Some(("package.name", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "related-note-names" => Some(("relatedNoteNames", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Vec })),
                    "short-description" => Some(("shortDescription", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "update-time" => Some(("updateTime", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "vulnerability.cvss-score" => Some(("vulnerability.cvssScore", JsonTypeInfo { jtype: JsonType::Float, ctype: ComplexType::Pod })),
                    "vulnerability.cvss-v3.attack-complexity" => Some(("vulnerability.cvssV3.attackComplexity", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "vulnerability.cvss-v3.attack-vector" => Some(("vulnerability.cvssV3.attackVector", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "vulnerability.cvss-v3.availability-impact" => Some(("vulnerability.cvssV3.availabilityImpact", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "vulnerability.cvss-v3.base-score" => Some(("vulnerability.cvssV3.baseScore", JsonTypeInfo { jtype: JsonType::Float, ctype: ComplexType::Pod })),
                    "vulnerability.cvss-v3.confidentiality-impact" => Some(("vulnerability.cvssV3.confidentialityImpact", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "vulnerability.cvss-v3.exploitability-score" => Some(("vulnerability.cvssV3.exploitabilityScore", JsonTypeInfo { jtype: JsonType::Float, ctype: ComplexType::Pod })),
                    "vulnerability.cvss-v3.impact-score" => Some(("vulnerability.cvssV3.impactScore", JsonTypeInfo { jtype: JsonType::Float, ctype: ComplexType::Pod })),
                    "vulnerability.cvss-v3.integrity-impact" => Some(("vulnerability.cvssV3.integrityImpact", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "vulnerability.cvss-v3.privileges-required" => Some(("vulnerability.cvssV3.privilegesRequired", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "vulnerability.cvss-v3.scope" => Some(("vulnerability.cvssV3.scope", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "vulnerability.cvss-v3.user-interaction" => Some(("vulnerability.cvssV3.userInteraction", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "vulnerability.severity" => Some(("vulnerability.severity", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "vulnerability.source-update-time" => Some(("vulnerability.sourceUpdateTime", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    _ => {
                        let suggestion = FieldCursor::did_you_mean(key, &vec!["analysis-kind", "attack-complexity", "attack-vector", "attestation-authority", "availability-impact", "base-image", "base-score", "build", "builder-version", "confidentiality-impact", "create-time", "cvss-score", "cvss-v3", "deployable", "discovery", "expected-command", "expiration-time", "exploitability-score", "fingerprint", "hint", "human-readable-name", "impact-score", "integrity-impact", "intoto", "key-id", "key-type", "kind", "long-description", "name", "package", "privileges-required", "public-key", "related-note-names", "resource-uri", "resource-url", "scope", "severity", "short-description", "signature", "source-update-time", "step-name", "threshold", "update-time", "user-interaction", "v1-name", "v2-blob", "v2-name", "vulnerability"]);
                        err.issues.push(CLIError::Field(FieldError::Unknown(temp_cursor.to_string(), suggestion, value.map(|v| v.to_string()))));
                        None
                    }
                };
            if let Some((field_cursor_str, type_info)) = type_info {
                FieldCursor::from(field_cursor_str).set_json_value(&mut object, value.unwrap(), type_info, err, &temp_cursor);
            }
        }
        let mut request: api::Note = json::value::from_value(object).unwrap();
        let mut call = self.hub.projects().notes_create(request, opt.value_of("parent").unwrap_or(""));
        for parg in opt.values_of("v").map(|i|i.collect()).unwrap_or(Vec::new()).iter() {
            let (key, value) = parse_kv_arg(&*parg, err, false);
            match key {
                "note-id" => {
                    call = call.note_id(value.unwrap_or(""));
                },
                _ => {
                    let mut found = false;
                    for param in &self.gp {
                        if key == *param {
                            found = true;
                            call = call.param(self.gpm.iter().find(|t| t.0 == key).unwrap_or(&("", key)).1, value.unwrap_or("unset"));
                            break;
                        }
                    }
                    if !found {
                        err.issues.push(CLIError::UnknownParameter(key.to_string(),
                                                                  {let mut v = Vec::new();
                                                                           v.extend(self.gp.iter().map(|v|*v));
                                                                           v.extend(["note-id"].iter().map(|v|*v));
                                                                           v } ));
                    }
                }
            }
        }
        let protocol = CallType::Standard;
        if dry_run {
            Ok(())
        } else {
            assert!(err.issues.len() == 0);
            for scope in self.opt.values_of("url").map(|i|i.collect()).unwrap_or(Vec::new()).iter() {
                call = call.add_scope(scope);
            }
            let mut ostream = match writer_from_opts(opt.value_of("out")) {
                Ok(mut f) => f,
                Err(io_err) => return Err(DoitError::IoError(opt.value_of("out").unwrap_or("-").to_string(), io_err)),
            };
            match match protocol {
                CallType::Standard => call.doit().await,
                _ => unreachable!()
            } {
                Err(api_err) => Err(DoitError::ApiError(api_err)),
                Ok((mut response, output_schema)) => {
                    let mut value = json::value::to_value(&output_schema).expect("serde to work");
                    remove_json_null_values(&mut value);
                    json::to_writer_pretty(&mut ostream, &value).unwrap();
                    ostream.flush().unwrap();
                    Ok(())
                }
            }
        }
    }

    async fn _projects_notes_delete(&self, opt: &ArgMatches<'n>, dry_run: bool, err: &mut InvalidOptionsError)
                                                    -> Result<(), DoitError> {
        let mut call = self.hub.projects().notes_delete(opt.value_of("name").unwrap_or(""));
        for parg in opt.values_of("v").map(|i|i.collect()).unwrap_or(Vec::new()).iter() {
            let (key, value) = parse_kv_arg(&*parg, err, false);
            match key {
                _ => {
                    let mut found = false;
                    for param in &self.gp {
                        if key == *param {
                            found = true;
                            call = call.param(self.gpm.iter().find(|t| t.0 == key).unwrap_or(&("", key)).1, value.unwrap_or("unset"));
                            break;
                        }
                    }
                    if !found {
                        err.issues.push(CLIError::UnknownParameter(key.to_string(),
                                                                  {let mut v = Vec::new();
                                                                           v.extend(self.gp.iter().map(|v|*v));
                                                                           v } ));
                    }
                }
            }
        }
        let protocol = CallType::Standard;
        if dry_run {
            Ok(())
        } else {
            assert!(err.issues.len() == 0);
            for scope in self.opt.values_of("url").map(|i|i.collect()).unwrap_or(Vec::new()).iter() {
                call = call.add_scope(scope);
            }
            let mut ostream = match writer_from_opts(opt.value_of("out")) {
                Ok(mut f) => f,
                Err(io_err) => return Err(DoitError::IoError(opt.value_of("out").unwrap_or("-").to_string(), io_err)),
            };
            match match protocol {
                CallType::Standard => call.doit().await,
                _ => unreachable!()
            } {
                Err(api_err) => Err(DoitError::ApiError(api_err)),
                Ok((mut response, output_schema)) => {
                    let mut value = json::value::to_value(&output_schema).expect("serde to work");
                    remove_json_null_values(&mut value);
                    json::to_writer_pretty(&mut ostream, &value).unwrap();
                    ostream.flush().unwrap();
                    Ok(())
                }
            }
        }
    }

    async fn _projects_notes_get(&self, opt: &ArgMatches<'n>, dry_run: bool, err: &mut InvalidOptionsError)
                                                    -> Result<(), DoitError> {
        let mut call = self.hub.projects().notes_get(opt.value_of("name").unwrap_or(""));
        for parg in opt.values_of("v").map(|i|i.collect()).unwrap_or(Vec::new()).iter() {
            let (key, value) = parse_kv_arg(&*parg, err, false);
            match key {
                _ => {
                    let mut found = false;
                    for param in &self.gp {
                        if key == *param {
                            found = true;
                            call = call.param(self.gpm.iter().find(|t| t.0 == key).unwrap_or(&("", key)).1, value.unwrap_or("unset"));
                            break;
                        }
                    }
                    if !found {
                        err.issues.push(CLIError::UnknownParameter(key.to_string(),
                                                                  {let mut v = Vec::new();
                                                                           v.extend(self.gp.iter().map(|v|*v));
                                                                           v } ));
                    }
                }
            }
        }
        let protocol = CallType::Standard;
        if dry_run {
            Ok(())
        } else {
            assert!(err.issues.len() == 0);
            for scope in self.opt.values_of("url").map(|i|i.collect()).unwrap_or(Vec::new()).iter() {
                call = call.add_scope(scope);
            }
            let mut ostream = match writer_from_opts(opt.value_of("out")) {
                Ok(mut f) => f,
                Err(io_err) => return Err(DoitError::IoError(opt.value_of("out").unwrap_or("-").to_string(), io_err)),
            };
            match match protocol {
                CallType::Standard => call.doit().await,
                _ => unreachable!()
            } {
                Err(api_err) => Err(DoitError::ApiError(api_err)),
                Ok((mut response, output_schema)) => {
                    let mut value = json::value::to_value(&output_schema).expect("serde to work");
                    remove_json_null_values(&mut value);
                    json::to_writer_pretty(&mut ostream, &value).unwrap();
                    ostream.flush().unwrap();
                    Ok(())
                }
            }
        }
    }

    async fn _projects_notes_get_iam_policy(&self, opt: &ArgMatches<'n>, dry_run: bool, err: &mut InvalidOptionsError)
                                                    -> Result<(), DoitError> {
        
        let mut field_cursor = FieldCursor::default();
        let mut object = json::value::Value::Object(Default::default());
        
        for kvarg in opt.values_of("kv").map(|i|i.collect()).unwrap_or(Vec::new()).iter() {
            let last_errc = err.issues.len();
            let (key, value) = parse_kv_arg(&*kvarg, err, false);
            let mut temp_cursor = field_cursor.clone();
            if let Err(field_err) = temp_cursor.set(&*key) {
                err.issues.push(field_err);
            }
            if value.is_none() {
                field_cursor = temp_cursor.clone();
                if err.issues.len() > last_errc {
                    err.issues.remove(last_errc);
                }
                continue;
            }
        
            let type_info: Option<(&'static str, JsonTypeInfo)> =
                match &temp_cursor.to_string()[..] {
                    "options.requested-policy-version" => Some(("options.requestedPolicyVersion", JsonTypeInfo { jtype: JsonType::Int, ctype: ComplexType::Pod })),
                    _ => {
                        let suggestion = FieldCursor::did_you_mean(key, &vec!["options", "requested-policy-version"]);
                        err.issues.push(CLIError::Field(FieldError::Unknown(temp_cursor.to_string(), suggestion, value.map(|v| v.to_string()))));
                        None
                    }
                };
            if let Some((field_cursor_str, type_info)) = type_info {
                FieldCursor::from(field_cursor_str).set_json_value(&mut object, value.unwrap(), type_info, err, &temp_cursor);
            }
        }
        let mut request: api::GetIamPolicyRequest = json::value::from_value(object).unwrap();
        let mut call = self.hub.projects().notes_get_iam_policy(request, opt.value_of("resource").unwrap_or(""));
        for parg in opt.values_of("v").map(|i|i.collect()).unwrap_or(Vec::new()).iter() {
            let (key, value) = parse_kv_arg(&*parg, err, false);
            match key {
                _ => {
                    let mut found = false;
                    for param in &self.gp {
                        if key == *param {
                            found = true;
                            call = call.param(self.gpm.iter().find(|t| t.0 == key).unwrap_or(&("", key)).1, value.unwrap_or("unset"));
                            break;
                        }
                    }
                    if !found {
                        err.issues.push(CLIError::UnknownParameter(key.to_string(),
                                                                  {let mut v = Vec::new();
                                                                           v.extend(self.gp.iter().map(|v|*v));
                                                                           v } ));
                    }
                }
            }
        }
        let protocol = CallType::Standard;
        if dry_run {
            Ok(())
        } else {
            assert!(err.issues.len() == 0);
            for scope in self.opt.values_of("url").map(|i|i.collect()).unwrap_or(Vec::new()).iter() {
                call = call.add_scope(scope);
            }
            let mut ostream = match writer_from_opts(opt.value_of("out")) {
                Ok(mut f) => f,
                Err(io_err) => return Err(DoitError::IoError(opt.value_of("out").unwrap_or("-").to_string(), io_err)),
            };
            match match protocol {
                CallType::Standard => call.doit().await,
                _ => unreachable!()
            } {
                Err(api_err) => Err(DoitError::ApiError(api_err)),
                Ok((mut response, output_schema)) => {
                    let mut value = json::value::to_value(&output_schema).expect("serde to work");
                    remove_json_null_values(&mut value);
                    json::to_writer_pretty(&mut ostream, &value).unwrap();
                    ostream.flush().unwrap();
                    Ok(())
                }
            }
        }
    }

    async fn _projects_notes_list(&self, opt: &ArgMatches<'n>, dry_run: bool, err: &mut InvalidOptionsError)
                                                    -> Result<(), DoitError> {
        let mut call = self.hub.projects().notes_list(opt.value_of("parent").unwrap_or(""));
        for parg in opt.values_of("v").map(|i|i.collect()).unwrap_or(Vec::new()).iter() {
            let (key, value) = parse_kv_arg(&*parg, err, false);
            match key {
                "page-token" => {
                    call = call.page_token(value.unwrap_or(""));
                },
                "page-size" => {
                    call = call.page_size(arg_from_str(value.unwrap_or("-0"), err, "page-size", "integer"));
                },
                "filter" => {
                    call = call.filter(value.unwrap_or(""));
                },
                _ => {
                    let mut found = false;
                    for param in &self.gp {
                        if key == *param {
                            found = true;
                            call = call.param(self.gpm.iter().find(|t| t.0 == key).unwrap_or(&("", key)).1, value.unwrap_or("unset"));
                            break;
                        }
                    }
                    if !found {
                        err.issues.push(CLIError::UnknownParameter(key.to_string(),
                                                                  {let mut v = Vec::new();
                                                                           v.extend(self.gp.iter().map(|v|*v));
                                                                           v.extend(["filter", "page-size", "page-token"].iter().map(|v|*v));
                                                                           v } ));
                    }
                }
            }
        }
        let protocol = CallType::Standard;
        if dry_run {
            Ok(())
        } else {
            assert!(err.issues.len() == 0);
            for scope in self.opt.values_of("url").map(|i|i.collect()).unwrap_or(Vec::new()).iter() {
                call = call.add_scope(scope);
            }
            let mut ostream = match writer_from_opts(opt.value_of("out")) {
                Ok(mut f) => f,
                Err(io_err) => return Err(DoitError::IoError(opt.value_of("out").unwrap_or("-").to_string(), io_err)),
            };
            match match protocol {
                CallType::Standard => call.doit().await,
                _ => unreachable!()
            } {
                Err(api_err) => Err(DoitError::ApiError(api_err)),
                Ok((mut response, output_schema)) => {
                    let mut value = json::value::to_value(&output_schema).expect("serde to work");
                    remove_json_null_values(&mut value);
                    json::to_writer_pretty(&mut ostream, &value).unwrap();
                    ostream.flush().unwrap();
                    Ok(())
                }
            }
        }
    }

    async fn _projects_notes_occurrences_list(&self, opt: &ArgMatches<'n>, dry_run: bool, err: &mut InvalidOptionsError)
                                                    -> Result<(), DoitError> {
        let mut call = self.hub.projects().notes_occurrences_list(opt.value_of("name").unwrap_or(""));
        for parg in opt.values_of("v").map(|i|i.collect()).unwrap_or(Vec::new()).iter() {
            let (key, value) = parse_kv_arg(&*parg, err, false);
            match key {
                "page-token" => {
                    call = call.page_token(value.unwrap_or(""));
                },
                "page-size" => {
                    call = call.page_size(arg_from_str(value.unwrap_or("-0"), err, "page-size", "integer"));
                },
                "filter" => {
                    call = call.filter(value.unwrap_or(""));
                },
                _ => {
                    let mut found = false;
                    for param in &self.gp {
                        if key == *param {
                            found = true;
                            call = call.param(self.gpm.iter().find(|t| t.0 == key).unwrap_or(&("", key)).1, value.unwrap_or("unset"));
                            break;
                        }
                    }
                    if !found {
                        err.issues.push(CLIError::UnknownParameter(key.to_string(),
                                                                  {let mut v = Vec::new();
                                                                           v.extend(self.gp.iter().map(|v|*v));
                                                                           v.extend(["filter", "page-size", "page-token"].iter().map(|v|*v));
                                                                           v } ));
                    }
                }
            }
        }
        let protocol = CallType::Standard;
        if dry_run {
            Ok(())
        } else {
            assert!(err.issues.len() == 0);
            for scope in self.opt.values_of("url").map(|i|i.collect()).unwrap_or(Vec::new()).iter() {
                call = call.add_scope(scope);
            }
            let mut ostream = match writer_from_opts(opt.value_of("out")) {
                Ok(mut f) => f,
                Err(io_err) => return Err(DoitError::IoError(opt.value_of("out").unwrap_or("-").to_string(), io_err)),
            };
            match match protocol {
                CallType::Standard => call.doit().await,
                _ => unreachable!()
            } {
                Err(api_err) => Err(DoitError::ApiError(api_err)),
                Ok((mut response, output_schema)) => {
                    let mut value = json::value::to_value(&output_schema).expect("serde to work");
                    remove_json_null_values(&mut value);
                    json::to_writer_pretty(&mut ostream, &value).unwrap();
                    ostream.flush().unwrap();
                    Ok(())
                }
            }
        }
    }

    async fn _projects_notes_patch(&self, opt: &ArgMatches<'n>, dry_run: bool, err: &mut InvalidOptionsError)
                                                    -> Result<(), DoitError> {
        
        let mut field_cursor = FieldCursor::default();
        let mut object = json::value::Value::Object(Default::default());
        
        for kvarg in opt.values_of("kv").map(|i|i.collect()).unwrap_or(Vec::new()).iter() {
            let last_errc = err.issues.len();
            let (key, value) = parse_kv_arg(&*kvarg, err, false);
            let mut temp_cursor = field_cursor.clone();
            if let Err(field_err) = temp_cursor.set(&*key) {
                err.issues.push(field_err);
            }
            if value.is_none() {
                field_cursor = temp_cursor.clone();
                if err.issues.len() > last_errc {
                    err.issues.remove(last_errc);
                }
                continue;
            }
        
            let type_info: Option<(&'static str, JsonTypeInfo)> =
                match &temp_cursor.to_string()[..] {
                    "attestation-authority.hint.human-readable-name" => Some(("attestationAuthority.hint.humanReadableName", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "base-image.fingerprint.v1-name" => Some(("baseImage.fingerprint.v1Name", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "base-image.fingerprint.v2-blob" => Some(("baseImage.fingerprint.v2Blob", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Vec })),
                    "base-image.fingerprint.v2-name" => Some(("baseImage.fingerprint.v2Name", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "base-image.resource-url" => Some(("baseImage.resourceUrl", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "build.builder-version" => Some(("build.builderVersion", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "build.signature.key-id" => Some(("build.signature.keyId", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "build.signature.key-type" => Some(("build.signature.keyType", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "build.signature.public-key" => Some(("build.signature.publicKey", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "build.signature.signature" => Some(("build.signature.signature", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "create-time" => Some(("createTime", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "deployable.resource-uri" => Some(("deployable.resourceUri", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Vec })),
                    "discovery.analysis-kind" => Some(("discovery.analysisKind", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "expiration-time" => Some(("expirationTime", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "intoto.expected-command" => Some(("intoto.expectedCommand", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Vec })),
                    "intoto.step-name" => Some(("intoto.stepName", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "intoto.threshold" => Some(("intoto.threshold", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "kind" => Some(("kind", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "long-description" => Some(("longDescription", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "name" => Some(("name", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "package.name" => Some(("package.name", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "related-note-names" => Some(("relatedNoteNames", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Vec })),
                    "short-description" => Some(("shortDescription", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "update-time" => Some(("updateTime", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "vulnerability.cvss-score" => Some(("vulnerability.cvssScore", JsonTypeInfo { jtype: JsonType::Float, ctype: ComplexType::Pod })),
                    "vulnerability.cvss-v3.attack-complexity" => Some(("vulnerability.cvssV3.attackComplexity", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "vulnerability.cvss-v3.attack-vector" => Some(("vulnerability.cvssV3.attackVector", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "vulnerability.cvss-v3.availability-impact" => Some(("vulnerability.cvssV3.availabilityImpact", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "vulnerability.cvss-v3.base-score" => Some(("vulnerability.cvssV3.baseScore", JsonTypeInfo { jtype: JsonType::Float, ctype: ComplexType::Pod })),
                    "vulnerability.cvss-v3.confidentiality-impact" => Some(("vulnerability.cvssV3.confidentialityImpact", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "vulnerability.cvss-v3.exploitability-score" => Some(("vulnerability.cvssV3.exploitabilityScore", JsonTypeInfo { jtype: JsonType::Float, ctype: ComplexType::Pod })),
                    "vulnerability.cvss-v3.impact-score" => Some(("vulnerability.cvssV3.impactScore", JsonTypeInfo { jtype: JsonType::Float, ctype: ComplexType::Pod })),
                    "vulnerability.cvss-v3.integrity-impact" => Some(("vulnerability.cvssV3.integrityImpact", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "vulnerability.cvss-v3.privileges-required" => Some(("vulnerability.cvssV3.privilegesRequired", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "vulnerability.cvss-v3.scope" => Some(("vulnerability.cvssV3.scope", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "vulnerability.cvss-v3.user-interaction" => Some(("vulnerability.cvssV3.userInteraction", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "vulnerability.severity" => Some(("vulnerability.severity", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "vulnerability.source-update-time" => Some(("vulnerability.sourceUpdateTime", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    _ => {
                        let suggestion = FieldCursor::did_you_mean(key, &vec!["analysis-kind", "attack-complexity", "attack-vector", "attestation-authority", "availability-impact", "base-image", "base-score", "build", "builder-version", "confidentiality-impact", "create-time", "cvss-score", "cvss-v3", "deployable", "discovery", "expected-command", "expiration-time", "exploitability-score", "fingerprint", "hint", "human-readable-name", "impact-score", "integrity-impact", "intoto", "key-id", "key-type", "kind", "long-description", "name", "package", "privileges-required", "public-key", "related-note-names", "resource-uri", "resource-url", "scope", "severity", "short-description", "signature", "source-update-time", "step-name", "threshold", "update-time", "user-interaction", "v1-name", "v2-blob", "v2-name", "vulnerability"]);
                        err.issues.push(CLIError::Field(FieldError::Unknown(temp_cursor.to_string(), suggestion, value.map(|v| v.to_string()))));
                        None
                    }
                };
            if let Some((field_cursor_str, type_info)) = type_info {
                FieldCursor::from(field_cursor_str).set_json_value(&mut object, value.unwrap(), type_info, err, &temp_cursor);
            }
        }
        let mut request: api::Note = json::value::from_value(object).unwrap();
        let mut call = self.hub.projects().notes_patch(request, opt.value_of("name").unwrap_or(""));
        for parg in opt.values_of("v").map(|i|i.collect()).unwrap_or(Vec::new()).iter() {
            let (key, value) = parse_kv_arg(&*parg, err, false);
            match key {
                "update-mask" => {
                    call = call.update_mask(value.unwrap_or(""));
                },
                _ => {
                    let mut found = false;
                    for param in &self.gp {
                        if key == *param {
                            found = true;
                            call = call.param(self.gpm.iter().find(|t| t.0 == key).unwrap_or(&("", key)).1, value.unwrap_or("unset"));
                            break;
                        }
                    }
                    if !found {
                        err.issues.push(CLIError::UnknownParameter(key.to_string(),
                                                                  {let mut v = Vec::new();
                                                                           v.extend(self.gp.iter().map(|v|*v));
                                                                           v.extend(["update-mask"].iter().map(|v|*v));
                                                                           v } ));
                    }
                }
            }
        }
        let protocol = CallType::Standard;
        if dry_run {
            Ok(())
        } else {
            assert!(err.issues.len() == 0);
            for scope in self.opt.values_of("url").map(|i|i.collect()).unwrap_or(Vec::new()).iter() {
                call = call.add_scope(scope);
            }
            let mut ostream = match writer_from_opts(opt.value_of("out")) {
                Ok(mut f) => f,
                Err(io_err) => return Err(DoitError::IoError(opt.value_of("out").unwrap_or("-").to_string(), io_err)),
            };
            match match protocol {
                CallType::Standard => call.doit().await,
                _ => unreachable!()
            } {
                Err(api_err) => Err(DoitError::ApiError(api_err)),
                Ok((mut response, output_schema)) => {
                    let mut value = json::value::to_value(&output_schema).expect("serde to work");
                    remove_json_null_values(&mut value);
                    json::to_writer_pretty(&mut ostream, &value).unwrap();
                    ostream.flush().unwrap();
                    Ok(())
                }
            }
        }
    }

    async fn _projects_notes_set_iam_policy(&self, opt: &ArgMatches<'n>, dry_run: bool, err: &mut InvalidOptionsError)
                                                    -> Result<(), DoitError> {
        
        let mut field_cursor = FieldCursor::default();
        let mut object = json::value::Value::Object(Default::default());
        
        for kvarg in opt.values_of("kv").map(|i|i.collect()).unwrap_or(Vec::new()).iter() {
            let last_errc = err.issues.len();
            let (key, value) = parse_kv_arg(&*kvarg, err, false);
            let mut temp_cursor = field_cursor.clone();
            if let Err(field_err) = temp_cursor.set(&*key) {
                err.issues.push(field_err);
            }
            if value.is_none() {
                field_cursor = temp_cursor.clone();
                if err.issues.len() > last_errc {
                    err.issues.remove(last_errc);
                }
                continue;
            }
        
            let type_info: Option<(&'static str, JsonTypeInfo)> =
                match &temp_cursor.to_string()[..] {
                    "policy.etag" => Some(("policy.etag", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "policy.version" => Some(("policy.version", JsonTypeInfo { jtype: JsonType::Int, ctype: ComplexType::Pod })),
                    _ => {
                        let suggestion = FieldCursor::did_you_mean(key, &vec!["etag", "policy", "version"]);
                        err.issues.push(CLIError::Field(FieldError::Unknown(temp_cursor.to_string(), suggestion, value.map(|v| v.to_string()))));
                        None
                    }
                };
            if let Some((field_cursor_str, type_info)) = type_info {
                FieldCursor::from(field_cursor_str).set_json_value(&mut object, value.unwrap(), type_info, err, &temp_cursor);
            }
        }
        let mut request: api::SetIamPolicyRequest = json::value::from_value(object).unwrap();
        let mut call = self.hub.projects().notes_set_iam_policy(request, opt.value_of("resource").unwrap_or(""));
        for parg in opt.values_of("v").map(|i|i.collect()).unwrap_or(Vec::new()).iter() {
            let (key, value) = parse_kv_arg(&*parg, err, false);
            match key {
                _ => {
                    let mut found = false;
                    for param in &self.gp {
                        if key == *param {
                            found = true;
                            call = call.param(self.gpm.iter().find(|t| t.0 == key).unwrap_or(&("", key)).1, value.unwrap_or("unset"));
                            break;
                        }
                    }
                    if !found {
                        err.issues.push(CLIError::UnknownParameter(key.to_string(),
                                                                  {let mut v = Vec::new();
                                                                           v.extend(self.gp.iter().map(|v|*v));
                                                                           v } ));
                    }
                }
            }
        }
        let protocol = CallType::Standard;
        if dry_run {
            Ok(())
        } else {
            assert!(err.issues.len() == 0);
            for scope in self.opt.values_of("url").map(|i|i.collect()).unwrap_or(Vec::new()).iter() {
                call = call.add_scope(scope);
            }
            let mut ostream = match writer_from_opts(opt.value_of("out")) {
                Ok(mut f) => f,
                Err(io_err) => return Err(DoitError::IoError(opt.value_of("out").unwrap_or("-").to_string(), io_err)),
            };
            match match protocol {
                CallType::Standard => call.doit().await,
                _ => unreachable!()
            } {
                Err(api_err) => Err(DoitError::ApiError(api_err)),
                Ok((mut response, output_schema)) => {
                    let mut value = json::value::to_value(&output_schema).expect("serde to work");
                    remove_json_null_values(&mut value);
                    json::to_writer_pretty(&mut ostream, &value).unwrap();
                    ostream.flush().unwrap();
                    Ok(())
                }
            }
        }
    }

    async fn _projects_notes_test_iam_permissions(&self, opt: &ArgMatches<'n>, dry_run: bool, err: &mut InvalidOptionsError)
                                                    -> Result<(), DoitError> {
        
        let mut field_cursor = FieldCursor::default();
        let mut object = json::value::Value::Object(Default::default());
        
        for kvarg in opt.values_of("kv").map(|i|i.collect()).unwrap_or(Vec::new()).iter() {
            let last_errc = err.issues.len();
            let (key, value) = parse_kv_arg(&*kvarg, err, false);
            let mut temp_cursor = field_cursor.clone();
            if let Err(field_err) = temp_cursor.set(&*key) {
                err.issues.push(field_err);
            }
            if value.is_none() {
                field_cursor = temp_cursor.clone();
                if err.issues.len() > last_errc {
                    err.issues.remove(last_errc);
                }
                continue;
            }
        
            let type_info: Option<(&'static str, JsonTypeInfo)> =
                match &temp_cursor.to_string()[..] {
                    "permissions" => Some(("permissions", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Vec })),
                    _ => {
                        let suggestion = FieldCursor::did_you_mean(key, &vec!["permissions"]);
                        err.issues.push(CLIError::Field(FieldError::Unknown(temp_cursor.to_string(), suggestion, value.map(|v| v.to_string()))));
                        None
                    }
                };
            if let Some((field_cursor_str, type_info)) = type_info {
                FieldCursor::from(field_cursor_str).set_json_value(&mut object, value.unwrap(), type_info, err, &temp_cursor);
            }
        }
        let mut request: api::TestIamPermissionsRequest = json::value::from_value(object).unwrap();
        let mut call = self.hub.projects().notes_test_iam_permissions(request, opt.value_of("resource").unwrap_or(""));
        for parg in opt.values_of("v").map(|i|i.collect()).unwrap_or(Vec::new()).iter() {
            let (key, value) = parse_kv_arg(&*parg, err, false);
            match key {
                _ => {
                    let mut found = false;
                    for param in &self.gp {
                        if key == *param {
                            found = true;
                            call = call.param(self.gpm.iter().find(|t| t.0 == key).unwrap_or(&("", key)).1, value.unwrap_or("unset"));
                            break;
                        }
                    }
                    if !found {
                        err.issues.push(CLIError::UnknownParameter(key.to_string(),
                                                                  {let mut v = Vec::new();
                                                                           v.extend(self.gp.iter().map(|v|*v));
                                                                           v } ));
                    }
                }
            }
        }
        let protocol = CallType::Standard;
        if dry_run {
            Ok(())
        } else {
            assert!(err.issues.len() == 0);
            for scope in self.opt.values_of("url").map(|i|i.collect()).unwrap_or(Vec::new()).iter() {
                call = call.add_scope(scope);
            }
            let mut ostream = match writer_from_opts(opt.value_of("out")) {
                Ok(mut f) => f,
                Err(io_err) => return Err(DoitError::IoError(opt.value_of("out").unwrap_or("-").to_string(), io_err)),
            };
            match match protocol {
                CallType::Standard => call.doit().await,
                _ => unreachable!()
            } {
                Err(api_err) => Err(DoitError::ApiError(api_err)),
                Ok((mut response, output_schema)) => {
                    let mut value = json::value::to_value(&output_schema).expect("serde to work");
                    remove_json_null_values(&mut value);
                    json::to_writer_pretty(&mut ostream, &value).unwrap();
                    ostream.flush().unwrap();
                    Ok(())
                }
            }
        }
    }

    async fn _projects_occurrences_batch_create(&self, opt: &ArgMatches<'n>, dry_run: bool, err: &mut InvalidOptionsError)
                                                    -> Result<(), DoitError> {
        
        let mut field_cursor = FieldCursor::default();
        let mut object = json::value::Value::Object(Default::default());
        
        for kvarg in opt.values_of("kv").map(|i|i.collect()).unwrap_or(Vec::new()).iter() {
            let last_errc = err.issues.len();
            let (key, value) = parse_kv_arg(&*kvarg, err, false);
            let mut temp_cursor = field_cursor.clone();
            if let Err(field_err) = temp_cursor.set(&*key) {
                err.issues.push(field_err);
            }
            if value.is_none() {
                field_cursor = temp_cursor.clone();
                if err.issues.len() > last_errc {
                    err.issues.remove(last_errc);
                }
                continue;
            }
        
            let type_info: Option<(&'static str, JsonTypeInfo)> =
                match &temp_cursor.to_string()[..] {
                    _ => {
                        let suggestion = FieldCursor::did_you_mean(key, &vec![]);
                        err.issues.push(CLIError::Field(FieldError::Unknown(temp_cursor.to_string(), suggestion, value.map(|v| v.to_string()))));
                        None
                    }
                };
            if let Some((field_cursor_str, type_info)) = type_info {
                FieldCursor::from(field_cursor_str).set_json_value(&mut object, value.unwrap(), type_info, err, &temp_cursor);
            }
        }
        let mut request: api::BatchCreateOccurrencesRequest = json::value::from_value(object).unwrap();
        let mut call = self.hub.projects().occurrences_batch_create(request, opt.value_of("parent").unwrap_or(""));
        for parg in opt.values_of("v").map(|i|i.collect()).unwrap_or(Vec::new()).iter() {
            let (key, value) = parse_kv_arg(&*parg, err, false);
            match key {
                _ => {
                    let mut found = false;
                    for param in &self.gp {
                        if key == *param {
                            found = true;
                            call = call.param(self.gpm.iter().find(|t| t.0 == key).unwrap_or(&("", key)).1, value.unwrap_or("unset"));
                            break;
                        }
                    }
                    if !found {
                        err.issues.push(CLIError::UnknownParameter(key.to_string(),
                                                                  {let mut v = Vec::new();
                                                                           v.extend(self.gp.iter().map(|v|*v));
                                                                           v } ));
                    }
                }
            }
        }
        let protocol = CallType::Standard;
        if dry_run {
            Ok(())
        } else {
            assert!(err.issues.len() == 0);
            for scope in self.opt.values_of("url").map(|i|i.collect()).unwrap_or(Vec::new()).iter() {
                call = call.add_scope(scope);
            }
            let mut ostream = match writer_from_opts(opt.value_of("out")) {
                Ok(mut f) => f,
                Err(io_err) => return Err(DoitError::IoError(opt.value_of("out").unwrap_or("-").to_string(), io_err)),
            };
            match match protocol {
                CallType::Standard => call.doit().await,
                _ => unreachable!()
            } {
                Err(api_err) => Err(DoitError::ApiError(api_err)),
                Ok((mut response, output_schema)) => {
                    let mut value = json::value::to_value(&output_schema).expect("serde to work");
                    remove_json_null_values(&mut value);
                    json::to_writer_pretty(&mut ostream, &value).unwrap();
                    ostream.flush().unwrap();
                    Ok(())
                }
            }
        }
    }

    async fn _projects_occurrences_create(&self, opt: &ArgMatches<'n>, dry_run: bool, err: &mut InvalidOptionsError)
                                                    -> Result<(), DoitError> {
        
        let mut field_cursor = FieldCursor::default();
        let mut object = json::value::Value::Object(Default::default());
        
        for kvarg in opt.values_of("kv").map(|i|i.collect()).unwrap_or(Vec::new()).iter() {
            let last_errc = err.issues.len();
            let (key, value) = parse_kv_arg(&*kvarg, err, false);
            let mut temp_cursor = field_cursor.clone();
            if let Err(field_err) = temp_cursor.set(&*key) {
                err.issues.push(field_err);
            }
            if value.is_none() {
                field_cursor = temp_cursor.clone();
                if err.issues.len() > last_errc {
                    err.issues.remove(last_errc);
                }
                continue;
            }
        
            let type_info: Option<(&'static str, JsonTypeInfo)> =
                match &temp_cursor.to_string()[..] {
                    "attestation.attestation.generic-signed-attestation.content-type" => Some(("attestation.attestation.genericSignedAttestation.contentType", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "attestation.attestation.generic-signed-attestation.serialized-payload" => Some(("attestation.attestation.genericSignedAttestation.serializedPayload", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "attestation.attestation.pgp-signed-attestation.content-type" => Some(("attestation.attestation.pgpSignedAttestation.contentType", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "attestation.attestation.pgp-signed-attestation.pgp-key-id" => Some(("attestation.attestation.pgpSignedAttestation.pgpKeyId", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "attestation.attestation.pgp-signed-attestation.signature" => Some(("attestation.attestation.pgpSignedAttestation.signature", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "build.provenance.build-options" => Some(("build.provenance.buildOptions", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Map })),
                    "build.provenance.builder-version" => Some(("build.provenance.builderVersion", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "build.provenance.create-time" => Some(("build.provenance.createTime", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "build.provenance.creator" => Some(("build.provenance.creator", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "build.provenance.end-time" => Some(("build.provenance.endTime", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "build.provenance.id" => Some(("build.provenance.id", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "build.provenance.logs-uri" => Some(("build.provenance.logsUri", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "build.provenance.project-id" => Some(("build.provenance.projectId", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "build.provenance.source-provenance.artifact-storage-source-uri" => Some(("build.provenance.sourceProvenance.artifactStorageSourceUri", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "build.provenance.source-provenance.context.cloud-repo.alias-context.kind" => Some(("build.provenance.sourceProvenance.context.cloudRepo.aliasContext.kind", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "build.provenance.source-provenance.context.cloud-repo.alias-context.name" => Some(("build.provenance.sourceProvenance.context.cloudRepo.aliasContext.name", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "build.provenance.source-provenance.context.cloud-repo.repo-id.project-repo-id.project-id" => Some(("build.provenance.sourceProvenance.context.cloudRepo.repoId.projectRepoId.projectId", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "build.provenance.source-provenance.context.cloud-repo.repo-id.project-repo-id.repo-name" => Some(("build.provenance.sourceProvenance.context.cloudRepo.repoId.projectRepoId.repoName", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "build.provenance.source-provenance.context.cloud-repo.repo-id.uid" => Some(("build.provenance.sourceProvenance.context.cloudRepo.repoId.uid", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "build.provenance.source-provenance.context.cloud-repo.revision-id" => Some(("build.provenance.sourceProvenance.context.cloudRepo.revisionId", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "build.provenance.source-provenance.context.gerrit.alias-context.kind" => Some(("build.provenance.sourceProvenance.context.gerrit.aliasContext.kind", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "build.provenance.source-provenance.context.gerrit.alias-context.name" => Some(("build.provenance.sourceProvenance.context.gerrit.aliasContext.name", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "build.provenance.source-provenance.context.gerrit.gerrit-project" => Some(("build.provenance.sourceProvenance.context.gerrit.gerritProject", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "build.provenance.source-provenance.context.gerrit.host-uri" => Some(("build.provenance.sourceProvenance.context.gerrit.hostUri", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "build.provenance.source-provenance.context.gerrit.revision-id" => Some(("build.provenance.sourceProvenance.context.gerrit.revisionId", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "build.provenance.source-provenance.context.git.revision-id" => Some(("build.provenance.sourceProvenance.context.git.revisionId", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "build.provenance.source-provenance.context.git.url" => Some(("build.provenance.sourceProvenance.context.git.url", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "build.provenance.source-provenance.context.labels" => Some(("build.provenance.sourceProvenance.context.labels", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Map })),
                    "build.provenance.start-time" => Some(("build.provenance.startTime", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "build.provenance.trigger-id" => Some(("build.provenance.triggerId", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "build.provenance-bytes" => Some(("build.provenanceBytes", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "create-time" => Some(("createTime", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "deployment.deployment.address" => Some(("deployment.deployment.address", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "deployment.deployment.config" => Some(("deployment.deployment.config", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "deployment.deployment.deploy-time" => Some(("deployment.deployment.deployTime", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "deployment.deployment.platform" => Some(("deployment.deployment.platform", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "deployment.deployment.resource-uri" => Some(("deployment.deployment.resourceUri", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Vec })),
                    "deployment.deployment.undeploy-time" => Some(("deployment.deployment.undeployTime", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "deployment.deployment.user-email" => Some(("deployment.deployment.userEmail", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "derived-image.derived-image.base-resource-url" => Some(("derivedImage.derivedImage.baseResourceUrl", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "derived-image.derived-image.distance" => Some(("derivedImage.derivedImage.distance", JsonTypeInfo { jtype: JsonType::Int, ctype: ComplexType::Pod })),
                    "derived-image.derived-image.fingerprint.v1-name" => Some(("derivedImage.derivedImage.fingerprint.v1Name", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "derived-image.derived-image.fingerprint.v2-blob" => Some(("derivedImage.derivedImage.fingerprint.v2Blob", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Vec })),
                    "derived-image.derived-image.fingerprint.v2-name" => Some(("derivedImage.derivedImage.fingerprint.v2Name", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "discovered.discovered.analysis-status" => Some(("discovered.discovered.analysisStatus", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "discovered.discovered.analysis-status-error.code" => Some(("discovered.discovered.analysisStatusError.code", JsonTypeInfo { jtype: JsonType::Int, ctype: ComplexType::Pod })),
                    "discovered.discovered.analysis-status-error.message" => Some(("discovered.discovered.analysisStatusError.message", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "discovered.discovered.continuous-analysis" => Some(("discovered.discovered.continuousAnalysis", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "discovered.discovered.last-analysis-time" => Some(("discovered.discovered.lastAnalysisTime", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "installation.installation.name" => Some(("installation.installation.name", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "intoto.signed.byproducts.custom-values" => Some(("intoto.signed.byproducts.customValues", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Map })),
                    "intoto.signed.command" => Some(("intoto.signed.command", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Vec })),
                    "intoto.signed.environment.custom-values" => Some(("intoto.signed.environment.customValues", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Map })),
                    "kind" => Some(("kind", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "name" => Some(("name", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "note-name" => Some(("noteName", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "remediation" => Some(("remediation", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "resource.content-hash.type" => Some(("resource.contentHash.type", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "resource.content-hash.value" => Some(("resource.contentHash.value", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "resource.name" => Some(("resource.name", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "resource.uri" => Some(("resource.uri", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "update-time" => Some(("updateTime", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "vulnerability.cvss-score" => Some(("vulnerability.cvssScore", JsonTypeInfo { jtype: JsonType::Float, ctype: ComplexType::Pod })),
                    "vulnerability.effective-severity" => Some(("vulnerability.effectiveSeverity", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "vulnerability.long-description" => Some(("vulnerability.longDescription", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "vulnerability.severity" => Some(("vulnerability.severity", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "vulnerability.short-description" => Some(("vulnerability.shortDescription", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "vulnerability.type" => Some(("vulnerability.type", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    _ => {
                        let suggestion = FieldCursor::did_you_mean(key, &vec!["address", "alias-context", "analysis-status", "analysis-status-error", "artifact-storage-source-uri", "attestation", "base-resource-url", "build", "build-options", "builder-version", "byproducts", "cloud-repo", "code", "command", "config", "content-hash", "content-type", "context", "continuous-analysis", "create-time", "creator", "custom-values", "cvss-score", "deploy-time", "deployment", "derived-image", "discovered", "distance", "effective-severity", "end-time", "environment", "fingerprint", "generic-signed-attestation", "gerrit", "gerrit-project", "git", "host-uri", "id", "installation", "intoto", "kind", "labels", "last-analysis-time", "logs-uri", "long-description", "message", "name", "note-name", "pgp-key-id", "pgp-signed-attestation", "platform", "project-id", "project-repo-id", "provenance", "provenance-bytes", "remediation", "repo-id", "repo-name", "resource", "resource-uri", "revision-id", "serialized-payload", "severity", "short-description", "signature", "signed", "source-provenance", "start-time", "trigger-id", "type", "uid", "undeploy-time", "update-time", "uri", "url", "user-email", "v1-name", "v2-blob", "v2-name", "value", "vulnerability"]);
                        err.issues.push(CLIError::Field(FieldError::Unknown(temp_cursor.to_string(), suggestion, value.map(|v| v.to_string()))));
                        None
                    }
                };
            if let Some((field_cursor_str, type_info)) = type_info {
                FieldCursor::from(field_cursor_str).set_json_value(&mut object, value.unwrap(), type_info, err, &temp_cursor);
            }
        }
        let mut request: api::Occurrence = json::value::from_value(object).unwrap();
        let mut call = self.hub.projects().occurrences_create(request, opt.value_of("parent").unwrap_or(""));
        for parg in opt.values_of("v").map(|i|i.collect()).unwrap_or(Vec::new()).iter() {
            let (key, value) = parse_kv_arg(&*parg, err, false);
            match key {
                _ => {
                    let mut found = false;
                    for param in &self.gp {
                        if key == *param {
                            found = true;
                            call = call.param(self.gpm.iter().find(|t| t.0 == key).unwrap_or(&("", key)).1, value.unwrap_or("unset"));
                            break;
                        }
                    }
                    if !found {
                        err.issues.push(CLIError::UnknownParameter(key.to_string(),
                                                                  {let mut v = Vec::new();
                                                                           v.extend(self.gp.iter().map(|v|*v));
                                                                           v } ));
                    }
                }
            }
        }
        let protocol = CallType::Standard;
        if dry_run {
            Ok(())
        } else {
            assert!(err.issues.len() == 0);
            for scope in self.opt.values_of("url").map(|i|i.collect()).unwrap_or(Vec::new()).iter() {
                call = call.add_scope(scope);
            }
            let mut ostream = match writer_from_opts(opt.value_of("out")) {
                Ok(mut f) => f,
                Err(io_err) => return Err(DoitError::IoError(opt.value_of("out").unwrap_or("-").to_string(), io_err)),
            };
            match match protocol {
                CallType::Standard => call.doit().await,
                _ => unreachable!()
            } {
                Err(api_err) => Err(DoitError::ApiError(api_err)),
                Ok((mut response, output_schema)) => {
                    let mut value = json::value::to_value(&output_schema).expect("serde to work");
                    remove_json_null_values(&mut value);
                    json::to_writer_pretty(&mut ostream, &value).unwrap();
                    ostream.flush().unwrap();
                    Ok(())
                }
            }
        }
    }

    async fn _projects_occurrences_delete(&self, opt: &ArgMatches<'n>, dry_run: bool, err: &mut InvalidOptionsError)
                                                    -> Result<(), DoitError> {
        let mut call = self.hub.projects().occurrences_delete(opt.value_of("name").unwrap_or(""));
        for parg in opt.values_of("v").map(|i|i.collect()).unwrap_or(Vec::new()).iter() {
            let (key, value) = parse_kv_arg(&*parg, err, false);
            match key {
                _ => {
                    let mut found = false;
                    for param in &self.gp {
                        if key == *param {
                            found = true;
                            call = call.param(self.gpm.iter().find(|t| t.0 == key).unwrap_or(&("", key)).1, value.unwrap_or("unset"));
                            break;
                        }
                    }
                    if !found {
                        err.issues.push(CLIError::UnknownParameter(key.to_string(),
                                                                  {let mut v = Vec::new();
                                                                           v.extend(self.gp.iter().map(|v|*v));
                                                                           v } ));
                    }
                }
            }
        }
        let protocol = CallType::Standard;
        if dry_run {
            Ok(())
        } else {
            assert!(err.issues.len() == 0);
            for scope in self.opt.values_of("url").map(|i|i.collect()).unwrap_or(Vec::new()).iter() {
                call = call.add_scope(scope);
            }
            let mut ostream = match writer_from_opts(opt.value_of("out")) {
                Ok(mut f) => f,
                Err(io_err) => return Err(DoitError::IoError(opt.value_of("out").unwrap_or("-").to_string(), io_err)),
            };
            match match protocol {
                CallType::Standard => call.doit().await,
                _ => unreachable!()
            } {
                Err(api_err) => Err(DoitError::ApiError(api_err)),
                Ok((mut response, output_schema)) => {
                    let mut value = json::value::to_value(&output_schema).expect("serde to work");
                    remove_json_null_values(&mut value);
                    json::to_writer_pretty(&mut ostream, &value).unwrap();
                    ostream.flush().unwrap();
                    Ok(())
                }
            }
        }
    }

    async fn _projects_occurrences_get(&self, opt: &ArgMatches<'n>, dry_run: bool, err: &mut InvalidOptionsError)
                                                    -> Result<(), DoitError> {
        let mut call = self.hub.projects().occurrences_get(opt.value_of("name").unwrap_or(""));
        for parg in opt.values_of("v").map(|i|i.collect()).unwrap_or(Vec::new()).iter() {
            let (key, value) = parse_kv_arg(&*parg, err, false);
            match key {
                _ => {
                    let mut found = false;
                    for param in &self.gp {
                        if key == *param {
                            found = true;
                            call = call.param(self.gpm.iter().find(|t| t.0 == key).unwrap_or(&("", key)).1, value.unwrap_or("unset"));
                            break;
                        }
                    }
                    if !found {
                        err.issues.push(CLIError::UnknownParameter(key.to_string(),
                                                                  {let mut v = Vec::new();
                                                                           v.extend(self.gp.iter().map(|v|*v));
                                                                           v } ));
                    }
                }
            }
        }
        let protocol = CallType::Standard;
        if dry_run {
            Ok(())
        } else {
            assert!(err.issues.len() == 0);
            for scope in self.opt.values_of("url").map(|i|i.collect()).unwrap_or(Vec::new()).iter() {
                call = call.add_scope(scope);
            }
            let mut ostream = match writer_from_opts(opt.value_of("out")) {
                Ok(mut f) => f,
                Err(io_err) => return Err(DoitError::IoError(opt.value_of("out").unwrap_or("-").to_string(), io_err)),
            };
            match match protocol {
                CallType::Standard => call.doit().await,
                _ => unreachable!()
            } {
                Err(api_err) => Err(DoitError::ApiError(api_err)),
                Ok((mut response, output_schema)) => {
                    let mut value = json::value::to_value(&output_schema).expect("serde to work");
                    remove_json_null_values(&mut value);
                    json::to_writer_pretty(&mut ostream, &value).unwrap();
                    ostream.flush().unwrap();
                    Ok(())
                }
            }
        }
    }

    async fn _projects_occurrences_get_iam_policy(&self, opt: &ArgMatches<'n>, dry_run: bool, err: &mut InvalidOptionsError)
                                                    -> Result<(), DoitError> {
        
        let mut field_cursor = FieldCursor::default();
        let mut object = json::value::Value::Object(Default::default());
        
        for kvarg in opt.values_of("kv").map(|i|i.collect()).unwrap_or(Vec::new()).iter() {
            let last_errc = err.issues.len();
            let (key, value) = parse_kv_arg(&*kvarg, err, false);
            let mut temp_cursor = field_cursor.clone();
            if let Err(field_err) = temp_cursor.set(&*key) {
                err.issues.push(field_err);
            }
            if value.is_none() {
                field_cursor = temp_cursor.clone();
                if err.issues.len() > last_errc {
                    err.issues.remove(last_errc);
                }
                continue;
            }
        
            let type_info: Option<(&'static str, JsonTypeInfo)> =
                match &temp_cursor.to_string()[..] {
                    "options.requested-policy-version" => Some(("options.requestedPolicyVersion", JsonTypeInfo { jtype: JsonType::Int, ctype: ComplexType::Pod })),
                    _ => {
                        let suggestion = FieldCursor::did_you_mean(key, &vec!["options", "requested-policy-version"]);
                        err.issues.push(CLIError::Field(FieldError::Unknown(temp_cursor.to_string(), suggestion, value.map(|v| v.to_string()))));
                        None
                    }
                };
            if let Some((field_cursor_str, type_info)) = type_info {
                FieldCursor::from(field_cursor_str).set_json_value(&mut object, value.unwrap(), type_info, err, &temp_cursor);
            }
        }
        let mut request: api::GetIamPolicyRequest = json::value::from_value(object).unwrap();
        let mut call = self.hub.projects().occurrences_get_iam_policy(request, opt.value_of("resource").unwrap_or(""));
        for parg in opt.values_of("v").map(|i|i.collect()).unwrap_or(Vec::new()).iter() {
            let (key, value) = parse_kv_arg(&*parg, err, false);
            match key {
                _ => {
                    let mut found = false;
                    for param in &self.gp {
                        if key == *param {
                            found = true;
                            call = call.param(self.gpm.iter().find(|t| t.0 == key).unwrap_or(&("", key)).1, value.unwrap_or("unset"));
                            break;
                        }
                    }
                    if !found {
                        err.issues.push(CLIError::UnknownParameter(key.to_string(),
                                                                  {let mut v = Vec::new();
                                                                           v.extend(self.gp.iter().map(|v|*v));
                                                                           v } ));
                    }
                }
            }
        }
        let protocol = CallType::Standard;
        if dry_run {
            Ok(())
        } else {
            assert!(err.issues.len() == 0);
            for scope in self.opt.values_of("url").map(|i|i.collect()).unwrap_or(Vec::new()).iter() {
                call = call.add_scope(scope);
            }
            let mut ostream = match writer_from_opts(opt.value_of("out")) {
                Ok(mut f) => f,
                Err(io_err) => return Err(DoitError::IoError(opt.value_of("out").unwrap_or("-").to_string(), io_err)),
            };
            match match protocol {
                CallType::Standard => call.doit().await,
                _ => unreachable!()
            } {
                Err(api_err) => Err(DoitError::ApiError(api_err)),
                Ok((mut response, output_schema)) => {
                    let mut value = json::value::to_value(&output_schema).expect("serde to work");
                    remove_json_null_values(&mut value);
                    json::to_writer_pretty(&mut ostream, &value).unwrap();
                    ostream.flush().unwrap();
                    Ok(())
                }
            }
        }
    }

    async fn _projects_occurrences_get_notes(&self, opt: &ArgMatches<'n>, dry_run: bool, err: &mut InvalidOptionsError)
                                                    -> Result<(), DoitError> {
        let mut call = self.hub.projects().occurrences_get_notes(opt.value_of("name").unwrap_or(""));
        for parg in opt.values_of("v").map(|i|i.collect()).unwrap_or(Vec::new()).iter() {
            let (key, value) = parse_kv_arg(&*parg, err, false);
            match key {
                _ => {
                    let mut found = false;
                    for param in &self.gp {
                        if key == *param {
                            found = true;
                            call = call.param(self.gpm.iter().find(|t| t.0 == key).unwrap_or(&("", key)).1, value.unwrap_or("unset"));
                            break;
                        }
                    }
                    if !found {
                        err.issues.push(CLIError::UnknownParameter(key.to_string(),
                                                                  {let mut v = Vec::new();
                                                                           v.extend(self.gp.iter().map(|v|*v));
                                                                           v } ));
                    }
                }
            }
        }
        let protocol = CallType::Standard;
        if dry_run {
            Ok(())
        } else {
            assert!(err.issues.len() == 0);
            for scope in self.opt.values_of("url").map(|i|i.collect()).unwrap_or(Vec::new()).iter() {
                call = call.add_scope(scope);
            }
            let mut ostream = match writer_from_opts(opt.value_of("out")) {
                Ok(mut f) => f,
                Err(io_err) => return Err(DoitError::IoError(opt.value_of("out").unwrap_or("-").to_string(), io_err)),
            };
            match match protocol {
                CallType::Standard => call.doit().await,
                _ => unreachable!()
            } {
                Err(api_err) => Err(DoitError::ApiError(api_err)),
                Ok((mut response, output_schema)) => {
                    let mut value = json::value::to_value(&output_schema).expect("serde to work");
                    remove_json_null_values(&mut value);
                    json::to_writer_pretty(&mut ostream, &value).unwrap();
                    ostream.flush().unwrap();
                    Ok(())
                }
            }
        }
    }

    async fn _projects_occurrences_get_vulnerability_summary(&self, opt: &ArgMatches<'n>, dry_run: bool, err: &mut InvalidOptionsError)
                                                    -> Result<(), DoitError> {
        let mut call = self.hub.projects().occurrences_get_vulnerability_summary(opt.value_of("parent").unwrap_or(""));
        for parg in opt.values_of("v").map(|i|i.collect()).unwrap_or(Vec::new()).iter() {
            let (key, value) = parse_kv_arg(&*parg, err, false);
            match key {
                "filter" => {
                    call = call.filter(value.unwrap_or(""));
                },
                _ => {
                    let mut found = false;
                    for param in &self.gp {
                        if key == *param {
                            found = true;
                            call = call.param(self.gpm.iter().find(|t| t.0 == key).unwrap_or(&("", key)).1, value.unwrap_or("unset"));
                            break;
                        }
                    }
                    if !found {
                        err.issues.push(CLIError::UnknownParameter(key.to_string(),
                                                                  {let mut v = Vec::new();
                                                                           v.extend(self.gp.iter().map(|v|*v));
                                                                           v.extend(["filter"].iter().map(|v|*v));
                                                                           v } ));
                    }
                }
            }
        }
        let protocol = CallType::Standard;
        if dry_run {
            Ok(())
        } else {
            assert!(err.issues.len() == 0);
            for scope in self.opt.values_of("url").map(|i|i.collect()).unwrap_or(Vec::new()).iter() {
                call = call.add_scope(scope);
            }
            let mut ostream = match writer_from_opts(opt.value_of("out")) {
                Ok(mut f) => f,
                Err(io_err) => return Err(DoitError::IoError(opt.value_of("out").unwrap_or("-").to_string(), io_err)),
            };
            match match protocol {
                CallType::Standard => call.doit().await,
                _ => unreachable!()
            } {
                Err(api_err) => Err(DoitError::ApiError(api_err)),
                Ok((mut response, output_schema)) => {
                    let mut value = json::value::to_value(&output_schema).expect("serde to work");
                    remove_json_null_values(&mut value);
                    json::to_writer_pretty(&mut ostream, &value).unwrap();
                    ostream.flush().unwrap();
                    Ok(())
                }
            }
        }
    }

    async fn _projects_occurrences_list(&self, opt: &ArgMatches<'n>, dry_run: bool, err: &mut InvalidOptionsError)
                                                    -> Result<(), DoitError> {
        let mut call = self.hub.projects().occurrences_list(opt.value_of("parent").unwrap_or(""));
        for parg in opt.values_of("v").map(|i|i.collect()).unwrap_or(Vec::new()).iter() {
            let (key, value) = parse_kv_arg(&*parg, err, false);
            match key {
                "page-token" => {
                    call = call.page_token(value.unwrap_or(""));
                },
                "page-size" => {
                    call = call.page_size(arg_from_str(value.unwrap_or("-0"), err, "page-size", "integer"));
                },
                "filter" => {
                    call = call.filter(value.unwrap_or(""));
                },
                _ => {
                    let mut found = false;
                    for param in &self.gp {
                        if key == *param {
                            found = true;
                            call = call.param(self.gpm.iter().find(|t| t.0 == key).unwrap_or(&("", key)).1, value.unwrap_or("unset"));
                            break;
                        }
                    }
                    if !found {
                        err.issues.push(CLIError::UnknownParameter(key.to_string(),
                                                                  {let mut v = Vec::new();
                                                                           v.extend(self.gp.iter().map(|v|*v));
                                                                           v.extend(["filter", "page-size", "page-token"].iter().map(|v|*v));
                                                                           v } ));
                    }
                }
            }
        }
        let protocol = CallType::Standard;
        if dry_run {
            Ok(())
        } else {
            assert!(err.issues.len() == 0);
            for scope in self.opt.values_of("url").map(|i|i.collect()).unwrap_or(Vec::new()).iter() {
                call = call.add_scope(scope);
            }
            let mut ostream = match writer_from_opts(opt.value_of("out")) {
                Ok(mut f) => f,
                Err(io_err) => return Err(DoitError::IoError(opt.value_of("out").unwrap_or("-").to_string(), io_err)),
            };
            match match protocol {
                CallType::Standard => call.doit().await,
                _ => unreachable!()
            } {
                Err(api_err) => Err(DoitError::ApiError(api_err)),
                Ok((mut response, output_schema)) => {
                    let mut value = json::value::to_value(&output_schema).expect("serde to work");
                    remove_json_null_values(&mut value);
                    json::to_writer_pretty(&mut ostream, &value).unwrap();
                    ostream.flush().unwrap();
                    Ok(())
                }
            }
        }
    }

    async fn _projects_occurrences_patch(&self, opt: &ArgMatches<'n>, dry_run: bool, err: &mut InvalidOptionsError)
                                                    -> Result<(), DoitError> {
        
        let mut field_cursor = FieldCursor::default();
        let mut object = json::value::Value::Object(Default::default());
        
        for kvarg in opt.values_of("kv").map(|i|i.collect()).unwrap_or(Vec::new()).iter() {
            let last_errc = err.issues.len();
            let (key, value) = parse_kv_arg(&*kvarg, err, false);
            let mut temp_cursor = field_cursor.clone();
            if let Err(field_err) = temp_cursor.set(&*key) {
                err.issues.push(field_err);
            }
            if value.is_none() {
                field_cursor = temp_cursor.clone();
                if err.issues.len() > last_errc {
                    err.issues.remove(last_errc);
                }
                continue;
            }
        
            let type_info: Option<(&'static str, JsonTypeInfo)> =
                match &temp_cursor.to_string()[..] {
                    "attestation.attestation.generic-signed-attestation.content-type" => Some(("attestation.attestation.genericSignedAttestation.contentType", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "attestation.attestation.generic-signed-attestation.serialized-payload" => Some(("attestation.attestation.genericSignedAttestation.serializedPayload", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "attestation.attestation.pgp-signed-attestation.content-type" => Some(("attestation.attestation.pgpSignedAttestation.contentType", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "attestation.attestation.pgp-signed-attestation.pgp-key-id" => Some(("attestation.attestation.pgpSignedAttestation.pgpKeyId", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "attestation.attestation.pgp-signed-attestation.signature" => Some(("attestation.attestation.pgpSignedAttestation.signature", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "build.provenance.build-options" => Some(("build.provenance.buildOptions", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Map })),
                    "build.provenance.builder-version" => Some(("build.provenance.builderVersion", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "build.provenance.create-time" => Some(("build.provenance.createTime", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "build.provenance.creator" => Some(("build.provenance.creator", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "build.provenance.end-time" => Some(("build.provenance.endTime", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "build.provenance.id" => Some(("build.provenance.id", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "build.provenance.logs-uri" => Some(("build.provenance.logsUri", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "build.provenance.project-id" => Some(("build.provenance.projectId", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "build.provenance.source-provenance.artifact-storage-source-uri" => Some(("build.provenance.sourceProvenance.artifactStorageSourceUri", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "build.provenance.source-provenance.context.cloud-repo.alias-context.kind" => Some(("build.provenance.sourceProvenance.context.cloudRepo.aliasContext.kind", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "build.provenance.source-provenance.context.cloud-repo.alias-context.name" => Some(("build.provenance.sourceProvenance.context.cloudRepo.aliasContext.name", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "build.provenance.source-provenance.context.cloud-repo.repo-id.project-repo-id.project-id" => Some(("build.provenance.sourceProvenance.context.cloudRepo.repoId.projectRepoId.projectId", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "build.provenance.source-provenance.context.cloud-repo.repo-id.project-repo-id.repo-name" => Some(("build.provenance.sourceProvenance.context.cloudRepo.repoId.projectRepoId.repoName", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "build.provenance.source-provenance.context.cloud-repo.repo-id.uid" => Some(("build.provenance.sourceProvenance.context.cloudRepo.repoId.uid", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "build.provenance.source-provenance.context.cloud-repo.revision-id" => Some(("build.provenance.sourceProvenance.context.cloudRepo.revisionId", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "build.provenance.source-provenance.context.gerrit.alias-context.kind" => Some(("build.provenance.sourceProvenance.context.gerrit.aliasContext.kind", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "build.provenance.source-provenance.context.gerrit.alias-context.name" => Some(("build.provenance.sourceProvenance.context.gerrit.aliasContext.name", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "build.provenance.source-provenance.context.gerrit.gerrit-project" => Some(("build.provenance.sourceProvenance.context.gerrit.gerritProject", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "build.provenance.source-provenance.context.gerrit.host-uri" => Some(("build.provenance.sourceProvenance.context.gerrit.hostUri", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "build.provenance.source-provenance.context.gerrit.revision-id" => Some(("build.provenance.sourceProvenance.context.gerrit.revisionId", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "build.provenance.source-provenance.context.git.revision-id" => Some(("build.provenance.sourceProvenance.context.git.revisionId", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "build.provenance.source-provenance.context.git.url" => Some(("build.provenance.sourceProvenance.context.git.url", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "build.provenance.source-provenance.context.labels" => Some(("build.provenance.sourceProvenance.context.labels", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Map })),
                    "build.provenance.start-time" => Some(("build.provenance.startTime", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "build.provenance.trigger-id" => Some(("build.provenance.triggerId", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "build.provenance-bytes" => Some(("build.provenanceBytes", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "create-time" => Some(("createTime", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "deployment.deployment.address" => Some(("deployment.deployment.address", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "deployment.deployment.config" => Some(("deployment.deployment.config", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "deployment.deployment.deploy-time" => Some(("deployment.deployment.deployTime", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "deployment.deployment.platform" => Some(("deployment.deployment.platform", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "deployment.deployment.resource-uri" => Some(("deployment.deployment.resourceUri", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Vec })),
                    "deployment.deployment.undeploy-time" => Some(("deployment.deployment.undeployTime", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "deployment.deployment.user-email" => Some(("deployment.deployment.userEmail", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "derived-image.derived-image.base-resource-url" => Some(("derivedImage.derivedImage.baseResourceUrl", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "derived-image.derived-image.distance" => Some(("derivedImage.derivedImage.distance", JsonTypeInfo { jtype: JsonType::Int, ctype: ComplexType::Pod })),
                    "derived-image.derived-image.fingerprint.v1-name" => Some(("derivedImage.derivedImage.fingerprint.v1Name", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "derived-image.derived-image.fingerprint.v2-blob" => Some(("derivedImage.derivedImage.fingerprint.v2Blob", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Vec })),
                    "derived-image.derived-image.fingerprint.v2-name" => Some(("derivedImage.derivedImage.fingerprint.v2Name", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "discovered.discovered.analysis-status" => Some(("discovered.discovered.analysisStatus", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "discovered.discovered.analysis-status-error.code" => Some(("discovered.discovered.analysisStatusError.code", JsonTypeInfo { jtype: JsonType::Int, ctype: ComplexType::Pod })),
                    "discovered.discovered.analysis-status-error.message" => Some(("discovered.discovered.analysisStatusError.message", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "discovered.discovered.continuous-analysis" => Some(("discovered.discovered.continuousAnalysis", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "discovered.discovered.last-analysis-time" => Some(("discovered.discovered.lastAnalysisTime", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "installation.installation.name" => Some(("installation.installation.name", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "intoto.signed.byproducts.custom-values" => Some(("intoto.signed.byproducts.customValues", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Map })),
                    "intoto.signed.command" => Some(("intoto.signed.command", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Vec })),
                    "intoto.signed.environment.custom-values" => Some(("intoto.signed.environment.customValues", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Map })),
                    "kind" => Some(("kind", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "name" => Some(("name", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "note-name" => Some(("noteName", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "remediation" => Some(("remediation", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "resource.content-hash.type" => Some(("resource.contentHash.type", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "resource.content-hash.value" => Some(("resource.contentHash.value", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "resource.name" => Some(("resource.name", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "resource.uri" => Some(("resource.uri", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "update-time" => Some(("updateTime", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "vulnerability.cvss-score" => Some(("vulnerability.cvssScore", JsonTypeInfo { jtype: JsonType::Float, ctype: ComplexType::Pod })),
                    "vulnerability.effective-severity" => Some(("vulnerability.effectiveSeverity", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "vulnerability.long-description" => Some(("vulnerability.longDescription", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "vulnerability.severity" => Some(("vulnerability.severity", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "vulnerability.short-description" => Some(("vulnerability.shortDescription", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "vulnerability.type" => Some(("vulnerability.type", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    _ => {
                        let suggestion = FieldCursor::did_you_mean(key, &vec!["address", "alias-context", "analysis-status", "analysis-status-error", "artifact-storage-source-uri", "attestation", "base-resource-url", "build", "build-options", "builder-version", "byproducts", "cloud-repo", "code", "command", "config", "content-hash", "content-type", "context", "continuous-analysis", "create-time", "creator", "custom-values", "cvss-score", "deploy-time", "deployment", "derived-image", "discovered", "distance", "effective-severity", "end-time", "environment", "fingerprint", "generic-signed-attestation", "gerrit", "gerrit-project", "git", "host-uri", "id", "installation", "intoto", "kind", "labels", "last-analysis-time", "logs-uri", "long-description", "message", "name", "note-name", "pgp-key-id", "pgp-signed-attestation", "platform", "project-id", "project-repo-id", "provenance", "provenance-bytes", "remediation", "repo-id", "repo-name", "resource", "resource-uri", "revision-id", "serialized-payload", "severity", "short-description", "signature", "signed", "source-provenance", "start-time", "trigger-id", "type", "uid", "undeploy-time", "update-time", "uri", "url", "user-email", "v1-name", "v2-blob", "v2-name", "value", "vulnerability"]);
                        err.issues.push(CLIError::Field(FieldError::Unknown(temp_cursor.to_string(), suggestion, value.map(|v| v.to_string()))));
                        None
                    }
                };
            if let Some((field_cursor_str, type_info)) = type_info {
                FieldCursor::from(field_cursor_str).set_json_value(&mut object, value.unwrap(), type_info, err, &temp_cursor);
            }
        }
        let mut request: api::Occurrence = json::value::from_value(object).unwrap();
        let mut call = self.hub.projects().occurrences_patch(request, opt.value_of("name").unwrap_or(""));
        for parg in opt.values_of("v").map(|i|i.collect()).unwrap_or(Vec::new()).iter() {
            let (key, value) = parse_kv_arg(&*parg, err, false);
            match key {
                "update-mask" => {
                    call = call.update_mask(value.unwrap_or(""));
                },
                _ => {
                    let mut found = false;
                    for param in &self.gp {
                        if key == *param {
                            found = true;
                            call = call.param(self.gpm.iter().find(|t| t.0 == key).unwrap_or(&("", key)).1, value.unwrap_or("unset"));
                            break;
                        }
                    }
                    if !found {
                        err.issues.push(CLIError::UnknownParameter(key.to_string(),
                                                                  {let mut v = Vec::new();
                                                                           v.extend(self.gp.iter().map(|v|*v));
                                                                           v.extend(["update-mask"].iter().map(|v|*v));
                                                                           v } ));
                    }
                }
            }
        }
        let protocol = CallType::Standard;
        if dry_run {
            Ok(())
        } else {
            assert!(err.issues.len() == 0);
            for scope in self.opt.values_of("url").map(|i|i.collect()).unwrap_or(Vec::new()).iter() {
                call = call.add_scope(scope);
            }
            let mut ostream = match writer_from_opts(opt.value_of("out")) {
                Ok(mut f) => f,
                Err(io_err) => return Err(DoitError::IoError(opt.value_of("out").unwrap_or("-").to_string(), io_err)),
            };
            match match protocol {
                CallType::Standard => call.doit().await,
                _ => unreachable!()
            } {
                Err(api_err) => Err(DoitError::ApiError(api_err)),
                Ok((mut response, output_schema)) => {
                    let mut value = json::value::to_value(&output_schema).expect("serde to work");
                    remove_json_null_values(&mut value);
                    json::to_writer_pretty(&mut ostream, &value).unwrap();
                    ostream.flush().unwrap();
                    Ok(())
                }
            }
        }
    }

    async fn _projects_occurrences_set_iam_policy(&self, opt: &ArgMatches<'n>, dry_run: bool, err: &mut InvalidOptionsError)
                                                    -> Result<(), DoitError> {
        
        let mut field_cursor = FieldCursor::default();
        let mut object = json::value::Value::Object(Default::default());
        
        for kvarg in opt.values_of("kv").map(|i|i.collect()).unwrap_or(Vec::new()).iter() {
            let last_errc = err.issues.len();
            let (key, value) = parse_kv_arg(&*kvarg, err, false);
            let mut temp_cursor = field_cursor.clone();
            if let Err(field_err) = temp_cursor.set(&*key) {
                err.issues.push(field_err);
            }
            if value.is_none() {
                field_cursor = temp_cursor.clone();
                if err.issues.len() > last_errc {
                    err.issues.remove(last_errc);
                }
                continue;
            }
        
            let type_info: Option<(&'static str, JsonTypeInfo)> =
                match &temp_cursor.to_string()[..] {
                    "policy.etag" => Some(("policy.etag", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "policy.version" => Some(("policy.version", JsonTypeInfo { jtype: JsonType::Int, ctype: ComplexType::Pod })),
                    _ => {
                        let suggestion = FieldCursor::did_you_mean(key, &vec!["etag", "policy", "version"]);
                        err.issues.push(CLIError::Field(FieldError::Unknown(temp_cursor.to_string(), suggestion, value.map(|v| v.to_string()))));
                        None
                    }
                };
            if let Some((field_cursor_str, type_info)) = type_info {
                FieldCursor::from(field_cursor_str).set_json_value(&mut object, value.unwrap(), type_info, err, &temp_cursor);
            }
        }
        let mut request: api::SetIamPolicyRequest = json::value::from_value(object).unwrap();
        let mut call = self.hub.projects().occurrences_set_iam_policy(request, opt.value_of("resource").unwrap_or(""));
        for parg in opt.values_of("v").map(|i|i.collect()).unwrap_or(Vec::new()).iter() {
            let (key, value) = parse_kv_arg(&*parg, err, false);
            match key {
                _ => {
                    let mut found = false;
                    for param in &self.gp {
                        if key == *param {
                            found = true;
                            call = call.param(self.gpm.iter().find(|t| t.0 == key).unwrap_or(&("", key)).1, value.unwrap_or("unset"));
                            break;
                        }
                    }
                    if !found {
                        err.issues.push(CLIError::UnknownParameter(key.to_string(),
                                                                  {let mut v = Vec::new();
                                                                           v.extend(self.gp.iter().map(|v|*v));
                                                                           v } ));
                    }
                }
            }
        }
        let protocol = CallType::Standard;
        if dry_run {
            Ok(())
        } else {
            assert!(err.issues.len() == 0);
            for scope in self.opt.values_of("url").map(|i|i.collect()).unwrap_or(Vec::new()).iter() {
                call = call.add_scope(scope);
            }
            let mut ostream = match writer_from_opts(opt.value_of("out")) {
                Ok(mut f) => f,
                Err(io_err) => return Err(DoitError::IoError(opt.value_of("out").unwrap_or("-").to_string(), io_err)),
            };
            match match protocol {
                CallType::Standard => call.doit().await,
                _ => unreachable!()
            } {
                Err(api_err) => Err(DoitError::ApiError(api_err)),
                Ok((mut response, output_schema)) => {
                    let mut value = json::value::to_value(&output_schema).expect("serde to work");
                    remove_json_null_values(&mut value);
                    json::to_writer_pretty(&mut ostream, &value).unwrap();
                    ostream.flush().unwrap();
                    Ok(())
                }
            }
        }
    }

    async fn _projects_occurrences_test_iam_permissions(&self, opt: &ArgMatches<'n>, dry_run: bool, err: &mut InvalidOptionsError)
                                                    -> Result<(), DoitError> {
        
        let mut field_cursor = FieldCursor::default();
        let mut object = json::value::Value::Object(Default::default());
        
        for kvarg in opt.values_of("kv").map(|i|i.collect()).unwrap_or(Vec::new()).iter() {
            let last_errc = err.issues.len();
            let (key, value) = parse_kv_arg(&*kvarg, err, false);
            let mut temp_cursor = field_cursor.clone();
            if let Err(field_err) = temp_cursor.set(&*key) {
                err.issues.push(field_err);
            }
            if value.is_none() {
                field_cursor = temp_cursor.clone();
                if err.issues.len() > last_errc {
                    err.issues.remove(last_errc);
                }
                continue;
            }
        
            let type_info: Option<(&'static str, JsonTypeInfo)> =
                match &temp_cursor.to_string()[..] {
                    "permissions" => Some(("permissions", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Vec })),
                    _ => {
                        let suggestion = FieldCursor::did_you_mean(key, &vec!["permissions"]);
                        err.issues.push(CLIError::Field(FieldError::Unknown(temp_cursor.to_string(), suggestion, value.map(|v| v.to_string()))));
                        None
                    }
                };
            if let Some((field_cursor_str, type_info)) = type_info {
                FieldCursor::from(field_cursor_str).set_json_value(&mut object, value.unwrap(), type_info, err, &temp_cursor);
            }
        }
        let mut request: api::TestIamPermissionsRequest = json::value::from_value(object).unwrap();
        let mut call = self.hub.projects().occurrences_test_iam_permissions(request, opt.value_of("resource").unwrap_or(""));
        for parg in opt.values_of("v").map(|i|i.collect()).unwrap_or(Vec::new()).iter() {
            let (key, value) = parse_kv_arg(&*parg, err, false);
            match key {
                _ => {
                    let mut found = false;
                    for param in &self.gp {
                        if key == *param {
                            found = true;
                            call = call.param(self.gpm.iter().find(|t| t.0 == key).unwrap_or(&("", key)).1, value.unwrap_or("unset"));
                            break;
                        }
                    }
                    if !found {
                        err.issues.push(CLIError::UnknownParameter(key.to_string(),
                                                                  {let mut v = Vec::new();
                                                                           v.extend(self.gp.iter().map(|v|*v));
                                                                           v } ));
                    }
                }
            }
        }
        let protocol = CallType::Standard;
        if dry_run {
            Ok(())
        } else {
            assert!(err.issues.len() == 0);
            for scope in self.opt.values_of("url").map(|i|i.collect()).unwrap_or(Vec::new()).iter() {
                call = call.add_scope(scope);
            }
            let mut ostream = match writer_from_opts(opt.value_of("out")) {
                Ok(mut f) => f,
                Err(io_err) => return Err(DoitError::IoError(opt.value_of("out").unwrap_or("-").to_string(), io_err)),
            };
            match match protocol {
                CallType::Standard => call.doit().await,
                _ => unreachable!()
            } {
                Err(api_err) => Err(DoitError::ApiError(api_err)),
                Ok((mut response, output_schema)) => {
                    let mut value = json::value::to_value(&output_schema).expect("serde to work");
                    remove_json_null_values(&mut value);
                    json::to_writer_pretty(&mut ostream, &value).unwrap();
                    ostream.flush().unwrap();
                    Ok(())
                }
            }
        }
    }

    async fn _projects_scan_configs_get(&self, opt: &ArgMatches<'n>, dry_run: bool, err: &mut InvalidOptionsError)
                                                    -> Result<(), DoitError> {
        let mut call = self.hub.projects().scan_configs_get(opt.value_of("name").unwrap_or(""));
        for parg in opt.values_of("v").map(|i|i.collect()).unwrap_or(Vec::new()).iter() {
            let (key, value) = parse_kv_arg(&*parg, err, false);
            match key {
                _ => {
                    let mut found = false;
                    for param in &self.gp {
                        if key == *param {
                            found = true;
                            call = call.param(self.gpm.iter().find(|t| t.0 == key).unwrap_or(&("", key)).1, value.unwrap_or("unset"));
                            break;
                        }
                    }
                    if !found {
                        err.issues.push(CLIError::UnknownParameter(key.to_string(),
                                                                  {let mut v = Vec::new();
                                                                           v.extend(self.gp.iter().map(|v|*v));
                                                                           v } ));
                    }
                }
            }
        }
        let protocol = CallType::Standard;
        if dry_run {
            Ok(())
        } else {
            assert!(err.issues.len() == 0);
            for scope in self.opt.values_of("url").map(|i|i.collect()).unwrap_or(Vec::new()).iter() {
                call = call.add_scope(scope);
            }
            let mut ostream = match writer_from_opts(opt.value_of("out")) {
                Ok(mut f) => f,
                Err(io_err) => return Err(DoitError::IoError(opt.value_of("out").unwrap_or("-").to_string(), io_err)),
            };
            match match protocol {
                CallType::Standard => call.doit().await,
                _ => unreachable!()
            } {
                Err(api_err) => Err(DoitError::ApiError(api_err)),
                Ok((mut response, output_schema)) => {
                    let mut value = json::value::to_value(&output_schema).expect("serde to work");
                    remove_json_null_values(&mut value);
                    json::to_writer_pretty(&mut ostream, &value).unwrap();
                    ostream.flush().unwrap();
                    Ok(())
                }
            }
        }
    }

    async fn _projects_scan_configs_list(&self, opt: &ArgMatches<'n>, dry_run: bool, err: &mut InvalidOptionsError)
                                                    -> Result<(), DoitError> {
        let mut call = self.hub.projects().scan_configs_list(opt.value_of("parent").unwrap_or(""));
        for parg in opt.values_of("v").map(|i|i.collect()).unwrap_or(Vec::new()).iter() {
            let (key, value) = parse_kv_arg(&*parg, err, false);
            match key {
                "page-token" => {
                    call = call.page_token(value.unwrap_or(""));
                },
                "page-size" => {
                    call = call.page_size(arg_from_str(value.unwrap_or("-0"), err, "page-size", "integer"));
                },
                "filter" => {
                    call = call.filter(value.unwrap_or(""));
                },
                _ => {
                    let mut found = false;
                    for param in &self.gp {
                        if key == *param {
                            found = true;
                            call = call.param(self.gpm.iter().find(|t| t.0 == key).unwrap_or(&("", key)).1, value.unwrap_or("unset"));
                            break;
                        }
                    }
                    if !found {
                        err.issues.push(CLIError::UnknownParameter(key.to_string(),
                                                                  {let mut v = Vec::new();
                                                                           v.extend(self.gp.iter().map(|v|*v));
                                                                           v.extend(["filter", "page-size", "page-token"].iter().map(|v|*v));
                                                                           v } ));
                    }
                }
            }
        }
        let protocol = CallType::Standard;
        if dry_run {
            Ok(())
        } else {
            assert!(err.issues.len() == 0);
            for scope in self.opt.values_of("url").map(|i|i.collect()).unwrap_or(Vec::new()).iter() {
                call = call.add_scope(scope);
            }
            let mut ostream = match writer_from_opts(opt.value_of("out")) {
                Ok(mut f) => f,
                Err(io_err) => return Err(DoitError::IoError(opt.value_of("out").unwrap_or("-").to_string(), io_err)),
            };
            match match protocol {
                CallType::Standard => call.doit().await,
                _ => unreachable!()
            } {
                Err(api_err) => Err(DoitError::ApiError(api_err)),
                Ok((mut response, output_schema)) => {
                    let mut value = json::value::to_value(&output_schema).expect("serde to work");
                    remove_json_null_values(&mut value);
                    json::to_writer_pretty(&mut ostream, &value).unwrap();
                    ostream.flush().unwrap();
                    Ok(())
                }
            }
        }
    }

    async fn _projects_scan_configs_update(&self, opt: &ArgMatches<'n>, dry_run: bool, err: &mut InvalidOptionsError)
                                                    -> Result<(), DoitError> {
        
        let mut field_cursor = FieldCursor::default();
        let mut object = json::value::Value::Object(Default::default());
        
        for kvarg in opt.values_of("kv").map(|i|i.collect()).unwrap_or(Vec::new()).iter() {
            let last_errc = err.issues.len();
            let (key, value) = parse_kv_arg(&*kvarg, err, false);
            let mut temp_cursor = field_cursor.clone();
            if let Err(field_err) = temp_cursor.set(&*key) {
                err.issues.push(field_err);
            }
            if value.is_none() {
                field_cursor = temp_cursor.clone();
                if err.issues.len() > last_errc {
                    err.issues.remove(last_errc);
                }
                continue;
            }
        
            let type_info: Option<(&'static str, JsonTypeInfo)> =
                match &temp_cursor.to_string()[..] {
                    "create-time" => Some(("createTime", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "description" => Some(("description", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "enabled" => Some(("enabled", JsonTypeInfo { jtype: JsonType::Boolean, ctype: ComplexType::Pod })),
                    "name" => Some(("name", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "update-time" => Some(("updateTime", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    _ => {
                        let suggestion = FieldCursor::did_you_mean(key, &vec!["create-time", "description", "enabled", "name", "update-time"]);
                        err.issues.push(CLIError::Field(FieldError::Unknown(temp_cursor.to_string(), suggestion, value.map(|v| v.to_string()))));
                        None
                    }
                };
            if let Some((field_cursor_str, type_info)) = type_info {
                FieldCursor::from(field_cursor_str).set_json_value(&mut object, value.unwrap(), type_info, err, &temp_cursor);
            }
        }
        let mut request: api::ScanConfig = json::value::from_value(object).unwrap();
        let mut call = self.hub.projects().scan_configs_update(request, opt.value_of("name").unwrap_or(""));
        for parg in opt.values_of("v").map(|i|i.collect()).unwrap_or(Vec::new()).iter() {
            let (key, value) = parse_kv_arg(&*parg, err, false);
            match key {
                _ => {
                    let mut found = false;
                    for param in &self.gp {
                        if key == *param {
                            found = true;
                            call = call.param(self.gpm.iter().find(|t| t.0 == key).unwrap_or(&("", key)).1, value.unwrap_or("unset"));
                            break;
                        }
                    }
                    if !found {
                        err.issues.push(CLIError::UnknownParameter(key.to_string(),
                                                                  {let mut v = Vec::new();
                                                                           v.extend(self.gp.iter().map(|v|*v));
                                                                           v } ));
                    }
                }
            }
        }
        let protocol = CallType::Standard;
        if dry_run {
            Ok(())
        } else {
            assert!(err.issues.len() == 0);
            for scope in self.opt.values_of("url").map(|i|i.collect()).unwrap_or(Vec::new()).iter() {
                call = call.add_scope(scope);
            }
            let mut ostream = match writer_from_opts(opt.value_of("out")) {
                Ok(mut f) => f,
                Err(io_err) => return Err(DoitError::IoError(opt.value_of("out").unwrap_or("-").to_string(), io_err)),
            };
            match match protocol {
                CallType::Standard => call.doit().await,
                _ => unreachable!()
            } {
                Err(api_err) => Err(DoitError::ApiError(api_err)),
                Ok((mut response, output_schema)) => {
                    let mut value = json::value::to_value(&output_schema).expect("serde to work");
                    remove_json_null_values(&mut value);
                    json::to_writer_pretty(&mut ostream, &value).unwrap();
                    ostream.flush().unwrap();
                    Ok(())
                }
            }
        }
    }

    async fn _doit(&self, dry_run: bool) -> Result<Result<(), DoitError>, Option<InvalidOptionsError>> {
        let mut err = InvalidOptionsError::new();
        let mut call_result: Result<(), DoitError> = Ok(());
        let mut err_opt: Option<InvalidOptionsError> = None;
        match self.opt.subcommand() {
            ("projects", Some(opt)) => {
                match opt.subcommand() {
                    ("notes-batch-create", Some(opt)) => {
                        call_result = self._projects_notes_batch_create(opt, dry_run, &mut err).await;
                    },
                    ("notes-create", Some(opt)) => {
                        call_result = self._projects_notes_create(opt, dry_run, &mut err).await;
                    },
                    ("notes-delete", Some(opt)) => {
                        call_result = self._projects_notes_delete(opt, dry_run, &mut err).await;
                    },
                    ("notes-get", Some(opt)) => {
                        call_result = self._projects_notes_get(opt, dry_run, &mut err).await;
                    },
                    ("notes-get-iam-policy", Some(opt)) => {
                        call_result = self._projects_notes_get_iam_policy(opt, dry_run, &mut err).await;
                    },
                    ("notes-list", Some(opt)) => {
                        call_result = self._projects_notes_list(opt, dry_run, &mut err).await;
                    },
                    ("notes-occurrences-list", Some(opt)) => {
                        call_result = self._projects_notes_occurrences_list(opt, dry_run, &mut err).await;
                    },
                    ("notes-patch", Some(opt)) => {
                        call_result = self._projects_notes_patch(opt, dry_run, &mut err).await;
                    },
                    ("notes-set-iam-policy", Some(opt)) => {
                        call_result = self._projects_notes_set_iam_policy(opt, dry_run, &mut err).await;
                    },
                    ("notes-test-iam-permissions", Some(opt)) => {
                        call_result = self._projects_notes_test_iam_permissions(opt, dry_run, &mut err).await;
                    },
                    ("occurrences-batch-create", Some(opt)) => {
                        call_result = self._projects_occurrences_batch_create(opt, dry_run, &mut err).await;
                    },
                    ("occurrences-create", Some(opt)) => {
                        call_result = self._projects_occurrences_create(opt, dry_run, &mut err).await;
                    },
                    ("occurrences-delete", Some(opt)) => {
                        call_result = self._projects_occurrences_delete(opt, dry_run, &mut err).await;
                    },
                    ("occurrences-get", Some(opt)) => {
                        call_result = self._projects_occurrences_get(opt, dry_run, &mut err).await;
                    },
                    ("occurrences-get-iam-policy", Some(opt)) => {
                        call_result = self._projects_occurrences_get_iam_policy(opt, dry_run, &mut err).await;
                    },
                    ("occurrences-get-notes", Some(opt)) => {
                        call_result = self._projects_occurrences_get_notes(opt, dry_run, &mut err).await;
                    },
                    ("occurrences-get-vulnerability-summary", Some(opt)) => {
                        call_result = self._projects_occurrences_get_vulnerability_summary(opt, dry_run, &mut err).await;
                    },
                    ("occurrences-list", Some(opt)) => {
                        call_result = self._projects_occurrences_list(opt, dry_run, &mut err).await;
                    },
                    ("occurrences-patch", Some(opt)) => {
                        call_result = self._projects_occurrences_patch(opt, dry_run, &mut err).await;
                    },
                    ("occurrences-set-iam-policy", Some(opt)) => {
                        call_result = self._projects_occurrences_set_iam_policy(opt, dry_run, &mut err).await;
                    },
                    ("occurrences-test-iam-permissions", Some(opt)) => {
                        call_result = self._projects_occurrences_test_iam_permissions(opt, dry_run, &mut err).await;
                    },
                    ("scan-configs-get", Some(opt)) => {
                        call_result = self._projects_scan_configs_get(opt, dry_run, &mut err).await;
                    },
                    ("scan-configs-list", Some(opt)) => {
                        call_result = self._projects_scan_configs_list(opt, dry_run, &mut err).await;
                    },
                    ("scan-configs-update", Some(opt)) => {
                        call_result = self._projects_scan_configs_update(opt, dry_run, &mut err).await;
                    },
                    _ => {
                        err.issues.push(CLIError::MissingMethodError("projects".to_string()));
                        writeln!(io::stderr(), "{}\n", opt.usage()).ok();
                    }
                }
            },
            _ => {
                err.issues.push(CLIError::MissingCommandError);
                writeln!(io::stderr(), "{}\n", self.opt.usage()).ok();
            }
        }

        if dry_run {
            if err.issues.len() > 0 {
                err_opt = Some(err);
            }
            Err(err_opt)
        } else {
            Ok(call_result)
        }
    }

    // Please note that this call will fail if any part of the opt can't be handled
    async fn new(opt: ArgMatches<'n>) -> Result<Engine<'n>, InvalidOptionsError> {
        let (config_dir, secret) = {
            let config_dir = match client::assure_config_dir_exists(opt.value_of("folder").unwrap_or("~/.google-service-cli")) {
                Err(e) => return Err(InvalidOptionsError::single(e, 3)),
                Ok(p) => p,
            };

            match client::application_secret_from_directory(&config_dir, "containeranalysis1-beta1-secret.json",
                                                         "{\"installed\":{\"auth_uri\":\"https://accounts.google.com/o/oauth2/auth\",\"client_secret\":\"hCsslbCUyfehWMmbkG8vTYxG\",\"token_uri\":\"https://accounts.google.com/o/oauth2/token\",\"client_email\":\"\",\"redirect_uris\":[\"urn:ietf:wg:oauth:2.0:oob\",\"oob\"],\"client_x509_cert_url\":\"\",\"client_id\":\"620010449518-9ngf7o4dhs0dka470npqvor6dc5lqb9b.apps.googleusercontent.com\",\"auth_provider_x509_cert_url\":\"https://www.googleapis.com/oauth2/v1/certs\"}}") {
                Ok(secret) => (config_dir, secret),
                Err(e) => return Err(InvalidOptionsError::single(e, 4))
            }
        };

        let auth = yup_oauth2::InstalledFlowAuthenticator::builder(
            secret,
            yup_oauth2::InstalledFlowReturnMethod::HTTPRedirect,
        ).persist_tokens_to_disk(format!("{}/containeranalysis1-beta1", config_dir)).build().await.unwrap();

        let client = hyper::Client::builder().build(hyper_rustls::HttpsConnector::with_native_roots());
        let engine = Engine {
            opt: opt,
            hub: api::ContainerAnalysis::new(client, auth),
            gp: vec!["$-xgafv", "access-token", "alt", "callback", "fields", "key", "oauth-token", "pretty-print", "quota-user", "upload-type", "upload-protocol"],
            gpm: vec![
                    ("$-xgafv", "$.xgafv"),
                    ("access-token", "access_token"),
                    ("oauth-token", "oauth_token"),
                    ("pretty-print", "prettyPrint"),
                    ("quota-user", "quotaUser"),
                    ("upload-type", "uploadType"),
                    ("upload-protocol", "upload_protocol"),
                ]
        };

        match engine._doit(true).await {
            Err(Some(err)) => Err(err),
            Err(None)      => Ok(engine),
            Ok(_)          => unreachable!(),
        }
    }

    async fn doit(&self) -> Result<(), DoitError> {
        match self._doit(false).await {
            Ok(res) => res,
            Err(_) => unreachable!(),
        }
    }
}

#[tokio::main]
async fn main() {
    let mut exit_status = 0i32;
    let arg_data = [
        ("projects", "methods: 'notes-batch-create', 'notes-create', 'notes-delete', 'notes-get', 'notes-get-iam-policy', 'notes-list', 'notes-occurrences-list', 'notes-patch', 'notes-set-iam-policy', 'notes-test-iam-permissions', 'occurrences-batch-create', 'occurrences-create', 'occurrences-delete', 'occurrences-get', 'occurrences-get-iam-policy', 'occurrences-get-notes', 'occurrences-get-vulnerability-summary', 'occurrences-list', 'occurrences-patch', 'occurrences-set-iam-policy', 'occurrences-test-iam-permissions', 'scan-configs-get', 'scan-configs-list' and 'scan-configs-update'", vec![
            ("notes-batch-create",
                    Some(r##"Creates new notes in batch."##),
                    "Details at http://byron.github.io/google-apis-rs/google_containeranalysis1_beta1_cli/projects_notes-batch-create",
                  vec![
                    (Some(r##"parent"##),
                     None,
                     Some(r##"Required. The name of the project in the form of `projects/[PROJECT_ID]`, under which the notes are to be created."##),
                     Some(true),
                     Some(false)),
        
                    (Some(r##"kv"##),
                     Some(r##"r"##),
                     Some(r##"Set various fields of the request structure, matching the key=value form"##),
                     Some(true),
                     Some(true)),
        
                    (Some(r##"v"##),
                     Some(r##"p"##),
                     Some(r##"Set various optional parameters, matching the key=value form"##),
                     Some(false),
                     Some(true)),
        
                    (Some(r##"out"##),
                     Some(r##"o"##),
                     Some(r##"Specify the file into which to write the program's output"##),
                     Some(false),
                     Some(false)),
                  ]),
            ("notes-create",
                    Some(r##"Creates a new note."##),
                    "Details at http://byron.github.io/google-apis-rs/google_containeranalysis1_beta1_cli/projects_notes-create",
                  vec![
                    (Some(r##"parent"##),
                     None,
                     Some(r##"Required. The name of the project in the form of `projects/[PROJECT_ID]`, under which the note is to be created."##),
                     Some(true),
                     Some(false)),
        
                    (Some(r##"kv"##),
                     Some(r##"r"##),
                     Some(r##"Set various fields of the request structure, matching the key=value form"##),
                     Some(true),
                     Some(true)),
        
                    (Some(r##"v"##),
                     Some(r##"p"##),
                     Some(r##"Set various optional parameters, matching the key=value form"##),
                     Some(false),
                     Some(true)),
        
                    (Some(r##"out"##),
                     Some(r##"o"##),
                     Some(r##"Specify the file into which to write the program's output"##),
                     Some(false),
                     Some(false)),
                  ]),
            ("notes-delete",
                    Some(r##"Deletes the specified note."##),
                    "Details at http://byron.github.io/google-apis-rs/google_containeranalysis1_beta1_cli/projects_notes-delete",
                  vec![
                    (Some(r##"name"##),
                     None,
                     Some(r##"Required. The name of the note in the form of `projects/[PROVIDER_ID]/notes/[NOTE_ID]`."##),
                     Some(true),
                     Some(false)),
        
                    (Some(r##"v"##),
                     Some(r##"p"##),
                     Some(r##"Set various optional parameters, matching the key=value form"##),
                     Some(false),
                     Some(true)),
        
                    (Some(r##"out"##),
                     Some(r##"o"##),
                     Some(r##"Specify the file into which to write the program's output"##),
                     Some(false),
                     Some(false)),
                  ]),
            ("notes-get",
                    Some(r##"Gets the specified note."##),
                    "Details at http://byron.github.io/google-apis-rs/google_containeranalysis1_beta1_cli/projects_notes-get",
                  vec![
                    (Some(r##"name"##),
                     None,
                     Some(r##"Required. The name of the note in the form of `projects/[PROVIDER_ID]/notes/[NOTE_ID]`."##),
                     Some(true),
                     Some(false)),
        
                    (Some(r##"v"##),
                     Some(r##"p"##),
                     Some(r##"Set various optional parameters, matching the key=value form"##),
                     Some(false),
                     Some(true)),
        
                    (Some(r##"out"##),
                     Some(r##"o"##),
                     Some(r##"Specify the file into which to write the program's output"##),
                     Some(false),
                     Some(false)),
                  ]),
            ("notes-get-iam-policy",
                    Some(r##"Gets the access control policy for a note or an occurrence resource. Requires `containeranalysis.notes.setIamPolicy` or `containeranalysis.occurrences.setIamPolicy` permission if the resource is a note or occurrence, respectively. The resource takes the format `projects/[PROJECT_ID]/notes/[NOTE_ID]` for notes and `projects/[PROJECT_ID]/occurrences/[OCCURRENCE_ID]` for occurrences."##),
                    "Details at http://byron.github.io/google-apis-rs/google_containeranalysis1_beta1_cli/projects_notes-get-iam-policy",
                  vec![
                    (Some(r##"resource"##),
                     None,
                     Some(r##"REQUIRED: The resource for which the policy is being requested. See the operation documentation for the appropriate value for this field."##),
                     Some(true),
                     Some(false)),
        
                    (Some(r##"kv"##),
                     Some(r##"r"##),
                     Some(r##"Set various fields of the request structure, matching the key=value form"##),
                     Some(true),
                     Some(true)),
        
                    (Some(r##"v"##),
                     Some(r##"p"##),
                     Some(r##"Set various optional parameters, matching the key=value form"##),
                     Some(false),
                     Some(true)),
        
                    (Some(r##"out"##),
                     Some(r##"o"##),
                     Some(r##"Specify the file into which to write the program's output"##),
                     Some(false),
                     Some(false)),
                  ]),
            ("notes-list",
                    Some(r##"Lists notes for the specified project."##),
                    "Details at http://byron.github.io/google-apis-rs/google_containeranalysis1_beta1_cli/projects_notes-list",
                  vec![
                    (Some(r##"parent"##),
                     None,
                     Some(r##"Required. The name of the project to list notes for in the form of `projects/[PROJECT_ID]`."##),
                     Some(true),
                     Some(false)),
        
                    (Some(r##"v"##),
                     Some(r##"p"##),
                     Some(r##"Set various optional parameters, matching the key=value form"##),
                     Some(false),
                     Some(true)),
        
                    (Some(r##"out"##),
                     Some(r##"o"##),
                     Some(r##"Specify the file into which to write the program's output"##),
                     Some(false),
                     Some(false)),
                  ]),
            ("notes-occurrences-list",
                    Some(r##"Lists occurrences referencing the specified note. Provider projects can use this method to get all occurrences across consumer projects referencing the specified note."##),
                    "Details at http://byron.github.io/google-apis-rs/google_containeranalysis1_beta1_cli/projects_notes-occurrences-list",
                  vec![
                    (Some(r##"name"##),
                     None,
                     Some(r##"Required. The name of the note to list occurrences for in the form of `projects/[PROVIDER_ID]/notes/[NOTE_ID]`."##),
                     Some(true),
                     Some(false)),
        
                    (Some(r##"v"##),
                     Some(r##"p"##),
                     Some(r##"Set various optional parameters, matching the key=value form"##),
                     Some(false),
                     Some(true)),
        
                    (Some(r##"out"##),
                     Some(r##"o"##),
                     Some(r##"Specify the file into which to write the program's output"##),
                     Some(false),
                     Some(false)),
                  ]),
            ("notes-patch",
                    Some(r##"Updates the specified note."##),
                    "Details at http://byron.github.io/google-apis-rs/google_containeranalysis1_beta1_cli/projects_notes-patch",
                  vec![
                    (Some(r##"name"##),
                     None,
                     Some(r##"Required. The name of the note in the form of `projects/[PROVIDER_ID]/notes/[NOTE_ID]`."##),
                     Some(true),
                     Some(false)),
        
                    (Some(r##"kv"##),
                     Some(r##"r"##),
                     Some(r##"Set various fields of the request structure, matching the key=value form"##),
                     Some(true),
                     Some(true)),
        
                    (Some(r##"v"##),
                     Some(r##"p"##),
                     Some(r##"Set various optional parameters, matching the key=value form"##),
                     Some(false),
                     Some(true)),
        
                    (Some(r##"out"##),
                     Some(r##"o"##),
                     Some(r##"Specify the file into which to write the program's output"##),
                     Some(false),
                     Some(false)),
                  ]),
            ("notes-set-iam-policy",
                    Some(r##"Sets the access control policy on the specified note or occurrence. Requires `containeranalysis.notes.setIamPolicy` or `containeranalysis.occurrences.setIamPolicy` permission if the resource is a note or an occurrence, respectively. The resource takes the format `projects/[PROJECT_ID]/notes/[NOTE_ID]` for notes and `projects/[PROJECT_ID]/occurrences/[OCCURRENCE_ID]` for occurrences."##),
                    "Details at http://byron.github.io/google-apis-rs/google_containeranalysis1_beta1_cli/projects_notes-set-iam-policy",
                  vec![
                    (Some(r##"resource"##),
                     None,
                     Some(r##"REQUIRED: The resource for which the policy is being specified. See the operation documentation for the appropriate value for this field."##),
                     Some(true),
                     Some(false)),
        
                    (Some(r##"kv"##),
                     Some(r##"r"##),
                     Some(r##"Set various fields of the request structure, matching the key=value form"##),
                     Some(true),
                     Some(true)),
        
                    (Some(r##"v"##),
                     Some(r##"p"##),
                     Some(r##"Set various optional parameters, matching the key=value form"##),
                     Some(false),
                     Some(true)),
        
                    (Some(r##"out"##),
                     Some(r##"o"##),
                     Some(r##"Specify the file into which to write the program's output"##),
                     Some(false),
                     Some(false)),
                  ]),
            ("notes-test-iam-permissions",
                    Some(r##"Returns the permissions that a caller has on the specified note or occurrence. Requires list permission on the project (for example, `containeranalysis.notes.list`). The resource takes the format `projects/[PROJECT_ID]/notes/[NOTE_ID]` for notes and `projects/[PROJECT_ID]/occurrences/[OCCURRENCE_ID]` for occurrences."##),
                    "Details at http://byron.github.io/google-apis-rs/google_containeranalysis1_beta1_cli/projects_notes-test-iam-permissions",
                  vec![
                    (Some(r##"resource"##),
                     None,
                     Some(r##"REQUIRED: The resource for which the policy detail is being requested. See the operation documentation for the appropriate value for this field."##),
                     Some(true),
                     Some(false)),
        
                    (Some(r##"kv"##),
                     Some(r##"r"##),
                     Some(r##"Set various fields of the request structure, matching the key=value form"##),
                     Some(true),
                     Some(true)),
        
                    (Some(r##"v"##),
                     Some(r##"p"##),
                     Some(r##"Set various optional parameters, matching the key=value form"##),
                     Some(false),
                     Some(true)),
        
                    (Some(r##"out"##),
                     Some(r##"o"##),
                     Some(r##"Specify the file into which to write the program's output"##),
                     Some(false),
                     Some(false)),
                  ]),
            ("occurrences-batch-create",
                    Some(r##"Creates new occurrences in batch."##),
                    "Details at http://byron.github.io/google-apis-rs/google_containeranalysis1_beta1_cli/projects_occurrences-batch-create",
                  vec![
                    (Some(r##"parent"##),
                     None,
                     Some(r##"Required. The name of the project in the form of `projects/[PROJECT_ID]`, under which the occurrences are to be created."##),
                     Some(true),
                     Some(false)),
        
                    (Some(r##"kv"##),
                     Some(r##"r"##),
                     Some(r##"Set various fields of the request structure, matching the key=value form"##),
                     Some(true),
                     Some(true)),
        
                    (Some(r##"v"##),
                     Some(r##"p"##),
                     Some(r##"Set various optional parameters, matching the key=value form"##),
                     Some(false),
                     Some(true)),
        
                    (Some(r##"out"##),
                     Some(r##"o"##),
                     Some(r##"Specify the file into which to write the program's output"##),
                     Some(false),
                     Some(false)),
                  ]),
            ("occurrences-create",
                    Some(r##"Creates a new occurrence."##),
                    "Details at http://byron.github.io/google-apis-rs/google_containeranalysis1_beta1_cli/projects_occurrences-create",
                  vec![
                    (Some(r##"parent"##),
                     None,
                     Some(r##"Required. The name of the project in the form of `projects/[PROJECT_ID]`, under which the occurrence is to be created."##),
                     Some(true),
                     Some(false)),
        
                    (Some(r##"kv"##),
                     Some(r##"r"##),
                     Some(r##"Set various fields of the request structure, matching the key=value form"##),
                     Some(true),
                     Some(true)),
        
                    (Some(r##"v"##),
                     Some(r##"p"##),
                     Some(r##"Set various optional parameters, matching the key=value form"##),
                     Some(false),
                     Some(true)),
        
                    (Some(r##"out"##),
                     Some(r##"o"##),
                     Some(r##"Specify the file into which to write the program's output"##),
                     Some(false),
                     Some(false)),
                  ]),
            ("occurrences-delete",
                    Some(r##"Deletes the specified occurrence. For example, use this method to delete an occurrence when the occurrence is no longer applicable for the given resource."##),
                    "Details at http://byron.github.io/google-apis-rs/google_containeranalysis1_beta1_cli/projects_occurrences-delete",
                  vec![
                    (Some(r##"name"##),
                     None,
                     Some(r##"Required. The name of the occurrence in the form of `projects/[PROJECT_ID]/occurrences/[OCCURRENCE_ID]`."##),
                     Some(true),
                     Some(false)),
        
                    (Some(r##"v"##),
                     Some(r##"p"##),
                     Some(r##"Set various optional parameters, matching the key=value form"##),
                     Some(false),
                     Some(true)),
        
                    (Some(r##"out"##),
                     Some(r##"o"##),
                     Some(r##"Specify the file into which to write the program's output"##),
                     Some(false),
                     Some(false)),
                  ]),
            ("occurrences-get",
                    Some(r##"Gets the specified occurrence."##),
                    "Details at http://byron.github.io/google-apis-rs/google_containeranalysis1_beta1_cli/projects_occurrences-get",
                  vec![
                    (Some(r##"name"##),
                     None,
                     Some(r##"Required. The name of the occurrence in the form of `projects/[PROJECT_ID]/occurrences/[OCCURRENCE_ID]`."##),
                     Some(true),
                     Some(false)),
        
                    (Some(r##"v"##),
                     Some(r##"p"##),
                     Some(r##"Set various optional parameters, matching the key=value form"##),
                     Some(false),
                     Some(true)),
        
                    (Some(r##"out"##),
                     Some(r##"o"##),
                     Some(r##"Specify the file into which to write the program's output"##),
                     Some(false),
                     Some(false)),
                  ]),
            ("occurrences-get-iam-policy",
                    Some(r##"Gets the access control policy for a note or an occurrence resource. Requires `containeranalysis.notes.setIamPolicy` or `containeranalysis.occurrences.setIamPolicy` permission if the resource is a note or occurrence, respectively. The resource takes the format `projects/[PROJECT_ID]/notes/[NOTE_ID]` for notes and `projects/[PROJECT_ID]/occurrences/[OCCURRENCE_ID]` for occurrences."##),
                    "Details at http://byron.github.io/google-apis-rs/google_containeranalysis1_beta1_cli/projects_occurrences-get-iam-policy",
                  vec![
                    (Some(r##"resource"##),
                     None,
                     Some(r##"REQUIRED: The resource for which the policy is being requested. See the operation documentation for the appropriate value for this field."##),
                     Some(true),
                     Some(false)),
        
                    (Some(r##"kv"##),
                     Some(r##"r"##),
                     Some(r##"Set various fields of the request structure, matching the key=value form"##),
                     Some(true),
                     Some(true)),
        
                    (Some(r##"v"##),
                     Some(r##"p"##),
                     Some(r##"Set various optional parameters, matching the key=value form"##),
                     Some(false),
                     Some(true)),
        
                    (Some(r##"out"##),
                     Some(r##"o"##),
                     Some(r##"Specify the file into which to write the program's output"##),
                     Some(false),
                     Some(false)),
                  ]),
            ("occurrences-get-notes",
                    Some(r##"Gets the note attached to the specified occurrence. Consumer projects can use this method to get a note that belongs to a provider project."##),
                    "Details at http://byron.github.io/google-apis-rs/google_containeranalysis1_beta1_cli/projects_occurrences-get-notes",
                  vec![
                    (Some(r##"name"##),
                     None,
                     Some(r##"Required. The name of the occurrence in the form of `projects/[PROJECT_ID]/occurrences/[OCCURRENCE_ID]`."##),
                     Some(true),
                     Some(false)),
        
                    (Some(r##"v"##),
                     Some(r##"p"##),
                     Some(r##"Set various optional parameters, matching the key=value form"##),
                     Some(false),
                     Some(true)),
        
                    (Some(r##"out"##),
                     Some(r##"o"##),
                     Some(r##"Specify the file into which to write the program's output"##),
                     Some(false),
                     Some(false)),
                  ]),
            ("occurrences-get-vulnerability-summary",
                    Some(r##"Gets a summary of the number and severity of occurrences."##),
                    "Details at http://byron.github.io/google-apis-rs/google_containeranalysis1_beta1_cli/projects_occurrences-get-vulnerability-summary",
                  vec![
                    (Some(r##"parent"##),
                     None,
                     Some(r##"Required. The name of the project to get a vulnerability summary for in the form of `projects/[PROJECT_ID]`."##),
                     Some(true),
                     Some(false)),
        
                    (Some(r##"v"##),
                     Some(r##"p"##),
                     Some(r##"Set various optional parameters, matching the key=value form"##),
                     Some(false),
                     Some(true)),
        
                    (Some(r##"out"##),
                     Some(r##"o"##),
                     Some(r##"Specify the file into which to write the program's output"##),
                     Some(false),
                     Some(false)),
                  ]),
            ("occurrences-list",
                    Some(r##"Lists occurrences for the specified project."##),
                    "Details at http://byron.github.io/google-apis-rs/google_containeranalysis1_beta1_cli/projects_occurrences-list",
                  vec![
                    (Some(r##"parent"##),
                     None,
                     Some(r##"Required. The name of the project to list occurrences for in the form of `projects/[PROJECT_ID]`."##),
                     Some(true),
                     Some(false)),
        
                    (Some(r##"v"##),
                     Some(r##"p"##),
                     Some(r##"Set various optional parameters, matching the key=value form"##),
                     Some(false),
                     Some(true)),
        
                    (Some(r##"out"##),
                     Some(r##"o"##),
                     Some(r##"Specify the file into which to write the program's output"##),
                     Some(false),
                     Some(false)),
                  ]),
            ("occurrences-patch",
                    Some(r##"Updates the specified occurrence."##),
                    "Details at http://byron.github.io/google-apis-rs/google_containeranalysis1_beta1_cli/projects_occurrences-patch",
                  vec![
                    (Some(r##"name"##),
                     None,
                     Some(r##"Required. The name of the occurrence in the form of `projects/[PROJECT_ID]/occurrences/[OCCURRENCE_ID]`."##),
                     Some(true),
                     Some(false)),
        
                    (Some(r##"kv"##),
                     Some(r##"r"##),
                     Some(r##"Set various fields of the request structure, matching the key=value form"##),
                     Some(true),
                     Some(true)),
        
                    (Some(r##"v"##),
                     Some(r##"p"##),
                     Some(r##"Set various optional parameters, matching the key=value form"##),
                     Some(false),
                     Some(true)),
        
                    (Some(r##"out"##),
                     Some(r##"o"##),
                     Some(r##"Specify the file into which to write the program's output"##),
                     Some(false),
                     Some(false)),
                  ]),
            ("occurrences-set-iam-policy",
                    Some(r##"Sets the access control policy on the specified note or occurrence. Requires `containeranalysis.notes.setIamPolicy` or `containeranalysis.occurrences.setIamPolicy` permission if the resource is a note or an occurrence, respectively. The resource takes the format `projects/[PROJECT_ID]/notes/[NOTE_ID]` for notes and `projects/[PROJECT_ID]/occurrences/[OCCURRENCE_ID]` for occurrences."##),
                    "Details at http://byron.github.io/google-apis-rs/google_containeranalysis1_beta1_cli/projects_occurrences-set-iam-policy",
                  vec![
                    (Some(r##"resource"##),
                     None,
                     Some(r##"REQUIRED: The resource for which the policy is being specified. See the operation documentation for the appropriate value for this field."##),
                     Some(true),
                     Some(false)),
        
                    (Some(r##"kv"##),
                     Some(r##"r"##),
                     Some(r##"Set various fields of the request structure, matching the key=value form"##),
                     Some(true),
                     Some(true)),
        
                    (Some(r##"v"##),
                     Some(r##"p"##),
                     Some(r##"Set various optional parameters, matching the key=value form"##),
                     Some(false),
                     Some(true)),
        
                    (Some(r##"out"##),
                     Some(r##"o"##),
                     Some(r##"Specify the file into which to write the program's output"##),
                     Some(false),
                     Some(false)),
                  ]),
            ("occurrences-test-iam-permissions",
                    Some(r##"Returns the permissions that a caller has on the specified note or occurrence. Requires list permission on the project (for example, `containeranalysis.notes.list`). The resource takes the format `projects/[PROJECT_ID]/notes/[NOTE_ID]` for notes and `projects/[PROJECT_ID]/occurrences/[OCCURRENCE_ID]` for occurrences."##),
                    "Details at http://byron.github.io/google-apis-rs/google_containeranalysis1_beta1_cli/projects_occurrences-test-iam-permissions",
                  vec![
                    (Some(r##"resource"##),
                     None,
                     Some(r##"REQUIRED: The resource for which the policy detail is being requested. See the operation documentation for the appropriate value for this field."##),
                     Some(true),
                     Some(false)),
        
                    (Some(r##"kv"##),
                     Some(r##"r"##),
                     Some(r##"Set various fields of the request structure, matching the key=value form"##),
                     Some(true),
                     Some(true)),
        
                    (Some(r##"v"##),
                     Some(r##"p"##),
                     Some(r##"Set various optional parameters, matching the key=value form"##),
                     Some(false),
                     Some(true)),
        
                    (Some(r##"out"##),
                     Some(r##"o"##),
                     Some(r##"Specify the file into which to write the program's output"##),
                     Some(false),
                     Some(false)),
                  ]),
            ("scan-configs-get",
                    Some(r##"Gets the specified scan configuration."##),
                    "Details at http://byron.github.io/google-apis-rs/google_containeranalysis1_beta1_cli/projects_scan-configs-get",
                  vec![
                    (Some(r##"name"##),
                     None,
                     Some(r##"Required. The name of the scan configuration in the form of `projects/[PROJECT_ID]/scanConfigs/[SCAN_CONFIG_ID]`."##),
                     Some(true),
                     Some(false)),
        
                    (Some(r##"v"##),
                     Some(r##"p"##),
                     Some(r##"Set various optional parameters, matching the key=value form"##),
                     Some(false),
                     Some(true)),
        
                    (Some(r##"out"##),
                     Some(r##"o"##),
                     Some(r##"Specify the file into which to write the program's output"##),
                     Some(false),
                     Some(false)),
                  ]),
            ("scan-configs-list",
                    Some(r##"Lists scan configurations for the specified project."##),
                    "Details at http://byron.github.io/google-apis-rs/google_containeranalysis1_beta1_cli/projects_scan-configs-list",
                  vec![
                    (Some(r##"parent"##),
                     None,
                     Some(r##"Required. The name of the project to list scan configurations for in the form of `projects/[PROJECT_ID]`."##),
                     Some(true),
                     Some(false)),
        
                    (Some(r##"v"##),
                     Some(r##"p"##),
                     Some(r##"Set various optional parameters, matching the key=value form"##),
                     Some(false),
                     Some(true)),
        
                    (Some(r##"out"##),
                     Some(r##"o"##),
                     Some(r##"Specify the file into which to write the program's output"##),
                     Some(false),
                     Some(false)),
                  ]),
            ("scan-configs-update",
                    Some(r##"Updates the specified scan configuration."##),
                    "Details at http://byron.github.io/google-apis-rs/google_containeranalysis1_beta1_cli/projects_scan-configs-update",
                  vec![
                    (Some(r##"name"##),
                     None,
                     Some(r##"Required. The name of the scan configuration in the form of `projects/[PROJECT_ID]/scanConfigs/[SCAN_CONFIG_ID]`."##),
                     Some(true),
                     Some(false)),
        
                    (Some(r##"kv"##),
                     Some(r##"r"##),
                     Some(r##"Set various fields of the request structure, matching the key=value form"##),
                     Some(true),
                     Some(true)),
        
                    (Some(r##"v"##),
                     Some(r##"p"##),
                     Some(r##"Set various optional parameters, matching the key=value form"##),
                     Some(false),
                     Some(true)),
        
                    (Some(r##"out"##),
                     Some(r##"o"##),
                     Some(r##"Specify the file into which to write the program's output"##),
                     Some(false),
                     Some(false)),
                  ]),
            ]),
        
    ];
    
    let mut app = App::new("containeranalysis1-beta1")
           .author("Sebastian Thiel <byronimo@gmail.com>")
           .version("2.0.8+20210326")
           .about("An implementation of the Grafeas API, which stores, and enables querying and retrieval of critical metadata about all of your software artifacts.")
           .after_help("All documentation details can be found at http://byron.github.io/google-apis-rs/google_containeranalysis1_beta1_cli")
           .arg(Arg::with_name("url")
                   .long("scope")
                   .help("Specify the authentication a method should be executed in. Each scope requires the user to grant this application permission to use it.If unset, it defaults to the shortest scope url for a particular method.")
                   .multiple(true)
                   .takes_value(true))
           .arg(Arg::with_name("folder")
                   .long("config-dir")
                   .help("A directory into which we will store our persistent data. Defaults to a user-writable directory that we will create during the first invocation.[default: ~/.google-service-cli")
                   .multiple(false)
                   .takes_value(true))
           .arg(Arg::with_name("debug")
                   .long("debug")
                   .help("Debug print all errors")
                   .multiple(false)
                   .takes_value(false));
           
           for &(main_command_name, about, ref subcommands) in arg_data.iter() {
               let mut mcmd = SubCommand::with_name(main_command_name).about(about);
           
               for &(sub_command_name, ref desc, url_info, ref args) in subcommands {
                   let mut scmd = SubCommand::with_name(sub_command_name);
                   if let &Some(desc) = desc {
                       scmd = scmd.about(desc);
                   }
                   scmd = scmd.after_help(url_info);
           
                   for &(ref arg_name, ref flag, ref desc, ref required, ref multi) in args {
                       let arg_name_str =
                           match (arg_name, flag) {
                                   (&Some(an), _       ) => an,
                                   (_        , &Some(f)) => f,
                                    _                    => unreachable!(),
                            };
                       let mut arg = Arg::with_name(arg_name_str)
                                         .empty_values(false);
                       if let &Some(short_flag) = flag {
                           arg = arg.short(short_flag);
                       }
                       if let &Some(desc) = desc {
                           arg = arg.help(desc);
                       }
                       if arg_name.is_some() && flag.is_some() {
                           arg = arg.takes_value(true);
                       }
                       if let &Some(required) = required {
                           arg = arg.required(required);
                       }
                       if let &Some(multi) = multi {
                           arg = arg.multiple(multi);
                       }
                       scmd = scmd.arg(arg);
                   }
                   mcmd = mcmd.subcommand(scmd);
               }
               app = app.subcommand(mcmd);
           }
           
        let matches = app.get_matches();

    let debug = matches.is_present("debug");
    match Engine::new(matches).await {
        Err(err) => {
            exit_status = err.exit_code;
            writeln!(io::stderr(), "{}", err).ok();
        },
        Ok(engine) => {
            if let Err(doit_err) = engine.doit().await {
                exit_status = 1;
                match doit_err {
                    DoitError::IoError(path, err) => {
                        writeln!(io::stderr(), "Failed to open output file '{}': {}", path, err).ok();
                    },
                    DoitError::ApiError(err) => {
                        if debug {
                            writeln!(io::stderr(), "{:#?}", err).ok();
                        } else {
                            writeln!(io::stderr(), "{}", err).ok();
                        }
                    }
                }
            }
        }
    }

    std::process::exit(exit_status);
}
