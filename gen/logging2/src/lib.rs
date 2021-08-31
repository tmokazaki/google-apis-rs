// DO NOT EDIT !
// This file was generated automatically from 'src/mako/api/lib.rs.mako'
// DO NOT EDIT !

//! This documentation was generated from *Logging* crate version *2.0.8+20210325*, where *20210325* is the exact revision of the *logging:v2* schema built by the [mako](http://www.makotemplates.org/) code generator *v2.0.8*.
//! 
//! Everything else about the *Logging* *v2* API can be found at the
//! [official documentation site](https://cloud.google.com/logging/docs/).
//! The original source code is [on github](https://github.com/Byron/google-apis-rs/tree/main/gen/logging2).
//! # Features
//! 
//! Handle the following *Resources* with ease from the central [hub](Logging) ... 
//! 
//! * billing accounts
//!  * [*buckets get*](api::BillingAccountBucketGetCall), [*buckets views get*](api::BillingAccountBucketViewGetCall), [*exclusions create*](api::BillingAccountExclusionCreateCall), [*exclusions delete*](api::BillingAccountExclusionDeleteCall), [*exclusions get*](api::BillingAccountExclusionGetCall), [*exclusions list*](api::BillingAccountExclusionListCall), [*exclusions patch*](api::BillingAccountExclusionPatchCall), [*locations buckets create*](api::BillingAccountLocationBucketCreateCall), [*locations buckets delete*](api::BillingAccountLocationBucketDeleteCall), [*locations buckets list*](api::BillingAccountLocationBucketListCall), [*locations buckets patch*](api::BillingAccountLocationBucketPatchCall), [*locations buckets undelete*](api::BillingAccountLocationBucketUndeleteCall), [*locations buckets views create*](api::BillingAccountLocationBucketViewCreateCall), [*locations buckets views delete*](api::BillingAccountLocationBucketViewDeleteCall), [*locations buckets views list*](api::BillingAccountLocationBucketViewListCall), [*locations buckets views patch*](api::BillingAccountLocationBucketViewPatchCall), [*locations get*](api::BillingAccountLocationGetCall), [*locations list*](api::BillingAccountLocationListCall), [*logs delete*](api::BillingAccountLogDeleteCall), [*logs list*](api::BillingAccountLogListCall), [*sinks create*](api::BillingAccountSinkCreateCall), [*sinks delete*](api::BillingAccountSinkDeleteCall), [*sinks get*](api::BillingAccountSinkGetCall), [*sinks list*](api::BillingAccountSinkListCall), [*sinks patch*](api::BillingAccountSinkPatchCall) and [*sinks update*](api::BillingAccountSinkUpdateCall)
//! * entries
//!  * [*list*](api::EntryListCall), [*tail*](api::EntryTailCall) and [*write*](api::EntryWriteCall)
//! * exclusions
//!  * [*create*](api::ExclusionCreateCall), [*delete*](api::ExclusionDeleteCall), [*get*](api::ExclusionGetCall), [*list*](api::ExclusionListCall) and [*patch*](api::ExclusionPatchCall)
//! * folders
//!  * [*exclusions create*](api::FolderExclusionCreateCall), [*exclusions delete*](api::FolderExclusionDeleteCall), [*exclusions get*](api::FolderExclusionGetCall), [*exclusions list*](api::FolderExclusionListCall), [*exclusions patch*](api::FolderExclusionPatchCall), [*locations buckets create*](api::FolderLocationBucketCreateCall), [*locations buckets delete*](api::FolderLocationBucketDeleteCall), [*locations buckets get*](api::FolderLocationBucketGetCall), [*locations buckets list*](api::FolderLocationBucketListCall), [*locations buckets patch*](api::FolderLocationBucketPatchCall), [*locations buckets undelete*](api::FolderLocationBucketUndeleteCall), [*locations buckets views create*](api::FolderLocationBucketViewCreateCall), [*locations buckets views delete*](api::FolderLocationBucketViewDeleteCall), [*locations buckets views get*](api::FolderLocationBucketViewGetCall), [*locations buckets views list*](api::FolderLocationBucketViewListCall), [*locations buckets views patch*](api::FolderLocationBucketViewPatchCall), [*locations get*](api::FolderLocationGetCall), [*locations list*](api::FolderLocationListCall), [*logs delete*](api::FolderLogDeleteCall), [*logs list*](api::FolderLogListCall), [*sinks create*](api::FolderSinkCreateCall), [*sinks delete*](api::FolderSinkDeleteCall), [*sinks get*](api::FolderSinkGetCall), [*sinks list*](api::FolderSinkListCall), [*sinks patch*](api::FolderSinkPatchCall) and [*sinks update*](api::FolderSinkUpdateCall)
//! * [locations](api::Location)
//!  * [*buckets create*](api::LocationBucketCreateCall), [*buckets delete*](api::LocationBucketDeleteCall), [*buckets get*](api::LocationBucketGetCall), [*buckets list*](api::LocationBucketListCall), [*buckets patch*](api::LocationBucketPatchCall), [*buckets undelete*](api::LocationBucketUndeleteCall), [*buckets views create*](api::LocationBucketViewCreateCall), [*buckets views delete*](api::LocationBucketViewDeleteCall), [*buckets views get*](api::LocationBucketViewGetCall), [*buckets views list*](api::LocationBucketViewListCall), [*buckets views patch*](api::LocationBucketViewPatchCall), [*get*](api::LocationGetCall) and [*list*](api::LocationListCall)
//! * logs
//!  * [*delete*](api::LogDeleteCall) and [*list*](api::LogListCall)
//! * [monitored resource descriptors](api::MonitoredResourceDescriptor)
//!  * [*list*](api::MonitoredResourceDescriptorListCall)
//! * organizations
//!  * [*exclusions create*](api::OrganizationExclusionCreateCall), [*exclusions delete*](api::OrganizationExclusionDeleteCall), [*exclusions get*](api::OrganizationExclusionGetCall), [*exclusions list*](api::OrganizationExclusionListCall), [*exclusions patch*](api::OrganizationExclusionPatchCall), [*get cmek settings*](api::OrganizationGetCmekSettingCall), [*locations buckets create*](api::OrganizationLocationBucketCreateCall), [*locations buckets delete*](api::OrganizationLocationBucketDeleteCall), [*locations buckets get*](api::OrganizationLocationBucketGetCall), [*locations buckets list*](api::OrganizationLocationBucketListCall), [*locations buckets patch*](api::OrganizationLocationBucketPatchCall), [*locations buckets undelete*](api::OrganizationLocationBucketUndeleteCall), [*locations buckets views create*](api::OrganizationLocationBucketViewCreateCall), [*locations buckets views delete*](api::OrganizationLocationBucketViewDeleteCall), [*locations buckets views get*](api::OrganizationLocationBucketViewGetCall), [*locations buckets views list*](api::OrganizationLocationBucketViewListCall), [*locations buckets views patch*](api::OrganizationLocationBucketViewPatchCall), [*locations get*](api::OrganizationLocationGetCall), [*locations list*](api::OrganizationLocationListCall), [*logs delete*](api::OrganizationLogDeleteCall), [*logs list*](api::OrganizationLogListCall), [*sinks create*](api::OrganizationSinkCreateCall), [*sinks delete*](api::OrganizationSinkDeleteCall), [*sinks get*](api::OrganizationSinkGetCall), [*sinks list*](api::OrganizationSinkListCall), [*sinks patch*](api::OrganizationSinkPatchCall), [*sinks update*](api::OrganizationSinkUpdateCall) and [*update cmek settings*](api::OrganizationUpdateCmekSettingCall)
//! * projects
//!  * [*exclusions create*](api::ProjectExclusionCreateCall), [*exclusions delete*](api::ProjectExclusionDeleteCall), [*exclusions get*](api::ProjectExclusionGetCall), [*exclusions list*](api::ProjectExclusionListCall), [*exclusions patch*](api::ProjectExclusionPatchCall), [*locations buckets create*](api::ProjectLocationBucketCreateCall), [*locations buckets delete*](api::ProjectLocationBucketDeleteCall), [*locations buckets get*](api::ProjectLocationBucketGetCall), [*locations buckets list*](api::ProjectLocationBucketListCall), [*locations buckets patch*](api::ProjectLocationBucketPatchCall), [*locations buckets undelete*](api::ProjectLocationBucketUndeleteCall), [*locations buckets views create*](api::ProjectLocationBucketViewCreateCall), [*locations buckets views delete*](api::ProjectLocationBucketViewDeleteCall), [*locations buckets views get*](api::ProjectLocationBucketViewGetCall), [*locations buckets views list*](api::ProjectLocationBucketViewListCall), [*locations buckets views patch*](api::ProjectLocationBucketViewPatchCall), [*locations get*](api::ProjectLocationGetCall), [*locations list*](api::ProjectLocationListCall), [*logs delete*](api::ProjectLogDeleteCall), [*logs list*](api::ProjectLogListCall), [*metrics create*](api::ProjectMetricCreateCall), [*metrics delete*](api::ProjectMetricDeleteCall), [*metrics get*](api::ProjectMetricGetCall), [*metrics list*](api::ProjectMetricListCall), [*metrics update*](api::ProjectMetricUpdateCall), [*sinks create*](api::ProjectSinkCreateCall), [*sinks delete*](api::ProjectSinkDeleteCall), [*sinks get*](api::ProjectSinkGetCall), [*sinks list*](api::ProjectSinkListCall), [*sinks patch*](api::ProjectSinkPatchCall) and [*sinks update*](api::ProjectSinkUpdateCall)
//! * sinks
//!  * [*create*](api::SinkCreateCall), [*delete*](api::SinkDeleteCall), [*get*](api::SinkGetCall), [*list*](api::SinkListCall) and [*update*](api::SinkUpdateCall)
//! 
//! Other activities are ...
//! 
//! * [get cmek settings](api::MethodGetCmekSettingCall)
//! * [update cmek settings](api::MethodUpdateCmekSettingCall)
//! 
//! 
//! 
//! Not what you are looking for ? Find all other Google APIs in their Rust [documentation index](http://byron.github.io/google-apis-rs).
//! 
//! # Structure of this Library
//! 
//! The API is structured into the following primary items:
//! 
//! * **[Hub](Logging)**
//!     * a central object to maintain state and allow accessing all *Activities*
//!     * creates [*Method Builders*](client::MethodsBuilder) which in turn
//!       allow access to individual [*Call Builders*](client::CallBuilder)
//! * **[Resources](client::Resource)**
//!     * primary types that you can apply *Activities* to
//!     * a collection of properties and *Parts*
//!     * **[Parts](client::Part)**
//!         * a collection of properties
//!         * never directly used in *Activities*
//! * **[Activities](client::CallBuilder)**
//!     * operations to apply to *Resources*
//! 
//! All *structures* are marked with applicable traits to further categorize them and ease browsing.
//! 
//! Generally speaking, you can invoke *Activities* like this:
//! 
//! ```Rust,ignore
//! let r = hub.resource().activity(...).doit().await
//! ```
//! 
//! Or specifically ...
//! 
//! ```ignore
//! let r = hub.billing_accounts().exclusions_delete(...).doit().await
//! let r = hub.billing_accounts().locations_buckets_views_delete(...).doit().await
//! let r = hub.billing_accounts().locations_buckets_delete(...).doit().await
//! let r = hub.billing_accounts().locations_buckets_undelete(...).doit().await
//! let r = hub.billing_accounts().logs_delete(...).doit().await
//! let r = hub.billing_accounts().sinks_delete(...).doit().await
//! let r = hub.exclusions().delete(...).doit().await
//! let r = hub.folders().exclusions_delete(...).doit().await
//! let r = hub.folders().locations_buckets_views_delete(...).doit().await
//! let r = hub.folders().locations_buckets_delete(...).doit().await
//! let r = hub.folders().locations_buckets_undelete(...).doit().await
//! let r = hub.folders().logs_delete(...).doit().await
//! let r = hub.folders().sinks_delete(...).doit().await
//! let r = hub.locations().buckets_views_delete(...).doit().await
//! let r = hub.locations().buckets_delete(...).doit().await
//! let r = hub.locations().buckets_undelete(...).doit().await
//! let r = hub.logs().delete(...).doit().await
//! let r = hub.organizations().exclusions_delete(...).doit().await
//! let r = hub.organizations().locations_buckets_views_delete(...).doit().await
//! let r = hub.organizations().locations_buckets_delete(...).doit().await
//! let r = hub.organizations().locations_buckets_undelete(...).doit().await
//! let r = hub.organizations().logs_delete(...).doit().await
//! let r = hub.organizations().sinks_delete(...).doit().await
//! let r = hub.projects().exclusions_delete(...).doit().await
//! let r = hub.projects().locations_buckets_views_delete(...).doit().await
//! let r = hub.projects().locations_buckets_delete(...).doit().await
//! let r = hub.projects().locations_buckets_undelete(...).doit().await
//! let r = hub.projects().logs_delete(...).doit().await
//! let r = hub.projects().metrics_delete(...).doit().await
//! let r = hub.projects().sinks_delete(...).doit().await
//! let r = hub.sinks().delete(...).doit().await
//! ```
//! 
//! The `resource()` and `activity(...)` calls create [builders][builder-pattern]. The second one dealing with `Activities` 
//! supports various methods to configure the impending operation (not shown here). It is made such that all required arguments have to be 
//! specified right away (i.e. `(...)`), whereas all optional ones can be [build up][builder-pattern] as desired.
//! The `doit()` method performs the actual communication with the server and returns the respective result.
//! 
//! # Usage
//! 
//! ## Setting up your Project
//! 
//! To use this library, you would put the following lines into your `Cargo.toml` file:
//! 
//! ```toml
//! [dependencies]
//! google-logging2 = "*"
//! hyper = "^0.14"
//! hyper-rustls = "^0.22"
//! serde = "^1.0"
//! serde_json = "^1.0"
//! yup-oauth2 = "^5.0"
//! ```
//! 
//! ## A complete example
//! 
//! ```test_harness,no_run
//! extern crate hyper;
//! extern crate hyper_rustls;
//! extern crate yup_oauth2 as oauth2;
//! extern crate google_logging2 as logging2;
//! use logging2::api::UndeleteBucketRequest;
//! use logging2::{Result, Error};
//! # async fn dox() {
//! use std::default::Default;
//! use oauth2;
//! use logging2::Logging;
//! 
//! // Get an ApplicationSecret instance by some means. It contains the `client_id` and 
//! // `client_secret`, among other things.
//! let secret: oauth2::ApplicationSecret = Default::default();
//! // Instantiate the authenticator. It will choose a suitable authentication flow for you, 
//! // unless you replace  `None` with the desired Flow.
//! // Provide your own `AuthenticatorDelegate` to adjust the way it operates and get feedback about 
//! // what's going on. You probably want to bring in your own `TokenStorage` to persist tokens and
//! // retrieve them from storage.
//! let auth = yup_oauth2::InstalledFlowAuthenticator::builder(
//!         secret,
//!         yup_oauth2::InstalledFlowReturnMethod::HTTPRedirect,
//!     ).build().await.unwrap();
//! let mut hub = Logging::new(hyper::Client::builder().build(hyper_rustls::HttpsConnector::with_native_roots()), auth);
//! // As the method needs a request, you would usually fill it with the desired information
//! // into the respective structure. Some of the parts shown here might not be applicable !
//! // Values shown here are possibly random and not representative !
//! let mut req = UndeleteBucketRequest::default();
//! 
//! // You can configure optional parameters by calling the respective setters at will, and
//! // execute the final call using `doit()`.
//! // Values shown here are possibly random and not representative !
//! let result = hub.billing_accounts().locations_buckets_undelete(req, "name")
//!              .doit().await;
//! 
//! match result {
//!     Err(e) => match e {
//!         // The Error enum provides details about what exactly happened.
//!         // You can also just use its `Debug`, `Display` or `Error` traits
//!          Error::HttpError(_)
//!         |Error::Io(_)
//!         |Error::MissingAPIKey
//!         |Error::MissingToken(_)
//!         |Error::Cancelled
//!         |Error::UploadSizeLimitExceeded(_, _)
//!         |Error::Failure(_)
//!         |Error::BadRequest(_)
//!         |Error::FieldClash(_)
//!         |Error::JsonDecodeError(_, _) => println!("{}", e),
//!     },
//!     Ok(res) => println!("Success: {:?}", res),
//! }
//! # }
//! ```
//! ## Handling Errors
//! 
//! All errors produced by the system are provided either as [Result](client::Result) enumeration as return value of
//! the doit() methods, or handed as possibly intermediate results to either the 
//! [Hub Delegate](client::Delegate), or the [Authenticator Delegate](https://docs.rs/yup-oauth2/*/yup_oauth2/trait.AuthenticatorDelegate.html).
//! 
//! When delegates handle errors or intermediate values, they may have a chance to instruct the system to retry. This 
//! makes the system potentially resilient to all kinds of errors.
//! 
//! ## Uploads and Downloads
//! If a method supports downloads, the response body, which is part of the [Result](client::Result), should be
//! read by you to obtain the media.
//! If such a method also supports a [Response Result](client::ResponseResult), it will return that by default.
//! You can see it as meta-data for the actual media. To trigger a media download, you will have to set up the builder by making
//! this call: `.param("alt", "media")`.
//! 
//! Methods supporting uploads can do so using up to 2 different protocols: 
//! *simple* and *resumable*. The distinctiveness of each is represented by customized 
//! `doit(...)` methods, which are then named `upload(...)` and `upload_resumable(...)` respectively.
//! 
//! ## Customization and Callbacks
//! 
//! You may alter the way an `doit()` method is called by providing a [delegate](client::Delegate) to the 
//! [Method Builder](client::CallBuilder) before making the final `doit()` call. 
//! Respective methods will be called to provide progress information, as well as determine whether the system should 
//! retry on failure.
//! 
//! The [delegate trait](client::Delegate) is default-implemented, allowing you to customize it with minimal effort.
//! 
//! ## Optional Parts in Server-Requests
//! 
//! All structures provided by this library are made to be [encodable](client::RequestValue) and 
//! [decodable](client::ResponseResult) via *json*. Optionals are used to indicate that partial requests are responses 
//! are valid.
//! Most optionals are are considered [Parts](client::Part) which are identifiable by name, which will be sent to 
//! the server to indicate either the set parts of the request or the desired parts in the response.
//! 
//! ## Builder Arguments
//! 
//! Using [method builders](client::CallBuilder), you are able to prepare an action call by repeatedly calling it's methods.
//! These will always take a single argument, for which the following statements are true.
//! 
//! * [PODs][wiki-pod] are handed by copy
//! * strings are passed as `&str`
//! * [request values](client::RequestValue) are moved
//! 
//! Arguments will always be copied or cloned into the builder, to make them independent of their original life times.
//! 
//! [wiki-pod]: http://en.wikipedia.org/wiki/Plain_old_data_structure
//! [builder-pattern]: http://en.wikipedia.org/wiki/Builder_pattern
//! [google-go-api]: https://github.com/google/google-api-go-client
//! 
//! 

// Unused attributes happen thanks to defined, but unused structures
// We don't warn about this, as depending on the API, some data structures or facilities are never used.
// Instead of pre-determining this, we just disable the lint. It's manually tuned to not have any
// unused imports in fully featured APIs. Same with unused_mut ... .
#![allow(unused_imports, unused_mut, dead_code)]

// DO NOT EDIT !
// This file was generated automatically from 'src/mako/api/lib.rs.mako'
// DO NOT EDIT !

#[macro_use]
extern crate serde_derive;

extern crate hyper;
extern crate serde;
extern crate serde_json;
extern crate yup_oauth2 as oauth2;
extern crate mime;
extern crate url;

pub mod api;
pub mod client;

// Re-export the hub type and some basic client structs
pub use api::Logging;
pub use client::{Result, Error, Delegate};
