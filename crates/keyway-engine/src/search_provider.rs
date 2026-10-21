// SPDX-FileCopyrightText: Keyway contributors
// SPDX-License-Identifier: GPL-3.0-or-later

//! `org.gnome.Shell.SearchProvider2` for Activities search.
//!
//! Until the catalog exists (Phase 2), the provider answers with a single result confirming
//! that Keyway is installed, offered when every search term is a prefix of "keyway".

use std::collections::HashMap;

use gio::prelude::*;
use glib::variant::ToVariant;

use crate::application::Engine;

const INTERFACE_XML: &str = include_str!("search_provider.xml");

const INSTALLED_RESULT_ID: &str = "keyway-installed";

/// Shorter terms would put the result under unrelated searches such as "k".
const MIN_TERM_CHARS: usize = 3;

#[derive(Debug, PartialEq)]
enum Call {
    GetInitialResultSet {
        terms: Vec<String>,
    },
    GetSubsearchResultSet {
        previous: Vec<String>,
        terms: Vec<String>,
    },
    GetResultMetas {
        ids: Vec<String>,
    },
    ActivateResult,
    LaunchSearch,
    Cancel,
}

impl DBusMethodCall for Call {
    fn parse_call(
        _object_path: &str,
        _interface: Option<&str>,
        method: &str,
        params: glib::Variant,
    ) -> Result<Self, glib::Error> {
        // GDBus has already checked the parameters against the introspection data, so `get`
        // only fails if this table and search_provider.xml disagree.
        let call = match method {
            "GetInitialResultSet" => params
                .get::<(Vec<String>,)>()
                .map(|(terms,)| Self::GetInitialResultSet { terms }),
            "GetSubsearchResultSet" => params
                .get::<(Vec<String>, Vec<String>)>()
                .map(|(previous, terms)| Self::GetSubsearchResultSet { previous, terms }),
            "GetResultMetas" => params
                .get::<(Vec<String>,)>()
                .map(|(ids,)| Self::GetResultMetas { ids }),
            "ActivateResult" => Some(Self::ActivateResult),
            "LaunchSearch" => Some(Self::LaunchSearch),
            "XUbuntuCancel" => Some(Self::Cancel),
            _ => None,
        };
        call.ok_or_else(|| {
            glib::Error::new(
                gio::DBusError::UnknownMethod,
                &format!("Unknown method {method}"),
            )
        })
    }
}

pub(crate) fn register(
    engine: &Engine,
    connection: &gio::DBusConnection,
) -> Result<gio::RegistrationId, glib::Error> {
    let node = gio::DBusNodeInfo::for_xml(INTERFACE_XML)?;
    let interface = node
        .lookup_interface(keyway_engine::SEARCH_PROVIDER_INTERFACE)
        .ok_or_else(|| {
            glib::Error::new(
                gio::DBusError::UnknownInterface,
                "search_provider.xml does not define SearchProvider2",
            )
        })?;
    let engine = engine.downgrade();

    connection
        .register_object(keyway_engine::SEARCH_PROVIDER_PATH, &interface)
        .typed_method_call::<Call>()
        .invoke_and_return(move |_connection, _sender, call| {
            // Each request restarts the inactivity timeout when the guard is dropped.
            let _hold = engine.upgrade().map(|engine| engine.hold());
            Ok(reply(call))
        })
        .build()
}

fn reply(call: Call) -> Option<glib::Variant> {
    match call {
        Call::GetInitialResultSet { terms } => Some((search(&terms),).to_variant()),
        Call::GetSubsearchResultSet { previous, terms } => {
            let results: Vec<String> = search(&terms)
                .into_iter()
                .filter(|id| previous.contains(id))
                .collect();
            Some((results,).to_variant())
        }
        Call::GetResultMetas { ids } => {
            let metas: Vec<_> = ids.iter().filter_map(|id| result_meta(id)).collect();
            Some((metas,).to_variant())
        }
        // There is no bar or preferences app to show yet (Phases 3 and 5), and no search runs
        // long enough to cancel.
        Call::ActivateResult | Call::LaunchSearch | Call::Cancel => None,
    }
}

fn search(terms: &[String]) -> Vec<String> {
    let matches = !terms.is_empty()
        && terms.iter().all(|term| {
            term.chars().count() >= MIN_TERM_CHARS && "keyway".starts_with(&term.to_lowercase())
        });
    if matches {
        vec![INSTALLED_RESULT_ID.to_owned()]
    } else {
        Vec::new()
    }
}

fn result_meta(id: &str) -> Option<HashMap<&'static str, glib::Variant>> {
    if id != INSTALLED_RESULT_ID {
        return None;
    }
    let mut meta = HashMap::from([
        ("id", INSTALLED_RESULT_ID.to_variant()),
        ("name", "Keyway".to_variant()),
        ("description", "Keyway is installed".to_variant()),
    ]);
    // The Shell expects g_icon_serialize() output under "icon", not a GIcon string.
    if let Some(icon) = gio::ThemedIcon::new(keyway_engine::APP_ID).serialize() {
        meta.insert("icon", icon);
    }
    Some(meta)
}

#[cfg(test)]
mod tests {
    use super::*;

    fn terms(words: &[&str]) -> Vec<String> {
        words.iter().map(|word| (*word).to_owned()).collect()
    }

    #[test]
    fn prefixes_of_keyway_find_the_result() {
        for query in [&["key"][..], &["KEYWAY"], &["Keyw"], &["key", "keyway"]] {
            assert_eq!(search(&terms(query)), [INSTALLED_RESULT_ID], "{query:?}");
        }
    }

    #[test]
    fn other_terms_find_nothing() {
        for query in [&[][..], &["ke"], &["keyways"], &["way"], &["key", "files"]] {
            assert_eq!(search(&terms(query)), Vec::<String>::new(), "{query:?}");
        }
    }

    #[test]
    fn subsearch_keeps_only_previous_results() {
        let call = Call::GetSubsearchResultSet {
            previous: Vec::new(),
            terms: terms(&["key"]),
        };
        let reply = reply(call).map(|variant| variant.get::<(Vec<String>,)>());
        assert_eq!(reply, Some(Some((Vec::new(),))));
    }

    #[test]
    fn metas_describe_only_known_results() {
        let meta = result_meta(INSTALLED_RESULT_ID).expect("known result");
        assert_eq!(meta["name"].str(), Some("Keyway"));
        assert!(meta.contains_key("icon"));
        assert!(result_meta("unknown").is_none());
    }

    #[test]
    fn unknown_methods_are_rejected() {
        let error =
            Call::parse_call("/", None, "Frobnicate", ().to_variant()).expect_err("unknown method");
        assert!(error.matches(gio::DBusError::UnknownMethod));
    }
}
