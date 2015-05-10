// DO NOT EDIT !
// This file was generated automatically from 'src/mako/cli/main.rs.mako'
// DO NOT EDIT !
#![feature(plugin, exit_status)]
#![allow(unused_variables, unused_imports, dead_code, unused_mut)]

#[macro_use]
extern crate clap;
extern crate yup_oauth2 as oauth2;
extern crate yup_hyper_mock as mock;
extern crate serde;
extern crate hyper;
extern crate mime;
extern crate strsim;
extern crate google_adsensehost4d1 as api;

use std::env;
use std::io::{self, Write};
use clap::{App, SubCommand, Arg};

mod cmn;

use cmn::{InvalidOptionsError, CLIError, JsonTokenStorage, arg_from_str, writer_from_opts, parse_kv_arg, 
          input_file_from_opts, input_mime_from_opts, FieldCursor, FieldError, CallType, UploadProtocol,
          calltype_from_str, remove_json_null_values};

use std::default::Default;
use std::str::FromStr;

use oauth2::{Authenticator, DefaultAuthenticatorDelegate};
use serde::json;
use clap::ArgMatches;

enum DoitError {
    IoError(String, io::Error),
    ApiError(api::Error),
}

struct Engine<'n, 'a> {
    opt: ArgMatches<'n, 'a>,
    hub: api::AdSenseHost<hyper::Client, Authenticator<DefaultAuthenticatorDelegate, JsonTokenStorage, hyper::Client>>,
    gp: Vec<&'static str>,
    gpm: Vec<(&'static str, &'static str)>,
}


impl<'n, 'a> Engine<'n, 'a> {
    fn _accounts_adclients_get(&self, opt: &ArgMatches<'n, 'a>, dry_run: bool, err: &mut InvalidOptionsError)
                                                    -> Result<(), DoitError> {
        let mut call = self.hub.accounts().adclients_get(opt.value_of("account-id").unwrap_or(""), opt.value_of("ad-client-id").unwrap_or(""));
        for parg in opt.values_of("v").unwrap_or(Vec::new()).iter() {
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
                                                Vec::new() + &self.gp + &[]
                                                            ));
                    }
                }
            }
        }
        let protocol = CallType::Standard;
        if dry_run {
            Ok(())
        } else {
            assert!(err.issues.len() == 0);
            for scope in self.opt.values_of("url").unwrap_or(Vec::new()).iter() {
                call = call.add_scope(scope);
            }
            let mut ostream = match writer_from_opts(opt.value_of("out")) {
                Ok(mut f) => f,
                Err(io_err) => return Err(DoitError::IoError(opt.value_of("out").unwrap_or("-").to_string(), io_err)),
            };
            match match protocol {
                CallType::Standard => call.doit(),
                _ => unreachable!()
            } {
                Err(api_err) => Err(DoitError::ApiError(api_err)),
                Ok((mut response, output_schema)) => {
                    let mut value = json::value::to_value(&output_schema);
                    remove_json_null_values(&mut value);
                    serde::json::to_writer_pretty(&mut ostream, &value).unwrap();
                    Ok(())
                }
            }
        }
    }

    fn _accounts_adclients_list(&self, opt: &ArgMatches<'n, 'a>, dry_run: bool, err: &mut InvalidOptionsError)
                                                    -> Result<(), DoitError> {
        let mut call = self.hub.accounts().adclients_list(opt.value_of("account-id").unwrap_or(""));
        for parg in opt.values_of("v").unwrap_or(Vec::new()).iter() {
            let (key, value) = parse_kv_arg(&*parg, err, false);
            match key {
                "page-token" => {
                    call = call.page_token(value.unwrap_or(""));
                },
                "max-results" => {
                    call = call.max_results(arg_from_str(value.unwrap_or("-0"), err, "max-results", "integer"));
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
                                                Vec::new() + &self.gp + &["page-token", "max-results"]
                                                            ));
                    }
                }
            }
        }
        let protocol = CallType::Standard;
        if dry_run {
            Ok(())
        } else {
            assert!(err.issues.len() == 0);
            for scope in self.opt.values_of("url").unwrap_or(Vec::new()).iter() {
                call = call.add_scope(scope);
            }
            let mut ostream = match writer_from_opts(opt.value_of("out")) {
                Ok(mut f) => f,
                Err(io_err) => return Err(DoitError::IoError(opt.value_of("out").unwrap_or("-").to_string(), io_err)),
            };
            match match protocol {
                CallType::Standard => call.doit(),
                _ => unreachable!()
            } {
                Err(api_err) => Err(DoitError::ApiError(api_err)),
                Ok((mut response, output_schema)) => {
                    let mut value = json::value::to_value(&output_schema);
                    remove_json_null_values(&mut value);
                    serde::json::to_writer_pretty(&mut ostream, &value).unwrap();
                    Ok(())
                }
            }
        }
    }

    fn _accounts_adunits_delete(&self, opt: &ArgMatches<'n, 'a>, dry_run: bool, err: &mut InvalidOptionsError)
                                                    -> Result<(), DoitError> {
        let mut call = self.hub.accounts().adunits_delete(opt.value_of("account-id").unwrap_or(""), opt.value_of("ad-client-id").unwrap_or(""), opt.value_of("ad-unit-id").unwrap_or(""));
        for parg in opt.values_of("v").unwrap_or(Vec::new()).iter() {
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
                                                Vec::new() + &self.gp + &[]
                                                            ));
                    }
                }
            }
        }
        let protocol = CallType::Standard;
        if dry_run {
            Ok(())
        } else {
            assert!(err.issues.len() == 0);
            for scope in self.opt.values_of("url").unwrap_or(Vec::new()).iter() {
                call = call.add_scope(scope);
            }
            let mut ostream = match writer_from_opts(opt.value_of("out")) {
                Ok(mut f) => f,
                Err(io_err) => return Err(DoitError::IoError(opt.value_of("out").unwrap_or("-").to_string(), io_err)),
            };
            match match protocol {
                CallType::Standard => call.doit(),
                _ => unreachable!()
            } {
                Err(api_err) => Err(DoitError::ApiError(api_err)),
                Ok((mut response, output_schema)) => {
                    let mut value = json::value::to_value(&output_schema);
                    remove_json_null_values(&mut value);
                    serde::json::to_writer_pretty(&mut ostream, &value).unwrap();
                    Ok(())
                }
            }
        }
    }

    fn _accounts_adunits_get(&self, opt: &ArgMatches<'n, 'a>, dry_run: bool, err: &mut InvalidOptionsError)
                                                    -> Result<(), DoitError> {
        let mut call = self.hub.accounts().adunits_get(opt.value_of("account-id").unwrap_or(""), opt.value_of("ad-client-id").unwrap_or(""), opt.value_of("ad-unit-id").unwrap_or(""));
        for parg in opt.values_of("v").unwrap_or(Vec::new()).iter() {
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
                                                Vec::new() + &self.gp + &[]
                                                            ));
                    }
                }
            }
        }
        let protocol = CallType::Standard;
        if dry_run {
            Ok(())
        } else {
            assert!(err.issues.len() == 0);
            for scope in self.opt.values_of("url").unwrap_or(Vec::new()).iter() {
                call = call.add_scope(scope);
            }
            let mut ostream = match writer_from_opts(opt.value_of("out")) {
                Ok(mut f) => f,
                Err(io_err) => return Err(DoitError::IoError(opt.value_of("out").unwrap_or("-").to_string(), io_err)),
            };
            match match protocol {
                CallType::Standard => call.doit(),
                _ => unreachable!()
            } {
                Err(api_err) => Err(DoitError::ApiError(api_err)),
                Ok((mut response, output_schema)) => {
                    let mut value = json::value::to_value(&output_schema);
                    remove_json_null_values(&mut value);
                    serde::json::to_writer_pretty(&mut ostream, &value).unwrap();
                    Ok(())
                }
            }
        }
    }

    fn _accounts_adunits_get_ad_code(&self, opt: &ArgMatches<'n, 'a>, dry_run: bool, err: &mut InvalidOptionsError)
                                                    -> Result<(), DoitError> {
        let mut call = self.hub.accounts().adunits_get_ad_code(opt.value_of("account-id").unwrap_or(""), opt.value_of("ad-client-id").unwrap_or(""), opt.value_of("ad-unit-id").unwrap_or(""));
        for parg in opt.values_of("v").unwrap_or(Vec::new()).iter() {
            let (key, value) = parse_kv_arg(&*parg, err, false);
            match key {
                "host-custom-channel-id" => {
                    call = call.add_host_custom_channel_id(value.unwrap_or(""));
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
                                                Vec::new() + &self.gp + &["host-custom-channel-id"]
                                                            ));
                    }
                }
            }
        }
        let protocol = CallType::Standard;
        if dry_run {
            Ok(())
        } else {
            assert!(err.issues.len() == 0);
            for scope in self.opt.values_of("url").unwrap_or(Vec::new()).iter() {
                call = call.add_scope(scope);
            }
            let mut ostream = match writer_from_opts(opt.value_of("out")) {
                Ok(mut f) => f,
                Err(io_err) => return Err(DoitError::IoError(opt.value_of("out").unwrap_or("-").to_string(), io_err)),
            };
            match match protocol {
                CallType::Standard => call.doit(),
                _ => unreachable!()
            } {
                Err(api_err) => Err(DoitError::ApiError(api_err)),
                Ok((mut response, output_schema)) => {
                    let mut value = json::value::to_value(&output_schema);
                    remove_json_null_values(&mut value);
                    serde::json::to_writer_pretty(&mut ostream, &value).unwrap();
                    Ok(())
                }
            }
        }
    }

    fn _accounts_adunits_insert(&self, opt: &ArgMatches<'n, 'a>, dry_run: bool, err: &mut InvalidOptionsError)
                                                    -> Result<(), DoitError> {
        
        let mut request = api::AdUnit::default();
        let mut field_cursor = FieldCursor::default();
        for kvarg in opt.values_of("kv").unwrap_or(Vec::new()).iter() {
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
            fn request_content_ads_settings_backup_option_init(request: &mut api::AdUnit) {
                request_content_ads_settings_init(request);
                if request.content_ads_settings.as_mut().unwrap().backup_option.is_none() {
                    request.content_ads_settings.as_mut().unwrap().backup_option = Some(Default::default());
                }
            }
            
            fn request_content_ads_settings_init(request: &mut api::AdUnit) {
                if request.content_ads_settings.is_none() {
                    request.content_ads_settings = Some(Default::default());
                }
            }
            
            fn request_custom_style_colors_init(request: &mut api::AdUnit) {
                request_custom_style_init(request);
                if request.custom_style.as_mut().unwrap().colors.is_none() {
                    request.custom_style.as_mut().unwrap().colors = Some(Default::default());
                }
            }
            
            fn request_custom_style_font_init(request: &mut api::AdUnit) {
                request_custom_style_init(request);
                if request.custom_style.as_mut().unwrap().font.is_none() {
                    request.custom_style.as_mut().unwrap().font = Some(Default::default());
                }
            }
            
            fn request_custom_style_init(request: &mut api::AdUnit) {
                if request.custom_style.is_none() {
                    request.custom_style = Some(Default::default());
                }
            }
            
            fn request_mobile_content_ads_settings_init(request: &mut api::AdUnit) {
                if request.mobile_content_ads_settings.is_none() {
                    request.mobile_content_ads_settings = Some(Default::default());
                }
            }
            
            match &temp_cursor.to_string()[..] {
                "status" => {
                        request.status = Some(value.unwrap_or("").to_string());
                    },
                "kind" => {
                        request.kind = Some(value.unwrap_or("").to_string());
                    },
                "code" => {
                        request.code = Some(value.unwrap_or("").to_string());
                    },
                "name" => {
                        request.name = Some(value.unwrap_or("").to_string());
                    },
                "content-ads-settings.type" => {
                        request_content_ads_settings_init(&mut request);
                        request.content_ads_settings.as_mut().unwrap().type_ = Some(value.unwrap_or("").to_string());
                    },
                "content-ads-settings.backup-option.color" => {
                        request_content_ads_settings_backup_option_init(&mut request);
                        request.content_ads_settings.as_mut().unwrap().backup_option.as_mut().unwrap().color = Some(value.unwrap_or("").to_string());
                    },
                "content-ads-settings.backup-option.url" => {
                        request_content_ads_settings_backup_option_init(&mut request);
                        request.content_ads_settings.as_mut().unwrap().backup_option.as_mut().unwrap().url = Some(value.unwrap_or("").to_string());
                    },
                "content-ads-settings.backup-option.type" => {
                        request_content_ads_settings_backup_option_init(&mut request);
                        request.content_ads_settings.as_mut().unwrap().backup_option.as_mut().unwrap().type_ = Some(value.unwrap_or("").to_string());
                    },
                "content-ads-settings.size" => {
                        request_content_ads_settings_backup_option_init(&mut request);
                        request.content_ads_settings.as_mut().unwrap().size = Some(value.unwrap_or("").to_string());
                    },
                "id" => {
                        request_content_ads_settings_init(&mut request);
                        request.id = Some(value.unwrap_or("").to_string());
                    },
                "mobile-content-ads-settings.scripting-language" => {
                        request_mobile_content_ads_settings_init(&mut request);
                        request.mobile_content_ads_settings.as_mut().unwrap().scripting_language = Some(value.unwrap_or("").to_string());
                    },
                "mobile-content-ads-settings.type" => {
                        request_mobile_content_ads_settings_init(&mut request);
                        request.mobile_content_ads_settings.as_mut().unwrap().type_ = Some(value.unwrap_or("").to_string());
                    },
                "mobile-content-ads-settings.markup-language" => {
                        request_mobile_content_ads_settings_init(&mut request);
                        request.mobile_content_ads_settings.as_mut().unwrap().markup_language = Some(value.unwrap_or("").to_string());
                    },
                "mobile-content-ads-settings.size" => {
                        request_mobile_content_ads_settings_init(&mut request);
                        request.mobile_content_ads_settings.as_mut().unwrap().size = Some(value.unwrap_or("").to_string());
                    },
                "custom-style.corners" => {
                        request_custom_style_init(&mut request);
                        request.custom_style.as_mut().unwrap().corners = Some(value.unwrap_or("").to_string());
                    },
                "custom-style.colors.url" => {
                        request_custom_style_colors_init(&mut request);
                        request.custom_style.as_mut().unwrap().colors.as_mut().unwrap().url = Some(value.unwrap_or("").to_string());
                    },
                "custom-style.colors.text" => {
                        request_custom_style_colors_init(&mut request);
                        request.custom_style.as_mut().unwrap().colors.as_mut().unwrap().text = Some(value.unwrap_or("").to_string());
                    },
                "custom-style.colors.border" => {
                        request_custom_style_colors_init(&mut request);
                        request.custom_style.as_mut().unwrap().colors.as_mut().unwrap().border = Some(value.unwrap_or("").to_string());
                    },
                "custom-style.colors.background" => {
                        request_custom_style_colors_init(&mut request);
                        request.custom_style.as_mut().unwrap().colors.as_mut().unwrap().background = Some(value.unwrap_or("").to_string());
                    },
                "custom-style.colors.title" => {
                        request_custom_style_colors_init(&mut request);
                        request.custom_style.as_mut().unwrap().colors.as_mut().unwrap().title = Some(value.unwrap_or("").to_string());
                    },
                "custom-style.font.family" => {
                        request_custom_style_font_init(&mut request);
                        request.custom_style.as_mut().unwrap().font.as_mut().unwrap().family = Some(value.unwrap_or("").to_string());
                    },
                "custom-style.font.size" => {
                        request_custom_style_font_init(&mut request);
                        request.custom_style.as_mut().unwrap().font.as_mut().unwrap().size = Some(value.unwrap_or("").to_string());
                    },
                "custom-style.kind" => {
                        request_custom_style_font_init(&mut request);
                        request.custom_style.as_mut().unwrap().kind = Some(value.unwrap_or("").to_string());
                    },
                _ => {
                    let suggestion = FieldCursor::did_you_mean(key, &vec!["background", "backup-option", "border", "code", "color", "colors", "content-ads-settings", "corners", "custom-style", "family", "font", "id", "kind", "markup-language", "mobile-content-ads-settings", "name", "scripting-language", "size", "status", "text", "title", "type", "url"]);
                    err.issues.push(CLIError::Field(FieldError::Unknown(temp_cursor.to_string(), suggestion, value.map(|v| v.to_string()))));
                }
            }
        }
        let mut call = self.hub.accounts().adunits_insert(request, opt.value_of("account-id").unwrap_or(""), opt.value_of("ad-client-id").unwrap_or(""));
        for parg in opt.values_of("v").unwrap_or(Vec::new()).iter() {
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
                                                Vec::new() + &self.gp + &[]
                                                            ));
                    }
                }
            }
        }
        let protocol = CallType::Standard;
        if dry_run {
            Ok(())
        } else {
            assert!(err.issues.len() == 0);
            for scope in self.opt.values_of("url").unwrap_or(Vec::new()).iter() {
                call = call.add_scope(scope);
            }
            let mut ostream = match writer_from_opts(opt.value_of("out")) {
                Ok(mut f) => f,
                Err(io_err) => return Err(DoitError::IoError(opt.value_of("out").unwrap_or("-").to_string(), io_err)),
            };
            match match protocol {
                CallType::Standard => call.doit(),
                _ => unreachable!()
            } {
                Err(api_err) => Err(DoitError::ApiError(api_err)),
                Ok((mut response, output_schema)) => {
                    let mut value = json::value::to_value(&output_schema);
                    remove_json_null_values(&mut value);
                    serde::json::to_writer_pretty(&mut ostream, &value).unwrap();
                    Ok(())
                }
            }
        }
    }

    fn _accounts_adunits_list(&self, opt: &ArgMatches<'n, 'a>, dry_run: bool, err: &mut InvalidOptionsError)
                                                    -> Result<(), DoitError> {
        let mut call = self.hub.accounts().adunits_list(opt.value_of("account-id").unwrap_or(""), opt.value_of("ad-client-id").unwrap_or(""));
        for parg in opt.values_of("v").unwrap_or(Vec::new()).iter() {
            let (key, value) = parse_kv_arg(&*parg, err, false);
            match key {
                "page-token" => {
                    call = call.page_token(value.unwrap_or(""));
                },
                "max-results" => {
                    call = call.max_results(arg_from_str(value.unwrap_or("-0"), err, "max-results", "integer"));
                },
                "include-inactive" => {
                    call = call.include_inactive(arg_from_str(value.unwrap_or("false"), err, "include-inactive", "boolean"));
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
                                                Vec::new() + &self.gp + &["include-inactive", "page-token", "max-results"]
                                                            ));
                    }
                }
            }
        }
        let protocol = CallType::Standard;
        if dry_run {
            Ok(())
        } else {
            assert!(err.issues.len() == 0);
            for scope in self.opt.values_of("url").unwrap_or(Vec::new()).iter() {
                call = call.add_scope(scope);
            }
            let mut ostream = match writer_from_opts(opt.value_of("out")) {
                Ok(mut f) => f,
                Err(io_err) => return Err(DoitError::IoError(opt.value_of("out").unwrap_or("-").to_string(), io_err)),
            };
            match match protocol {
                CallType::Standard => call.doit(),
                _ => unreachable!()
            } {
                Err(api_err) => Err(DoitError::ApiError(api_err)),
                Ok((mut response, output_schema)) => {
                    let mut value = json::value::to_value(&output_schema);
                    remove_json_null_values(&mut value);
                    serde::json::to_writer_pretty(&mut ostream, &value).unwrap();
                    Ok(())
                }
            }
        }
    }

    fn _accounts_adunits_patch(&self, opt: &ArgMatches<'n, 'a>, dry_run: bool, err: &mut InvalidOptionsError)
                                                    -> Result<(), DoitError> {
        
        let mut request = api::AdUnit::default();
        let mut field_cursor = FieldCursor::default();
        for kvarg in opt.values_of("kv").unwrap_or(Vec::new()).iter() {
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
            fn request_content_ads_settings_backup_option_init(request: &mut api::AdUnit) {
                request_content_ads_settings_init(request);
                if request.content_ads_settings.as_mut().unwrap().backup_option.is_none() {
                    request.content_ads_settings.as_mut().unwrap().backup_option = Some(Default::default());
                }
            }
            
            fn request_content_ads_settings_init(request: &mut api::AdUnit) {
                if request.content_ads_settings.is_none() {
                    request.content_ads_settings = Some(Default::default());
                }
            }
            
            fn request_custom_style_colors_init(request: &mut api::AdUnit) {
                request_custom_style_init(request);
                if request.custom_style.as_mut().unwrap().colors.is_none() {
                    request.custom_style.as_mut().unwrap().colors = Some(Default::default());
                }
            }
            
            fn request_custom_style_font_init(request: &mut api::AdUnit) {
                request_custom_style_init(request);
                if request.custom_style.as_mut().unwrap().font.is_none() {
                    request.custom_style.as_mut().unwrap().font = Some(Default::default());
                }
            }
            
            fn request_custom_style_init(request: &mut api::AdUnit) {
                if request.custom_style.is_none() {
                    request.custom_style = Some(Default::default());
                }
            }
            
            fn request_mobile_content_ads_settings_init(request: &mut api::AdUnit) {
                if request.mobile_content_ads_settings.is_none() {
                    request.mobile_content_ads_settings = Some(Default::default());
                }
            }
            
            match &temp_cursor.to_string()[..] {
                "status" => {
                        request.status = Some(value.unwrap_or("").to_string());
                    },
                "kind" => {
                        request.kind = Some(value.unwrap_or("").to_string());
                    },
                "code" => {
                        request.code = Some(value.unwrap_or("").to_string());
                    },
                "name" => {
                        request.name = Some(value.unwrap_or("").to_string());
                    },
                "content-ads-settings.type" => {
                        request_content_ads_settings_init(&mut request);
                        request.content_ads_settings.as_mut().unwrap().type_ = Some(value.unwrap_or("").to_string());
                    },
                "content-ads-settings.backup-option.color" => {
                        request_content_ads_settings_backup_option_init(&mut request);
                        request.content_ads_settings.as_mut().unwrap().backup_option.as_mut().unwrap().color = Some(value.unwrap_or("").to_string());
                    },
                "content-ads-settings.backup-option.url" => {
                        request_content_ads_settings_backup_option_init(&mut request);
                        request.content_ads_settings.as_mut().unwrap().backup_option.as_mut().unwrap().url = Some(value.unwrap_or("").to_string());
                    },
                "content-ads-settings.backup-option.type" => {
                        request_content_ads_settings_backup_option_init(&mut request);
                        request.content_ads_settings.as_mut().unwrap().backup_option.as_mut().unwrap().type_ = Some(value.unwrap_or("").to_string());
                    },
                "content-ads-settings.size" => {
                        request_content_ads_settings_backup_option_init(&mut request);
                        request.content_ads_settings.as_mut().unwrap().size = Some(value.unwrap_or("").to_string());
                    },
                "id" => {
                        request_content_ads_settings_init(&mut request);
                        request.id = Some(value.unwrap_or("").to_string());
                    },
                "mobile-content-ads-settings.scripting-language" => {
                        request_mobile_content_ads_settings_init(&mut request);
                        request.mobile_content_ads_settings.as_mut().unwrap().scripting_language = Some(value.unwrap_or("").to_string());
                    },
                "mobile-content-ads-settings.type" => {
                        request_mobile_content_ads_settings_init(&mut request);
                        request.mobile_content_ads_settings.as_mut().unwrap().type_ = Some(value.unwrap_or("").to_string());
                    },
                "mobile-content-ads-settings.markup-language" => {
                        request_mobile_content_ads_settings_init(&mut request);
                        request.mobile_content_ads_settings.as_mut().unwrap().markup_language = Some(value.unwrap_or("").to_string());
                    },
                "mobile-content-ads-settings.size" => {
                        request_mobile_content_ads_settings_init(&mut request);
                        request.mobile_content_ads_settings.as_mut().unwrap().size = Some(value.unwrap_or("").to_string());
                    },
                "custom-style.corners" => {
                        request_custom_style_init(&mut request);
                        request.custom_style.as_mut().unwrap().corners = Some(value.unwrap_or("").to_string());
                    },
                "custom-style.colors.url" => {
                        request_custom_style_colors_init(&mut request);
                        request.custom_style.as_mut().unwrap().colors.as_mut().unwrap().url = Some(value.unwrap_or("").to_string());
                    },
                "custom-style.colors.text" => {
                        request_custom_style_colors_init(&mut request);
                        request.custom_style.as_mut().unwrap().colors.as_mut().unwrap().text = Some(value.unwrap_or("").to_string());
                    },
                "custom-style.colors.border" => {
                        request_custom_style_colors_init(&mut request);
                        request.custom_style.as_mut().unwrap().colors.as_mut().unwrap().border = Some(value.unwrap_or("").to_string());
                    },
                "custom-style.colors.background" => {
                        request_custom_style_colors_init(&mut request);
                        request.custom_style.as_mut().unwrap().colors.as_mut().unwrap().background = Some(value.unwrap_or("").to_string());
                    },
                "custom-style.colors.title" => {
                        request_custom_style_colors_init(&mut request);
                        request.custom_style.as_mut().unwrap().colors.as_mut().unwrap().title = Some(value.unwrap_or("").to_string());
                    },
                "custom-style.font.family" => {
                        request_custom_style_font_init(&mut request);
                        request.custom_style.as_mut().unwrap().font.as_mut().unwrap().family = Some(value.unwrap_or("").to_string());
                    },
                "custom-style.font.size" => {
                        request_custom_style_font_init(&mut request);
                        request.custom_style.as_mut().unwrap().font.as_mut().unwrap().size = Some(value.unwrap_or("").to_string());
                    },
                "custom-style.kind" => {
                        request_custom_style_font_init(&mut request);
                        request.custom_style.as_mut().unwrap().kind = Some(value.unwrap_or("").to_string());
                    },
                _ => {
                    let suggestion = FieldCursor::did_you_mean(key, &vec!["background", "backup-option", "border", "code", "color", "colors", "content-ads-settings", "corners", "custom-style", "family", "font", "id", "kind", "markup-language", "mobile-content-ads-settings", "name", "scripting-language", "size", "status", "text", "title", "type", "url"]);
                    err.issues.push(CLIError::Field(FieldError::Unknown(temp_cursor.to_string(), suggestion, value.map(|v| v.to_string()))));
                }
            }
        }
        let mut call = self.hub.accounts().adunits_patch(request, opt.value_of("account-id").unwrap_or(""), opt.value_of("ad-client-id").unwrap_or(""), opt.value_of("ad-unit-id").unwrap_or(""));
        for parg in opt.values_of("v").unwrap_or(Vec::new()).iter() {
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
                                                Vec::new() + &self.gp + &[]
                                                            ));
                    }
                }
            }
        }
        let protocol = CallType::Standard;
        if dry_run {
            Ok(())
        } else {
            assert!(err.issues.len() == 0);
            for scope in self.opt.values_of("url").unwrap_or(Vec::new()).iter() {
                call = call.add_scope(scope);
            }
            let mut ostream = match writer_from_opts(opt.value_of("out")) {
                Ok(mut f) => f,
                Err(io_err) => return Err(DoitError::IoError(opt.value_of("out").unwrap_or("-").to_string(), io_err)),
            };
            match match protocol {
                CallType::Standard => call.doit(),
                _ => unreachable!()
            } {
                Err(api_err) => Err(DoitError::ApiError(api_err)),
                Ok((mut response, output_schema)) => {
                    let mut value = json::value::to_value(&output_schema);
                    remove_json_null_values(&mut value);
                    serde::json::to_writer_pretty(&mut ostream, &value).unwrap();
                    Ok(())
                }
            }
        }
    }

    fn _accounts_adunits_update(&self, opt: &ArgMatches<'n, 'a>, dry_run: bool, err: &mut InvalidOptionsError)
                                                    -> Result<(), DoitError> {
        
        let mut request = api::AdUnit::default();
        let mut field_cursor = FieldCursor::default();
        for kvarg in opt.values_of("kv").unwrap_or(Vec::new()).iter() {
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
            fn request_content_ads_settings_backup_option_init(request: &mut api::AdUnit) {
                request_content_ads_settings_init(request);
                if request.content_ads_settings.as_mut().unwrap().backup_option.is_none() {
                    request.content_ads_settings.as_mut().unwrap().backup_option = Some(Default::default());
                }
            }
            
            fn request_content_ads_settings_init(request: &mut api::AdUnit) {
                if request.content_ads_settings.is_none() {
                    request.content_ads_settings = Some(Default::default());
                }
            }
            
            fn request_custom_style_colors_init(request: &mut api::AdUnit) {
                request_custom_style_init(request);
                if request.custom_style.as_mut().unwrap().colors.is_none() {
                    request.custom_style.as_mut().unwrap().colors = Some(Default::default());
                }
            }
            
            fn request_custom_style_font_init(request: &mut api::AdUnit) {
                request_custom_style_init(request);
                if request.custom_style.as_mut().unwrap().font.is_none() {
                    request.custom_style.as_mut().unwrap().font = Some(Default::default());
                }
            }
            
            fn request_custom_style_init(request: &mut api::AdUnit) {
                if request.custom_style.is_none() {
                    request.custom_style = Some(Default::default());
                }
            }
            
            fn request_mobile_content_ads_settings_init(request: &mut api::AdUnit) {
                if request.mobile_content_ads_settings.is_none() {
                    request.mobile_content_ads_settings = Some(Default::default());
                }
            }
            
            match &temp_cursor.to_string()[..] {
                "status" => {
                        request.status = Some(value.unwrap_or("").to_string());
                    },
                "kind" => {
                        request.kind = Some(value.unwrap_or("").to_string());
                    },
                "code" => {
                        request.code = Some(value.unwrap_or("").to_string());
                    },
                "name" => {
                        request.name = Some(value.unwrap_or("").to_string());
                    },
                "content-ads-settings.type" => {
                        request_content_ads_settings_init(&mut request);
                        request.content_ads_settings.as_mut().unwrap().type_ = Some(value.unwrap_or("").to_string());
                    },
                "content-ads-settings.backup-option.color" => {
                        request_content_ads_settings_backup_option_init(&mut request);
                        request.content_ads_settings.as_mut().unwrap().backup_option.as_mut().unwrap().color = Some(value.unwrap_or("").to_string());
                    },
                "content-ads-settings.backup-option.url" => {
                        request_content_ads_settings_backup_option_init(&mut request);
                        request.content_ads_settings.as_mut().unwrap().backup_option.as_mut().unwrap().url = Some(value.unwrap_or("").to_string());
                    },
                "content-ads-settings.backup-option.type" => {
                        request_content_ads_settings_backup_option_init(&mut request);
                        request.content_ads_settings.as_mut().unwrap().backup_option.as_mut().unwrap().type_ = Some(value.unwrap_or("").to_string());
                    },
                "content-ads-settings.size" => {
                        request_content_ads_settings_backup_option_init(&mut request);
                        request.content_ads_settings.as_mut().unwrap().size = Some(value.unwrap_or("").to_string());
                    },
                "id" => {
                        request_content_ads_settings_init(&mut request);
                        request.id = Some(value.unwrap_or("").to_string());
                    },
                "mobile-content-ads-settings.scripting-language" => {
                        request_mobile_content_ads_settings_init(&mut request);
                        request.mobile_content_ads_settings.as_mut().unwrap().scripting_language = Some(value.unwrap_or("").to_string());
                    },
                "mobile-content-ads-settings.type" => {
                        request_mobile_content_ads_settings_init(&mut request);
                        request.mobile_content_ads_settings.as_mut().unwrap().type_ = Some(value.unwrap_or("").to_string());
                    },
                "mobile-content-ads-settings.markup-language" => {
                        request_mobile_content_ads_settings_init(&mut request);
                        request.mobile_content_ads_settings.as_mut().unwrap().markup_language = Some(value.unwrap_or("").to_string());
                    },
                "mobile-content-ads-settings.size" => {
                        request_mobile_content_ads_settings_init(&mut request);
                        request.mobile_content_ads_settings.as_mut().unwrap().size = Some(value.unwrap_or("").to_string());
                    },
                "custom-style.corners" => {
                        request_custom_style_init(&mut request);
                        request.custom_style.as_mut().unwrap().corners = Some(value.unwrap_or("").to_string());
                    },
                "custom-style.colors.url" => {
                        request_custom_style_colors_init(&mut request);
                        request.custom_style.as_mut().unwrap().colors.as_mut().unwrap().url = Some(value.unwrap_or("").to_string());
                    },
                "custom-style.colors.text" => {
                        request_custom_style_colors_init(&mut request);
                        request.custom_style.as_mut().unwrap().colors.as_mut().unwrap().text = Some(value.unwrap_or("").to_string());
                    },
                "custom-style.colors.border" => {
                        request_custom_style_colors_init(&mut request);
                        request.custom_style.as_mut().unwrap().colors.as_mut().unwrap().border = Some(value.unwrap_or("").to_string());
                    },
                "custom-style.colors.background" => {
                        request_custom_style_colors_init(&mut request);
                        request.custom_style.as_mut().unwrap().colors.as_mut().unwrap().background = Some(value.unwrap_or("").to_string());
                    },
                "custom-style.colors.title" => {
                        request_custom_style_colors_init(&mut request);
                        request.custom_style.as_mut().unwrap().colors.as_mut().unwrap().title = Some(value.unwrap_or("").to_string());
                    },
                "custom-style.font.family" => {
                        request_custom_style_font_init(&mut request);
                        request.custom_style.as_mut().unwrap().font.as_mut().unwrap().family = Some(value.unwrap_or("").to_string());
                    },
                "custom-style.font.size" => {
                        request_custom_style_font_init(&mut request);
                        request.custom_style.as_mut().unwrap().font.as_mut().unwrap().size = Some(value.unwrap_or("").to_string());
                    },
                "custom-style.kind" => {
                        request_custom_style_font_init(&mut request);
                        request.custom_style.as_mut().unwrap().kind = Some(value.unwrap_or("").to_string());
                    },
                _ => {
                    let suggestion = FieldCursor::did_you_mean(key, &vec!["background", "backup-option", "border", "code", "color", "colors", "content-ads-settings", "corners", "custom-style", "family", "font", "id", "kind", "markup-language", "mobile-content-ads-settings", "name", "scripting-language", "size", "status", "text", "title", "type", "url"]);
                    err.issues.push(CLIError::Field(FieldError::Unknown(temp_cursor.to_string(), suggestion, value.map(|v| v.to_string()))));
                }
            }
        }
        let mut call = self.hub.accounts().adunits_update(request, opt.value_of("account-id").unwrap_or(""), opt.value_of("ad-client-id").unwrap_or(""));
        for parg in opt.values_of("v").unwrap_or(Vec::new()).iter() {
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
                                                Vec::new() + &self.gp + &[]
                                                            ));
                    }
                }
            }
        }
        let protocol = CallType::Standard;
        if dry_run {
            Ok(())
        } else {
            assert!(err.issues.len() == 0);
            for scope in self.opt.values_of("url").unwrap_or(Vec::new()).iter() {
                call = call.add_scope(scope);
            }
            let mut ostream = match writer_from_opts(opt.value_of("out")) {
                Ok(mut f) => f,
                Err(io_err) => return Err(DoitError::IoError(opt.value_of("out").unwrap_or("-").to_string(), io_err)),
            };
            match match protocol {
                CallType::Standard => call.doit(),
                _ => unreachable!()
            } {
                Err(api_err) => Err(DoitError::ApiError(api_err)),
                Ok((mut response, output_schema)) => {
                    let mut value = json::value::to_value(&output_schema);
                    remove_json_null_values(&mut value);
                    serde::json::to_writer_pretty(&mut ostream, &value).unwrap();
                    Ok(())
                }
            }
        }
    }

    fn _accounts_get(&self, opt: &ArgMatches<'n, 'a>, dry_run: bool, err: &mut InvalidOptionsError)
                                                    -> Result<(), DoitError> {
        let mut call = self.hub.accounts().get(opt.value_of("account-id").unwrap_or(""));
        for parg in opt.values_of("v").unwrap_or(Vec::new()).iter() {
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
                                                Vec::new() + &self.gp + &[]
                                                            ));
                    }
                }
            }
        }
        let protocol = CallType::Standard;
        if dry_run {
            Ok(())
        } else {
            assert!(err.issues.len() == 0);
            for scope in self.opt.values_of("url").unwrap_or(Vec::new()).iter() {
                call = call.add_scope(scope);
            }
            let mut ostream = match writer_from_opts(opt.value_of("out")) {
                Ok(mut f) => f,
                Err(io_err) => return Err(DoitError::IoError(opt.value_of("out").unwrap_or("-").to_string(), io_err)),
            };
            match match protocol {
                CallType::Standard => call.doit(),
                _ => unreachable!()
            } {
                Err(api_err) => Err(DoitError::ApiError(api_err)),
                Ok((mut response, output_schema)) => {
                    let mut value = json::value::to_value(&output_schema);
                    remove_json_null_values(&mut value);
                    serde::json::to_writer_pretty(&mut ostream, &value).unwrap();
                    Ok(())
                }
            }
        }
    }

    fn _accounts_list(&self, opt: &ArgMatches<'n, 'a>, dry_run: bool, err: &mut InvalidOptionsError)
                                                    -> Result<(), DoitError> {
        let mut call = self.hub.accounts().list(&opt.values_of("filter-ad-client-id").unwrap_or(Vec::new()).iter().map(|&v| v.to_string()).collect::<Vec<String>>());
        for parg in opt.values_of("v").unwrap_or(Vec::new()).iter() {
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
                                                Vec::new() + &self.gp + &[]
                                                            ));
                    }
                }
            }
        }
        let protocol = CallType::Standard;
        if dry_run {
            Ok(())
        } else {
            assert!(err.issues.len() == 0);
            for scope in self.opt.values_of("url").unwrap_or(Vec::new()).iter() {
                call = call.add_scope(scope);
            }
            let mut ostream = match writer_from_opts(opt.value_of("out")) {
                Ok(mut f) => f,
                Err(io_err) => return Err(DoitError::IoError(opt.value_of("out").unwrap_or("-").to_string(), io_err)),
            };
            match match protocol {
                CallType::Standard => call.doit(),
                _ => unreachable!()
            } {
                Err(api_err) => Err(DoitError::ApiError(api_err)),
                Ok((mut response, output_schema)) => {
                    let mut value = json::value::to_value(&output_schema);
                    remove_json_null_values(&mut value);
                    serde::json::to_writer_pretty(&mut ostream, &value).unwrap();
                    Ok(())
                }
            }
        }
    }

    fn _accounts_reports_generate(&self, opt: &ArgMatches<'n, 'a>, dry_run: bool, err: &mut InvalidOptionsError)
                                                    -> Result<(), DoitError> {
        let mut call = self.hub.accounts().reports_generate(opt.value_of("account-id").unwrap_or(""), opt.value_of("start-date").unwrap_or(""), opt.value_of("end-date").unwrap_or(""));
        for parg in opt.values_of("v").unwrap_or(Vec::new()).iter() {
            let (key, value) = parse_kv_arg(&*parg, err, false);
            match key {
                "start-index" => {
                    call = call.start_index(arg_from_str(value.unwrap_or("-0"), err, "start-index", "integer"));
                },
                "sort" => {
                    call = call.add_sort(value.unwrap_or(""));
                },
                "metric" => {
                    call = call.add_metric(value.unwrap_or(""));
                },
                "max-results" => {
                    call = call.max_results(arg_from_str(value.unwrap_or("-0"), err, "max-results", "integer"));
                },
                "locale" => {
                    call = call.locale(value.unwrap_or(""));
                },
                "filter" => {
                    call = call.add_filter(value.unwrap_or(""));
                },
                "dimension" => {
                    call = call.add_dimension(value.unwrap_or(""));
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
                                                Vec::new() + &self.gp + &["sort", "locale", "metric", "max-results", "filter", "start-index", "dimension"]
                                                            ));
                    }
                }
            }
        }
        let protocol = CallType::Standard;
        if dry_run {
            Ok(())
        } else {
            assert!(err.issues.len() == 0);
            for scope in self.opt.values_of("url").unwrap_or(Vec::new()).iter() {
                call = call.add_scope(scope);
            }
            let mut ostream = match writer_from_opts(opt.value_of("out")) {
                Ok(mut f) => f,
                Err(io_err) => return Err(DoitError::IoError(opt.value_of("out").unwrap_or("-").to_string(), io_err)),
            };
            match match protocol {
                CallType::Standard => call.doit(),
                _ => unreachable!()
            } {
                Err(api_err) => Err(DoitError::ApiError(api_err)),
                Ok((mut response, output_schema)) => {
                    let mut value = json::value::to_value(&output_schema);
                    remove_json_null_values(&mut value);
                    serde::json::to_writer_pretty(&mut ostream, &value).unwrap();
                    Ok(())
                }
            }
        }
    }

    fn _adclients_get(&self, opt: &ArgMatches<'n, 'a>, dry_run: bool, err: &mut InvalidOptionsError)
                                                    -> Result<(), DoitError> {
        let mut call = self.hub.adclients().get(opt.value_of("ad-client-id").unwrap_or(""));
        for parg in opt.values_of("v").unwrap_or(Vec::new()).iter() {
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
                                                Vec::new() + &self.gp + &[]
                                                            ));
                    }
                }
            }
        }
        let protocol = CallType::Standard;
        if dry_run {
            Ok(())
        } else {
            assert!(err.issues.len() == 0);
            for scope in self.opt.values_of("url").unwrap_or(Vec::new()).iter() {
                call = call.add_scope(scope);
            }
            let mut ostream = match writer_from_opts(opt.value_of("out")) {
                Ok(mut f) => f,
                Err(io_err) => return Err(DoitError::IoError(opt.value_of("out").unwrap_or("-").to_string(), io_err)),
            };
            match match protocol {
                CallType::Standard => call.doit(),
                _ => unreachable!()
            } {
                Err(api_err) => Err(DoitError::ApiError(api_err)),
                Ok((mut response, output_schema)) => {
                    let mut value = json::value::to_value(&output_schema);
                    remove_json_null_values(&mut value);
                    serde::json::to_writer_pretty(&mut ostream, &value).unwrap();
                    Ok(())
                }
            }
        }
    }

    fn _adclients_list(&self, opt: &ArgMatches<'n, 'a>, dry_run: bool, err: &mut InvalidOptionsError)
                                                    -> Result<(), DoitError> {
        let mut call = self.hub.adclients().list();
        for parg in opt.values_of("v").unwrap_or(Vec::new()).iter() {
            let (key, value) = parse_kv_arg(&*parg, err, false);
            match key {
                "page-token" => {
                    call = call.page_token(value.unwrap_or(""));
                },
                "max-results" => {
                    call = call.max_results(arg_from_str(value.unwrap_or("-0"), err, "max-results", "integer"));
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
                                                Vec::new() + &self.gp + &["page-token", "max-results"]
                                                            ));
                    }
                }
            }
        }
        let protocol = CallType::Standard;
        if dry_run {
            Ok(())
        } else {
            assert!(err.issues.len() == 0);
            for scope in self.opt.values_of("url").unwrap_or(Vec::new()).iter() {
                call = call.add_scope(scope);
            }
            let mut ostream = match writer_from_opts(opt.value_of("out")) {
                Ok(mut f) => f,
                Err(io_err) => return Err(DoitError::IoError(opt.value_of("out").unwrap_or("-").to_string(), io_err)),
            };
            match match protocol {
                CallType::Standard => call.doit(),
                _ => unreachable!()
            } {
                Err(api_err) => Err(DoitError::ApiError(api_err)),
                Ok((mut response, output_schema)) => {
                    let mut value = json::value::to_value(&output_schema);
                    remove_json_null_values(&mut value);
                    serde::json::to_writer_pretty(&mut ostream, &value).unwrap();
                    Ok(())
                }
            }
        }
    }

    fn _associationsessions_start(&self, opt: &ArgMatches<'n, 'a>, dry_run: bool, err: &mut InvalidOptionsError)
                                                    -> Result<(), DoitError> {
        let mut call = self.hub.associationsessions().start(&opt.values_of("product-code").unwrap_or(Vec::new()).iter().map(|&v| v.to_string()).collect::<Vec<String>>(), opt.value_of("website-url").unwrap_or(""));
        for parg in opt.values_of("v").unwrap_or(Vec::new()).iter() {
            let (key, value) = parse_kv_arg(&*parg, err, false);
            match key {
                "website-locale" => {
                    call = call.website_locale(value.unwrap_or(""));
                },
                "user-locale" => {
                    call = call.user_locale(value.unwrap_or(""));
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
                                                Vec::new() + &self.gp + &["website-locale", "user-locale"]
                                                            ));
                    }
                }
            }
        }
        let protocol = CallType::Standard;
        if dry_run {
            Ok(())
        } else {
            assert!(err.issues.len() == 0);
            for scope in self.opt.values_of("url").unwrap_or(Vec::new()).iter() {
                call = call.add_scope(scope);
            }
            let mut ostream = match writer_from_opts(opt.value_of("out")) {
                Ok(mut f) => f,
                Err(io_err) => return Err(DoitError::IoError(opt.value_of("out").unwrap_or("-").to_string(), io_err)),
            };
            match match protocol {
                CallType::Standard => call.doit(),
                _ => unreachable!()
            } {
                Err(api_err) => Err(DoitError::ApiError(api_err)),
                Ok((mut response, output_schema)) => {
                    let mut value = json::value::to_value(&output_schema);
                    remove_json_null_values(&mut value);
                    serde::json::to_writer_pretty(&mut ostream, &value).unwrap();
                    Ok(())
                }
            }
        }
    }

    fn _associationsessions_verify(&self, opt: &ArgMatches<'n, 'a>, dry_run: bool, err: &mut InvalidOptionsError)
                                                    -> Result<(), DoitError> {
        let mut call = self.hub.associationsessions().verify(opt.value_of("token").unwrap_or(""));
        for parg in opt.values_of("v").unwrap_or(Vec::new()).iter() {
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
                                                Vec::new() + &self.gp + &[]
                                                            ));
                    }
                }
            }
        }
        let protocol = CallType::Standard;
        if dry_run {
            Ok(())
        } else {
            assert!(err.issues.len() == 0);
            for scope in self.opt.values_of("url").unwrap_or(Vec::new()).iter() {
                call = call.add_scope(scope);
            }
            let mut ostream = match writer_from_opts(opt.value_of("out")) {
                Ok(mut f) => f,
                Err(io_err) => return Err(DoitError::IoError(opt.value_of("out").unwrap_or("-").to_string(), io_err)),
            };
            match match protocol {
                CallType::Standard => call.doit(),
                _ => unreachable!()
            } {
                Err(api_err) => Err(DoitError::ApiError(api_err)),
                Ok((mut response, output_schema)) => {
                    let mut value = json::value::to_value(&output_schema);
                    remove_json_null_values(&mut value);
                    serde::json::to_writer_pretty(&mut ostream, &value).unwrap();
                    Ok(())
                }
            }
        }
    }

    fn _customchannels_delete(&self, opt: &ArgMatches<'n, 'a>, dry_run: bool, err: &mut InvalidOptionsError)
                                                    -> Result<(), DoitError> {
        let mut call = self.hub.customchannels().delete(opt.value_of("ad-client-id").unwrap_or(""), opt.value_of("custom-channel-id").unwrap_or(""));
        for parg in opt.values_of("v").unwrap_or(Vec::new()).iter() {
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
                                                Vec::new() + &self.gp + &[]
                                                            ));
                    }
                }
            }
        }
        let protocol = CallType::Standard;
        if dry_run {
            Ok(())
        } else {
            assert!(err.issues.len() == 0);
            for scope in self.opt.values_of("url").unwrap_or(Vec::new()).iter() {
                call = call.add_scope(scope);
            }
            let mut ostream = match writer_from_opts(opt.value_of("out")) {
                Ok(mut f) => f,
                Err(io_err) => return Err(DoitError::IoError(opt.value_of("out").unwrap_or("-").to_string(), io_err)),
            };
            match match protocol {
                CallType::Standard => call.doit(),
                _ => unreachable!()
            } {
                Err(api_err) => Err(DoitError::ApiError(api_err)),
                Ok((mut response, output_schema)) => {
                    let mut value = json::value::to_value(&output_schema);
                    remove_json_null_values(&mut value);
                    serde::json::to_writer_pretty(&mut ostream, &value).unwrap();
                    Ok(())
                }
            }
        }
    }

    fn _customchannels_get(&self, opt: &ArgMatches<'n, 'a>, dry_run: bool, err: &mut InvalidOptionsError)
                                                    -> Result<(), DoitError> {
        let mut call = self.hub.customchannels().get(opt.value_of("ad-client-id").unwrap_or(""), opt.value_of("custom-channel-id").unwrap_or(""));
        for parg in opt.values_of("v").unwrap_or(Vec::new()).iter() {
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
                                                Vec::new() + &self.gp + &[]
                                                            ));
                    }
                }
            }
        }
        let protocol = CallType::Standard;
        if dry_run {
            Ok(())
        } else {
            assert!(err.issues.len() == 0);
            for scope in self.opt.values_of("url").unwrap_or(Vec::new()).iter() {
                call = call.add_scope(scope);
            }
            let mut ostream = match writer_from_opts(opt.value_of("out")) {
                Ok(mut f) => f,
                Err(io_err) => return Err(DoitError::IoError(opt.value_of("out").unwrap_or("-").to_string(), io_err)),
            };
            match match protocol {
                CallType::Standard => call.doit(),
                _ => unreachable!()
            } {
                Err(api_err) => Err(DoitError::ApiError(api_err)),
                Ok((mut response, output_schema)) => {
                    let mut value = json::value::to_value(&output_schema);
                    remove_json_null_values(&mut value);
                    serde::json::to_writer_pretty(&mut ostream, &value).unwrap();
                    Ok(())
                }
            }
        }
    }

    fn _customchannels_insert(&self, opt: &ArgMatches<'n, 'a>, dry_run: bool, err: &mut InvalidOptionsError)
                                                    -> Result<(), DoitError> {
        
        let mut request = api::CustomChannel::default();
        let mut field_cursor = FieldCursor::default();
        for kvarg in opt.values_of("kv").unwrap_or(Vec::new()).iter() {
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
            match &temp_cursor.to_string()[..] {
                "kind" => {
                        request.kind = Some(value.unwrap_or("").to_string());
                    },
                "code" => {
                        request.code = Some(value.unwrap_or("").to_string());
                    },
                "id" => {
                        request.id = Some(value.unwrap_or("").to_string());
                    },
                "name" => {
                        request.name = Some(value.unwrap_or("").to_string());
                    },
                _ => {
                    let suggestion = FieldCursor::did_you_mean(key, &vec!["code", "id", "kind", "name"]);
                    err.issues.push(CLIError::Field(FieldError::Unknown(temp_cursor.to_string(), suggestion, value.map(|v| v.to_string()))));
                }
            }
        }
        let mut call = self.hub.customchannels().insert(request, opt.value_of("ad-client-id").unwrap_or(""));
        for parg in opt.values_of("v").unwrap_or(Vec::new()).iter() {
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
                                                Vec::new() + &self.gp + &[]
                                                            ));
                    }
                }
            }
        }
        let protocol = CallType::Standard;
        if dry_run {
            Ok(())
        } else {
            assert!(err.issues.len() == 0);
            for scope in self.opt.values_of("url").unwrap_or(Vec::new()).iter() {
                call = call.add_scope(scope);
            }
            let mut ostream = match writer_from_opts(opt.value_of("out")) {
                Ok(mut f) => f,
                Err(io_err) => return Err(DoitError::IoError(opt.value_of("out").unwrap_or("-").to_string(), io_err)),
            };
            match match protocol {
                CallType::Standard => call.doit(),
                _ => unreachable!()
            } {
                Err(api_err) => Err(DoitError::ApiError(api_err)),
                Ok((mut response, output_schema)) => {
                    let mut value = json::value::to_value(&output_schema);
                    remove_json_null_values(&mut value);
                    serde::json::to_writer_pretty(&mut ostream, &value).unwrap();
                    Ok(())
                }
            }
        }
    }

    fn _customchannels_list(&self, opt: &ArgMatches<'n, 'a>, dry_run: bool, err: &mut InvalidOptionsError)
                                                    -> Result<(), DoitError> {
        let mut call = self.hub.customchannels().list(opt.value_of("ad-client-id").unwrap_or(""));
        for parg in opt.values_of("v").unwrap_or(Vec::new()).iter() {
            let (key, value) = parse_kv_arg(&*parg, err, false);
            match key {
                "page-token" => {
                    call = call.page_token(value.unwrap_or(""));
                },
                "max-results" => {
                    call = call.max_results(arg_from_str(value.unwrap_or("-0"), err, "max-results", "integer"));
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
                                                Vec::new() + &self.gp + &["page-token", "max-results"]
                                                            ));
                    }
                }
            }
        }
        let protocol = CallType::Standard;
        if dry_run {
            Ok(())
        } else {
            assert!(err.issues.len() == 0);
            for scope in self.opt.values_of("url").unwrap_or(Vec::new()).iter() {
                call = call.add_scope(scope);
            }
            let mut ostream = match writer_from_opts(opt.value_of("out")) {
                Ok(mut f) => f,
                Err(io_err) => return Err(DoitError::IoError(opt.value_of("out").unwrap_or("-").to_string(), io_err)),
            };
            match match protocol {
                CallType::Standard => call.doit(),
                _ => unreachable!()
            } {
                Err(api_err) => Err(DoitError::ApiError(api_err)),
                Ok((mut response, output_schema)) => {
                    let mut value = json::value::to_value(&output_schema);
                    remove_json_null_values(&mut value);
                    serde::json::to_writer_pretty(&mut ostream, &value).unwrap();
                    Ok(())
                }
            }
        }
    }

    fn _customchannels_patch(&self, opt: &ArgMatches<'n, 'a>, dry_run: bool, err: &mut InvalidOptionsError)
                                                    -> Result<(), DoitError> {
        
        let mut request = api::CustomChannel::default();
        let mut field_cursor = FieldCursor::default();
        for kvarg in opt.values_of("kv").unwrap_or(Vec::new()).iter() {
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
            match &temp_cursor.to_string()[..] {
                "kind" => {
                        request.kind = Some(value.unwrap_or("").to_string());
                    },
                "code" => {
                        request.code = Some(value.unwrap_or("").to_string());
                    },
                "id" => {
                        request.id = Some(value.unwrap_or("").to_string());
                    },
                "name" => {
                        request.name = Some(value.unwrap_or("").to_string());
                    },
                _ => {
                    let suggestion = FieldCursor::did_you_mean(key, &vec!["code", "id", "kind", "name"]);
                    err.issues.push(CLIError::Field(FieldError::Unknown(temp_cursor.to_string(), suggestion, value.map(|v| v.to_string()))));
                }
            }
        }
        let mut call = self.hub.customchannels().patch(request, opt.value_of("ad-client-id").unwrap_or(""), opt.value_of("custom-channel-id").unwrap_or(""));
        for parg in opt.values_of("v").unwrap_or(Vec::new()).iter() {
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
                                                Vec::new() + &self.gp + &[]
                                                            ));
                    }
                }
            }
        }
        let protocol = CallType::Standard;
        if dry_run {
            Ok(())
        } else {
            assert!(err.issues.len() == 0);
            for scope in self.opt.values_of("url").unwrap_or(Vec::new()).iter() {
                call = call.add_scope(scope);
            }
            let mut ostream = match writer_from_opts(opt.value_of("out")) {
                Ok(mut f) => f,
                Err(io_err) => return Err(DoitError::IoError(opt.value_of("out").unwrap_or("-").to_string(), io_err)),
            };
            match match protocol {
                CallType::Standard => call.doit(),
                _ => unreachable!()
            } {
                Err(api_err) => Err(DoitError::ApiError(api_err)),
                Ok((mut response, output_schema)) => {
                    let mut value = json::value::to_value(&output_schema);
                    remove_json_null_values(&mut value);
                    serde::json::to_writer_pretty(&mut ostream, &value).unwrap();
                    Ok(())
                }
            }
        }
    }

    fn _customchannels_update(&self, opt: &ArgMatches<'n, 'a>, dry_run: bool, err: &mut InvalidOptionsError)
                                                    -> Result<(), DoitError> {
        
        let mut request = api::CustomChannel::default();
        let mut field_cursor = FieldCursor::default();
        for kvarg in opt.values_of("kv").unwrap_or(Vec::new()).iter() {
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
            match &temp_cursor.to_string()[..] {
                "kind" => {
                        request.kind = Some(value.unwrap_or("").to_string());
                    },
                "code" => {
                        request.code = Some(value.unwrap_or("").to_string());
                    },
                "id" => {
                        request.id = Some(value.unwrap_or("").to_string());
                    },
                "name" => {
                        request.name = Some(value.unwrap_or("").to_string());
                    },
                _ => {
                    let suggestion = FieldCursor::did_you_mean(key, &vec!["code", "id", "kind", "name"]);
                    err.issues.push(CLIError::Field(FieldError::Unknown(temp_cursor.to_string(), suggestion, value.map(|v| v.to_string()))));
                }
            }
        }
        let mut call = self.hub.customchannels().update(request, opt.value_of("ad-client-id").unwrap_or(""));
        for parg in opt.values_of("v").unwrap_or(Vec::new()).iter() {
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
                                                Vec::new() + &self.gp + &[]
                                                            ));
                    }
                }
            }
        }
        let protocol = CallType::Standard;
        if dry_run {
            Ok(())
        } else {
            assert!(err.issues.len() == 0);
            for scope in self.opt.values_of("url").unwrap_or(Vec::new()).iter() {
                call = call.add_scope(scope);
            }
            let mut ostream = match writer_from_opts(opt.value_of("out")) {
                Ok(mut f) => f,
                Err(io_err) => return Err(DoitError::IoError(opt.value_of("out").unwrap_or("-").to_string(), io_err)),
            };
            match match protocol {
                CallType::Standard => call.doit(),
                _ => unreachable!()
            } {
                Err(api_err) => Err(DoitError::ApiError(api_err)),
                Ok((mut response, output_schema)) => {
                    let mut value = json::value::to_value(&output_schema);
                    remove_json_null_values(&mut value);
                    serde::json::to_writer_pretty(&mut ostream, &value).unwrap();
                    Ok(())
                }
            }
        }
    }

    fn _reports_generate(&self, opt: &ArgMatches<'n, 'a>, dry_run: bool, err: &mut InvalidOptionsError)
                                                    -> Result<(), DoitError> {
        let mut call = self.hub.reports().generate(opt.value_of("start-date").unwrap_or(""), opt.value_of("end-date").unwrap_or(""));
        for parg in opt.values_of("v").unwrap_or(Vec::new()).iter() {
            let (key, value) = parse_kv_arg(&*parg, err, false);
            match key {
                "start-index" => {
                    call = call.start_index(arg_from_str(value.unwrap_or("-0"), err, "start-index", "integer"));
                },
                "sort" => {
                    call = call.add_sort(value.unwrap_or(""));
                },
                "metric" => {
                    call = call.add_metric(value.unwrap_or(""));
                },
                "max-results" => {
                    call = call.max_results(arg_from_str(value.unwrap_or("-0"), err, "max-results", "integer"));
                },
                "locale" => {
                    call = call.locale(value.unwrap_or(""));
                },
                "filter" => {
                    call = call.add_filter(value.unwrap_or(""));
                },
                "dimension" => {
                    call = call.add_dimension(value.unwrap_or(""));
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
                                                Vec::new() + &self.gp + &["sort", "locale", "metric", "max-results", "filter", "start-index", "dimension"]
                                                            ));
                    }
                }
            }
        }
        let protocol = CallType::Standard;
        if dry_run {
            Ok(())
        } else {
            assert!(err.issues.len() == 0);
            for scope in self.opt.values_of("url").unwrap_or(Vec::new()).iter() {
                call = call.add_scope(scope);
            }
            let mut ostream = match writer_from_opts(opt.value_of("out")) {
                Ok(mut f) => f,
                Err(io_err) => return Err(DoitError::IoError(opt.value_of("out").unwrap_or("-").to_string(), io_err)),
            };
            match match protocol {
                CallType::Standard => call.doit(),
                _ => unreachable!()
            } {
                Err(api_err) => Err(DoitError::ApiError(api_err)),
                Ok((mut response, output_schema)) => {
                    let mut value = json::value::to_value(&output_schema);
                    remove_json_null_values(&mut value);
                    serde::json::to_writer_pretty(&mut ostream, &value).unwrap();
                    Ok(())
                }
            }
        }
    }

    fn _urlchannels_delete(&self, opt: &ArgMatches<'n, 'a>, dry_run: bool, err: &mut InvalidOptionsError)
                                                    -> Result<(), DoitError> {
        let mut call = self.hub.urlchannels().delete(opt.value_of("ad-client-id").unwrap_or(""), opt.value_of("url-channel-id").unwrap_or(""));
        for parg in opt.values_of("v").unwrap_or(Vec::new()).iter() {
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
                                                Vec::new() + &self.gp + &[]
                                                            ));
                    }
                }
            }
        }
        let protocol = CallType::Standard;
        if dry_run {
            Ok(())
        } else {
            assert!(err.issues.len() == 0);
            for scope in self.opt.values_of("url").unwrap_or(Vec::new()).iter() {
                call = call.add_scope(scope);
            }
            let mut ostream = match writer_from_opts(opt.value_of("out")) {
                Ok(mut f) => f,
                Err(io_err) => return Err(DoitError::IoError(opt.value_of("out").unwrap_or("-").to_string(), io_err)),
            };
            match match protocol {
                CallType::Standard => call.doit(),
                _ => unreachable!()
            } {
                Err(api_err) => Err(DoitError::ApiError(api_err)),
                Ok((mut response, output_schema)) => {
                    let mut value = json::value::to_value(&output_schema);
                    remove_json_null_values(&mut value);
                    serde::json::to_writer_pretty(&mut ostream, &value).unwrap();
                    Ok(())
                }
            }
        }
    }

    fn _urlchannels_insert(&self, opt: &ArgMatches<'n, 'a>, dry_run: bool, err: &mut InvalidOptionsError)
                                                    -> Result<(), DoitError> {
        
        let mut request = api::UrlChannel::default();
        let mut field_cursor = FieldCursor::default();
        for kvarg in opt.values_of("kv").unwrap_or(Vec::new()).iter() {
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
            match &temp_cursor.to_string()[..] {
                "kind" => {
                        request.kind = Some(value.unwrap_or("").to_string());
                    },
                "id" => {
                        request.id = Some(value.unwrap_or("").to_string());
                    },
                "url-pattern" => {
                        request.url_pattern = Some(value.unwrap_or("").to_string());
                    },
                _ => {
                    let suggestion = FieldCursor::did_you_mean(key, &vec!["id", "kind", "url-pattern"]);
                    err.issues.push(CLIError::Field(FieldError::Unknown(temp_cursor.to_string(), suggestion, value.map(|v| v.to_string()))));
                }
            }
        }
        let mut call = self.hub.urlchannels().insert(request, opt.value_of("ad-client-id").unwrap_or(""));
        for parg in opt.values_of("v").unwrap_or(Vec::new()).iter() {
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
                                                Vec::new() + &self.gp + &[]
                                                            ));
                    }
                }
            }
        }
        let protocol = CallType::Standard;
        if dry_run {
            Ok(())
        } else {
            assert!(err.issues.len() == 0);
            for scope in self.opt.values_of("url").unwrap_or(Vec::new()).iter() {
                call = call.add_scope(scope);
            }
            let mut ostream = match writer_from_opts(opt.value_of("out")) {
                Ok(mut f) => f,
                Err(io_err) => return Err(DoitError::IoError(opt.value_of("out").unwrap_or("-").to_string(), io_err)),
            };
            match match protocol {
                CallType::Standard => call.doit(),
                _ => unreachable!()
            } {
                Err(api_err) => Err(DoitError::ApiError(api_err)),
                Ok((mut response, output_schema)) => {
                    let mut value = json::value::to_value(&output_schema);
                    remove_json_null_values(&mut value);
                    serde::json::to_writer_pretty(&mut ostream, &value).unwrap();
                    Ok(())
                }
            }
        }
    }

    fn _urlchannels_list(&self, opt: &ArgMatches<'n, 'a>, dry_run: bool, err: &mut InvalidOptionsError)
                                                    -> Result<(), DoitError> {
        let mut call = self.hub.urlchannels().list(opt.value_of("ad-client-id").unwrap_or(""));
        for parg in opt.values_of("v").unwrap_or(Vec::new()).iter() {
            let (key, value) = parse_kv_arg(&*parg, err, false);
            match key {
                "page-token" => {
                    call = call.page_token(value.unwrap_or(""));
                },
                "max-results" => {
                    call = call.max_results(arg_from_str(value.unwrap_or("-0"), err, "max-results", "integer"));
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
                                                Vec::new() + &self.gp + &["page-token", "max-results"]
                                                            ));
                    }
                }
            }
        }
        let protocol = CallType::Standard;
        if dry_run {
            Ok(())
        } else {
            assert!(err.issues.len() == 0);
            for scope in self.opt.values_of("url").unwrap_or(Vec::new()).iter() {
                call = call.add_scope(scope);
            }
            let mut ostream = match writer_from_opts(opt.value_of("out")) {
                Ok(mut f) => f,
                Err(io_err) => return Err(DoitError::IoError(opt.value_of("out").unwrap_or("-").to_string(), io_err)),
            };
            match match protocol {
                CallType::Standard => call.doit(),
                _ => unreachable!()
            } {
                Err(api_err) => Err(DoitError::ApiError(api_err)),
                Ok((mut response, output_schema)) => {
                    let mut value = json::value::to_value(&output_schema);
                    remove_json_null_values(&mut value);
                    serde::json::to_writer_pretty(&mut ostream, &value).unwrap();
                    Ok(())
                }
            }
        }
    }

    fn _doit(&self, dry_run: bool) -> Result<Result<(), DoitError>, Option<InvalidOptionsError>> {
        let mut err = InvalidOptionsError::new();
        let mut call_result: Result<(), DoitError> = Ok(());
        let mut err_opt: Option<InvalidOptionsError> = None;
        match self.opt.subcommand() {
            ("accounts", Some(opt)) => {
                match opt.subcommand() {
                    ("adclients-get", Some(opt)) => {
                        call_result = self._accounts_adclients_get(opt, dry_run, &mut err);
                    },
                    ("adclients-list", Some(opt)) => {
                        call_result = self._accounts_adclients_list(opt, dry_run, &mut err);
                    },
                    ("adunits-delete", Some(opt)) => {
                        call_result = self._accounts_adunits_delete(opt, dry_run, &mut err);
                    },
                    ("adunits-get", Some(opt)) => {
                        call_result = self._accounts_adunits_get(opt, dry_run, &mut err);
                    },
                    ("adunits-get-ad-code", Some(opt)) => {
                        call_result = self._accounts_adunits_get_ad_code(opt, dry_run, &mut err);
                    },
                    ("adunits-insert", Some(opt)) => {
                        call_result = self._accounts_adunits_insert(opt, dry_run, &mut err);
                    },
                    ("adunits-list", Some(opt)) => {
                        call_result = self._accounts_adunits_list(opt, dry_run, &mut err);
                    },
                    ("adunits-patch", Some(opt)) => {
                        call_result = self._accounts_adunits_patch(opt, dry_run, &mut err);
                    },
                    ("adunits-update", Some(opt)) => {
                        call_result = self._accounts_adunits_update(opt, dry_run, &mut err);
                    },
                    ("get", Some(opt)) => {
                        call_result = self._accounts_get(opt, dry_run, &mut err);
                    },
                    ("list", Some(opt)) => {
                        call_result = self._accounts_list(opt, dry_run, &mut err);
                    },
                    ("reports-generate", Some(opt)) => {
                        call_result = self._accounts_reports_generate(opt, dry_run, &mut err);
                    },
                    _ => {
                        err.issues.push(CLIError::MissingMethodError("accounts".to_string()));
                        writeln!(io::stderr(), "{}\n", opt.usage()).ok();
                    }
                }
            },
            ("adclients", Some(opt)) => {
                match opt.subcommand() {
                    ("get", Some(opt)) => {
                        call_result = self._adclients_get(opt, dry_run, &mut err);
                    },
                    ("list", Some(opt)) => {
                        call_result = self._adclients_list(opt, dry_run, &mut err);
                    },
                    _ => {
                        err.issues.push(CLIError::MissingMethodError("adclients".to_string()));
                        writeln!(io::stderr(), "{}\n", opt.usage()).ok();
                    }
                }
            },
            ("associationsessions", Some(opt)) => {
                match opt.subcommand() {
                    ("start", Some(opt)) => {
                        call_result = self._associationsessions_start(opt, dry_run, &mut err);
                    },
                    ("verify", Some(opt)) => {
                        call_result = self._associationsessions_verify(opt, dry_run, &mut err);
                    },
                    _ => {
                        err.issues.push(CLIError::MissingMethodError("associationsessions".to_string()));
                        writeln!(io::stderr(), "{}\n", opt.usage()).ok();
                    }
                }
            },
            ("customchannels", Some(opt)) => {
                match opt.subcommand() {
                    ("delete", Some(opt)) => {
                        call_result = self._customchannels_delete(opt, dry_run, &mut err);
                    },
                    ("get", Some(opt)) => {
                        call_result = self._customchannels_get(opt, dry_run, &mut err);
                    },
                    ("insert", Some(opt)) => {
                        call_result = self._customchannels_insert(opt, dry_run, &mut err);
                    },
                    ("list", Some(opt)) => {
                        call_result = self._customchannels_list(opt, dry_run, &mut err);
                    },
                    ("patch", Some(opt)) => {
                        call_result = self._customchannels_patch(opt, dry_run, &mut err);
                    },
                    ("update", Some(opt)) => {
                        call_result = self._customchannels_update(opt, dry_run, &mut err);
                    },
                    _ => {
                        err.issues.push(CLIError::MissingMethodError("customchannels".to_string()));
                        writeln!(io::stderr(), "{}\n", opt.usage()).ok();
                    }
                }
            },
            ("reports", Some(opt)) => {
                match opt.subcommand() {
                    ("generate", Some(opt)) => {
                        call_result = self._reports_generate(opt, dry_run, &mut err);
                    },
                    _ => {
                        err.issues.push(CLIError::MissingMethodError("reports".to_string()));
                        writeln!(io::stderr(), "{}\n", opt.usage()).ok();
                    }
                }
            },
            ("urlchannels", Some(opt)) => {
                match opt.subcommand() {
                    ("delete", Some(opt)) => {
                        call_result = self._urlchannels_delete(opt, dry_run, &mut err);
                    },
                    ("insert", Some(opt)) => {
                        call_result = self._urlchannels_insert(opt, dry_run, &mut err);
                    },
                    ("list", Some(opt)) => {
                        call_result = self._urlchannels_list(opt, dry_run, &mut err);
                    },
                    _ => {
                        err.issues.push(CLIError::MissingMethodError("urlchannels".to_string()));
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
    fn new(opt: ArgMatches<'a, 'n>) -> Result<Engine<'a, 'n>, InvalidOptionsError> {
        let (config_dir, secret) = {
            let config_dir = match cmn::assure_config_dir_exists(opt.value_of("folder").unwrap_or("~/.google-service-cli")) {
                Err(e) => return Err(InvalidOptionsError::single(e, 3)),
                Ok(p) => p,
            };

            match cmn::application_secret_from_directory(&config_dir, "adsensehost4d1-secret.json", 
                                                         "{\"installed\":{\"auth_uri\":\"https://accounts.google.com/o/oauth2/auth\",\"client_secret\":\"hCsslbCUyfehWMmbkG8vTYxG\",\"token_uri\":\"https://accounts.google.com/o/oauth2/token\",\"client_email\":\"\",\"redirect_uris\":[\"urn:ietf:wg:oauth:2.0:oob\",\"oob\"],\"client_x509_cert_url\":\"\",\"client_id\":\"620010449518-9ngf7o4dhs0dka470npqvor6dc5lqb9b.apps.googleusercontent.com\",\"auth_provider_x509_cert_url\":\"https://www.googleapis.com/oauth2/v1/certs\"}}") {
                Ok(secret) => (config_dir, secret),
                Err(e) => return Err(InvalidOptionsError::single(e, 4))
            }
        };

        let auth = Authenticator::new(  &secret, DefaultAuthenticatorDelegate,
                                        if opt.is_present("debug-auth") {
                                            hyper::Client::with_connector(mock::TeeConnector {
                                                    connector: hyper::net::HttpConnector(None) 
                                                })
                                        } else {
                                            hyper::Client::new()
                                        },
                                        JsonTokenStorage {
                                          program_name: "adsensehost4d1",
                                          db_dir: config_dir.clone(),
                                        }, None);

        let client = 
            if opt.is_present("debug") {
                hyper::Client::with_connector(mock::TeeConnector {
                        connector: hyper::net::HttpConnector(None) 
                    })
            } else {
                hyper::Client::new()
            };
        let engine = Engine {
            opt: opt,
            hub: api::AdSenseHost::new(client, auth),
            gp: vec!["alt", "fields", "key", "oauth-token", "pretty-print", "quota-user", "user-ip"],
            gpm: vec![
                    ("oauth-token", "oauth_token"),
                    ("pretty-print", "prettyPrint"),
                    ("quota-user", "quotaUser"),
                    ("user-ip", "userIp"),
                ]
        };

        match engine._doit(true) {
            Err(Some(err)) => Err(err),
            Err(None)      => Ok(engine),
            Ok(_)          => unreachable!(),
        }
    }

    fn doit(&self) -> Result<(), DoitError> {
        match self._doit(false) {
            Ok(res) => res,
            Err(_) => unreachable!(),
        }
    }
}

fn main() {
    let arg_data = [
        ("accounts", "methods: 'adclients-get', 'adclients-list', 'adunits-delete', 'adunits-get', 'adunits-get-ad-code', 'adunits-insert', 'adunits-list', 'adunits-patch', 'adunits-update', 'get', 'list' and 'reports-generate'", vec![
            ("adclients-get",  
                    Some(r##"Get information about one of the ad clients in the specified publisher's AdSense account."##),
                    "Details at http://byron.github.io/google-apis-rs/google_adsensehost4d1_cli/accounts_adclients-get",
                  vec![
                    (Some(r##"account-id"##),
                     None,
                     Some(r##"Account which contains the ad client."##),
                     Some(true),
                     Some(false)),
        
                    (Some(r##"ad-client-id"##),
                     None,
                     Some(r##"Ad client to get."##),
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
            ("adclients-list",  
                    Some(r##"List all hosted ad clients in the specified hosted account."##),
                    "Details at http://byron.github.io/google-apis-rs/google_adsensehost4d1_cli/accounts_adclients-list",
                  vec![
                    (Some(r##"account-id"##),
                     None,
                     Some(r##"Account for which to list ad clients."##),
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
            ("adunits-delete",  
                    Some(r##"Delete the specified ad unit from the specified publisher AdSense account."##),
                    "Details at http://byron.github.io/google-apis-rs/google_adsensehost4d1_cli/accounts_adunits-delete",
                  vec![
                    (Some(r##"account-id"##),
                     None,
                     Some(r##"Account which contains the ad unit."##),
                     Some(true),
                     Some(false)),
        
                    (Some(r##"ad-client-id"##),
                     None,
                     Some(r##"Ad client for which to get ad unit."##),
                     Some(true),
                     Some(false)),
        
                    (Some(r##"ad-unit-id"##),
                     None,
                     Some(r##"Ad unit to delete."##),
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
            ("adunits-get",  
                    Some(r##"Get the specified host ad unit in this AdSense account."##),
                    "Details at http://byron.github.io/google-apis-rs/google_adsensehost4d1_cli/accounts_adunits-get",
                  vec![
                    (Some(r##"account-id"##),
                     None,
                     Some(r##"Account which contains the ad unit."##),
                     Some(true),
                     Some(false)),
        
                    (Some(r##"ad-client-id"##),
                     None,
                     Some(r##"Ad client for which to get ad unit."##),
                     Some(true),
                     Some(false)),
        
                    (Some(r##"ad-unit-id"##),
                     None,
                     Some(r##"Ad unit to get."##),
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
            ("adunits-get-ad-code",  
                    Some(r##"Get ad code for the specified ad unit, attaching the specified host custom channels."##),
                    "Details at http://byron.github.io/google-apis-rs/google_adsensehost4d1_cli/accounts_adunits-get-ad-code",
                  vec![
                    (Some(r##"account-id"##),
                     None,
                     Some(r##"Account which contains the ad client."##),
                     Some(true),
                     Some(false)),
        
                    (Some(r##"ad-client-id"##),
                     None,
                     Some(r##"Ad client with contains the ad unit."##),
                     Some(true),
                     Some(false)),
        
                    (Some(r##"ad-unit-id"##),
                     None,
                     Some(r##"Ad unit to get the code for."##),
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
            ("adunits-insert",  
                    Some(r##"Insert the supplied ad unit into the specified publisher AdSense account."##),
                    "Details at http://byron.github.io/google-apis-rs/google_adsensehost4d1_cli/accounts_adunits-insert",
                  vec![
                    (Some(r##"account-id"##),
                     None,
                     Some(r##"Account which will contain the ad unit."##),
                     Some(true),
                     Some(false)),
        
                    (Some(r##"ad-client-id"##),
                     None,
                     Some(r##"Ad client into which to insert the ad unit."##),
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
            ("adunits-list",  
                    Some(r##"List all ad units in the specified publisher's AdSense account."##),
                    "Details at http://byron.github.io/google-apis-rs/google_adsensehost4d1_cli/accounts_adunits-list",
                  vec![
                    (Some(r##"account-id"##),
                     None,
                     Some(r##"Account which contains the ad client."##),
                     Some(true),
                     Some(false)),
        
                    (Some(r##"ad-client-id"##),
                     None,
                     Some(r##"Ad client for which to list ad units."##),
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
            ("adunits-patch",  
                    Some(r##"Update the supplied ad unit in the specified publisher AdSense account. This method supports patch semantics."##),
                    "Details at http://byron.github.io/google-apis-rs/google_adsensehost4d1_cli/accounts_adunits-patch",
                  vec![
                    (Some(r##"account-id"##),
                     None,
                     Some(r##"Account which contains the ad client."##),
                     Some(true),
                     Some(false)),
        
                    (Some(r##"ad-client-id"##),
                     None,
                     Some(r##"Ad client which contains the ad unit."##),
                     Some(true),
                     Some(false)),
        
                    (Some(r##"ad-unit-id"##),
                     None,
                     Some(r##"Ad unit to get."##),
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
            ("adunits-update",  
                    Some(r##"Update the supplied ad unit in the specified publisher AdSense account."##),
                    "Details at http://byron.github.io/google-apis-rs/google_adsensehost4d1_cli/accounts_adunits-update",
                  vec![
                    (Some(r##"account-id"##),
                     None,
                     Some(r##"Account which contains the ad client."##),
                     Some(true),
                     Some(false)),
        
                    (Some(r##"ad-client-id"##),
                     None,
                     Some(r##"Ad client which contains the ad unit."##),
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
            ("get",  
                    Some(r##"Get information about the selected associated AdSense account."##),
                    "Details at http://byron.github.io/google-apis-rs/google_adsensehost4d1_cli/accounts_get",
                  vec![
                    (Some(r##"account-id"##),
                     None,
                     Some(r##"Account to get information about."##),
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
            ("list",  
                    Some(r##"List hosted accounts associated with this AdSense account by ad client id."##),
                    "Details at http://byron.github.io/google-apis-rs/google_adsensehost4d1_cli/accounts_list",
                  vec![
                    (Some(r##"filter-ad-client-id"##),
                     None,
                     Some(r##"Ad clients to list accounts for."##),
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
            ("reports-generate",  
                    Some(r##"Generate an AdSense report based on the report request sent in the query parameters. Returns the result as JSON; to retrieve output in CSV format specify "alt=csv" as a query parameter."##),
                    "Details at http://byron.github.io/google-apis-rs/google_adsensehost4d1_cli/accounts_reports-generate",
                  vec![
                    (Some(r##"account-id"##),
                     None,
                     Some(r##"Hosted account upon which to report."##),
                     Some(true),
                     Some(false)),
        
                    (Some(r##"start-date"##),
                     None,
                     Some(r##"Start of the date range to report on in "YYYY-MM-DD" format, inclusive."##),
                     Some(true),
                     Some(false)),
        
                    (Some(r##"end-date"##),
                     None,
                     Some(r##"End of the date range to report on in "YYYY-MM-DD" format, inclusive."##),
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
            ]),
        
        ("adclients", "methods: 'get' and 'list'", vec![
            ("get",  
                    Some(r##"Get information about one of the ad clients in the Host AdSense account."##),
                    "Details at http://byron.github.io/google-apis-rs/google_adsensehost4d1_cli/adclients_get",
                  vec![
                    (Some(r##"ad-client-id"##),
                     None,
                     Some(r##"Ad client to get."##),
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
            ("list",  
                    Some(r##"List all host ad clients in this AdSense account."##),
                    "Details at http://byron.github.io/google-apis-rs/google_adsensehost4d1_cli/adclients_list",
                  vec![
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
        
        ("associationsessions", "methods: 'start' and 'verify'", vec![
            ("start",  
                    Some(r##"Create an association session for initiating an association with an AdSense user."##),
                    "Details at http://byron.github.io/google-apis-rs/google_adsensehost4d1_cli/associationsessions_start",
                  vec![
                    (Some(r##"product-code"##),
                     None,
                     Some(r##"Products to associate with the user."##),
                     Some(true),
                     Some(false)),
        
                    (Some(r##"website-url"##),
                     None,
                     Some(r##"The URL of the user's hosted website."##),
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
            ("verify",  
                    Some(r##"Verify an association session after the association callback returns from AdSense signup."##),
                    "Details at http://byron.github.io/google-apis-rs/google_adsensehost4d1_cli/associationsessions_verify",
                  vec![
                    (Some(r##"token"##),
                     None,
                     Some(r##"The token returned to the association callback URL."##),
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
            ]),
        
        ("customchannels", "methods: 'delete', 'get', 'insert', 'list', 'patch' and 'update'", vec![
            ("delete",  
                    Some(r##"Delete a specific custom channel from the host AdSense account."##),
                    "Details at http://byron.github.io/google-apis-rs/google_adsensehost4d1_cli/customchannels_delete",
                  vec![
                    (Some(r##"ad-client-id"##),
                     None,
                     Some(r##"Ad client from which to delete the custom channel."##),
                     Some(true),
                     Some(false)),
        
                    (Some(r##"custom-channel-id"##),
                     None,
                     Some(r##"Custom channel to delete."##),
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
            ("get",  
                    Some(r##"Get a specific custom channel from the host AdSense account."##),
                    "Details at http://byron.github.io/google-apis-rs/google_adsensehost4d1_cli/customchannels_get",
                  vec![
                    (Some(r##"ad-client-id"##),
                     None,
                     Some(r##"Ad client from which to get the custom channel."##),
                     Some(true),
                     Some(false)),
        
                    (Some(r##"custom-channel-id"##),
                     None,
                     Some(r##"Custom channel to get."##),
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
            ("insert",  
                    Some(r##"Add a new custom channel to the host AdSense account."##),
                    "Details at http://byron.github.io/google-apis-rs/google_adsensehost4d1_cli/customchannels_insert",
                  vec![
                    (Some(r##"ad-client-id"##),
                     None,
                     Some(r##"Ad client to which the new custom channel will be added."##),
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
            ("list",  
                    Some(r##"List all host custom channels in this AdSense account."##),
                    "Details at http://byron.github.io/google-apis-rs/google_adsensehost4d1_cli/customchannels_list",
                  vec![
                    (Some(r##"ad-client-id"##),
                     None,
                     Some(r##"Ad client for which to list custom channels."##),
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
            ("patch",  
                    Some(r##"Update a custom channel in the host AdSense account. This method supports patch semantics."##),
                    "Details at http://byron.github.io/google-apis-rs/google_adsensehost4d1_cli/customchannels_patch",
                  vec![
                    (Some(r##"ad-client-id"##),
                     None,
                     Some(r##"Ad client in which the custom channel will be updated."##),
                     Some(true),
                     Some(false)),
        
                    (Some(r##"custom-channel-id"##),
                     None,
                     Some(r##"Custom channel to get."##),
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
            ("update",  
                    Some(r##"Update a custom channel in the host AdSense account."##),
                    "Details at http://byron.github.io/google-apis-rs/google_adsensehost4d1_cli/customchannels_update",
                  vec![
                    (Some(r##"ad-client-id"##),
                     None,
                     Some(r##"Ad client in which the custom channel will be updated."##),
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
        
        ("reports", "methods: 'generate'", vec![
            ("generate",  
                    Some(r##"Generate an AdSense report based on the report request sent in the query parameters. Returns the result as JSON; to retrieve output in CSV format specify "alt=csv" as a query parameter."##),
                    "Details at http://byron.github.io/google-apis-rs/google_adsensehost4d1_cli/reports_generate",
                  vec![
                    (Some(r##"start-date"##),
                     None,
                     Some(r##"Start of the date range to report on in "YYYY-MM-DD" format, inclusive."##),
                     Some(true),
                     Some(false)),
        
                    (Some(r##"end-date"##),
                     None,
                     Some(r##"End of the date range to report on in "YYYY-MM-DD" format, inclusive."##),
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
            ]),
        
        ("urlchannels", "methods: 'delete', 'insert' and 'list'", vec![
            ("delete",  
                    Some(r##"Delete a URL channel from the host AdSense account."##),
                    "Details at http://byron.github.io/google-apis-rs/google_adsensehost4d1_cli/urlchannels_delete",
                  vec![
                    (Some(r##"ad-client-id"##),
                     None,
                     Some(r##"Ad client from which to delete the URL channel."##),
                     Some(true),
                     Some(false)),
        
                    (Some(r##"url-channel-id"##),
                     None,
                     Some(r##"URL channel to delete."##),
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
            ("insert",  
                    Some(r##"Add a new URL channel to the host AdSense account."##),
                    "Details at http://byron.github.io/google-apis-rs/google_adsensehost4d1_cli/urlchannels_insert",
                  vec![
                    (Some(r##"ad-client-id"##),
                     None,
                     Some(r##"Ad client to which the new URL channel will be added."##),
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
            ("list",  
                    Some(r##"List all host URL channels in the host AdSense account."##),
                    "Details at http://byron.github.io/google-apis-rs/google_adsensehost4d1_cli/urlchannels_list",
                  vec![
                    (Some(r##"ad-client-id"##),
                     None,
                     Some(r##"Ad client for which to list URL channels."##),
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
            ]),
        
    ];
    
    let mut app = App::new("adsensehost4d1")
           .author("Sebastian Thiel <byronimo@gmail.com>")
           .version("0.2.0+20150307")
           .about("Gives AdSense Hosts access to report generation, ad code generation, and publisher management capabilities.")
           .after_help("All documentation details can be found at http://byron.github.io/google-apis-rs/google_adsensehost4d1_cli")
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
                   .help("Output all server communication to standard error. `tx` and `rx` are placed into the same stream.")
                   .multiple(false)
                   .takes_value(false))
           .arg(Arg::with_name("debug-auth")
                   .long("debug-auth")
                   .help("Output all communication related to authentication to standard error. `tx` and `rx` are placed into the same stream.")
                   .multiple(false)
                   .takes_value(false));
           
           for &(main_command_name, ref about, ref subcommands) in arg_data.iter() {
               let mut mcmd = SubCommand::new(main_command_name).about(about);
           
               for &(sub_command_name, ref desc, url_info, ref args) in subcommands {
                   let mut scmd = SubCommand::new(sub_command_name);
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
                       let mut arg = Arg::with_name(arg_name_str);
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
    match Engine::new(matches) {
        Err(err) => {
            env::set_exit_status(err.exit_code);
            writeln!(io::stderr(), "{}", err).ok();
        },
        Ok(engine) => {
            if let Err(doit_err) = engine.doit() {
                env::set_exit_status(1);
                match doit_err {
                    DoitError::IoError(path, err) => {
                        writeln!(io::stderr(), "Failed to open output file '{}': {}", path, err).ok();
                    },
                    DoitError::ApiError(err) => {
                        if debug {
                            writeln!(io::stderr(), "{:?}", err).ok();
                        } else {
                            writeln!(io::stderr(), "{}", err).ok();
                        }
                    }
                }
            }
        }
    }
}