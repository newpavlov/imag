//
// imag - the personal information management suite for the commandline
// Copyright (C) 2015-2018 Matthias Beyer <mail@beyermatthias.de> and contributors
//
// This library is free software; you can redistribute it and/or
// modify it under the terms of the GNU Lesser General Public
// License as published by the Free Software Foundation; version
// 2.1 of the License.
//
// This library is distributed in the hope that it will be useful,
// but WITHOUT ANY WARRANTY; without even the implied warranty of
// MERCHANTABILITY or FITNESS FOR A PARTICULAR PURPOSE.  See the GNU
// Lesser General Public License for more details.
//
// You should have received a copy of the GNU Lesser General Public
// License along with this library; if not, write to the Free Software
// Foundation, Inc., 51 Franklin Street, Fifth Floor, Boston, MA  02110-1301  USA
//

use std::path::PathBuf;
use std::process::exit;

use walkdir::WalkDir;
use walkdir::DirEntry;
use clap::ArgMatches;

use libimagrt::runtime::Runtime;
use libimagerror::iter::TraceIterator;
use libimagerror::trace::MapErrTrace;
use libimagcalendar::store::calendars::CalendarStore;
use libimagcalendar::store::collections::CalendarCollectionStore;
use libimagentryref::reference::Ref;
use libimagcalendar::collection::Collection;
use libimagstore::iter::get::StoreIdGetIteratorExtension;
use libimagutil::warn_result::*;

pub fn collection(rt: &Runtime) {
    let scmd = rt.cli().subcommand_matches("collection").unwrap(); // safed by main()

    match scmd.subcommand() {
        ("add", scmd)    => add(rt, scmd.unwrap()),
        ("remove", scmd) => remove(rt, scmd.unwrap()),
        ("show", scmd)   => show(rt, scmd.unwrap()),
        ("list", scmd)   => list(rt, scmd.unwrap()),
        ("find", scmd)   => find(rt, scmd.unwrap()),
        _ => {
            unimplemented!()
        }
    }
}

fn add<'a>(rt: &Runtime, scmd: &ArgMatches<'a>) {
    let name = scmd.value_of("collection-add-name").map(String::from).unwrap(); // safe by clap
    let path = scmd.value_of("collection-add-path").map(PathBuf::from).unwrap(); // safe by clap

    if !path.is_dir() { // TODO: Move this check to libimagcalendar
        error!("Path '{:?}' is not a directory", path);
        exit(1)
    } else {

        let mut collection = rt.store()
            .retrieve_calendar_collection(path.clone())
            .map_err_trace_exit_unwrap(1);

        info!("Collection added");

        let is_not_hidden = |entry: &DirEntry| !entry
            .file_name()
            .to_str()
            .map(|s| s.starts_with("."))
            .unwrap_or(false);

        for entry in WalkDir::new(path).follow_links(false).into_iter().filter_entry(is_not_hidden) {
            match entry {
                Ok(de) => {
                    if de.file_type().is_file() {
                        let entry = collection
                            .add_retrieve_calendar_from_path(de.path(), rt.store())
                            .map_err_trace_exit_unwrap(1);

                        info!("Created entry: {} -> {}", entry.get_location(), de.path().display());
                    } else {
                        debug!("Ignored: {}", de.path().display());
                        /* ignored */
                    }
                },

                Err(e) => {
                    error!("Error processing directory entry: {:?}", e);
                },
            }
        }

        debug!("Ready");
    }
}

fn remove<'a>(rt: &Runtime, scmd: &ArgMatches<'a>) {
    let name = scmd.value_of("collection-remove-name").map(String::from).unwrap(); // safe by clap

    let collection_hash = {
        let collection = rt
            .store()
            .get_calendar_collection(&name)
            .map_err_trace_exit_unwrap(1)
            .unwrap_or_else(|| {
                error!("No callendar collection named {}", name);
                exit(1)
            });

        let hash = collection
            .get_hash()
            .map(String::from)
            .map_err_trace_exit_unwrap(1);

        let errstr = format!("Failed to get entry from store for collection {}", hash);

        collection
            .calendars()
            .map_err_trace_exit_unwrap(1)
            .into_get_iter(rt.store())
            .map(|e| e.map_warn_err_str(&errstr))
            .trace_unwrap_exit(1)
            .filter_map(|o| o)
            .map(|e| {
                let hash = e.get_hash().map(String::from).map_err_trace_exit_unwrap(1);
                debug!("Entry: {} -> Hash: {}", e.get_location(), hash);
                hash
            })
            .for_each(|hash| {
                debug!("Deleting {}", hash);
                rt.store()
                    .delete_calendar_by_hash(hash)
                    .map_err_trace_exit_unwrap(1);
            });

        hash
    };

    rt.store()
        .delete_calendar_collection_by_hash(collection_hash)
        .map_err_trace_exit_unwrap(1);
}

fn show<'a>(rt: &Runtime, scmd: &ArgMatches<'a>) {
    unimplemented!()
}

fn list<'a>(rt: &Runtime, scmd: &ArgMatches<'a>) {
    unimplemented!()
}

fn find<'a>(rt: &Runtime, scmd: &ArgMatches<'a>) {
    unimplemented!()
}
