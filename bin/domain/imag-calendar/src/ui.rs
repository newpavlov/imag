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

use clap::{Arg, App, SubCommand};

pub fn build_ui<'a>(app: App<'a, 'a>) -> App<'a, 'a> {
    app
        .subcommand(SubCommand::with_name("collection")
                   .about("Manage calendar collection")
                   .version("0.1")
                   .subcommand(SubCommand::with_name("add")
                              .about("Add calendar collection")
                              .version("0.1")
                              .arg(Arg::with_name("collection-add-name")
                                   .index(1)
                                   .takes_value(true)
                                   .required(true)
                                   .multiple(false)
                                   .value_name("name")
                                   .help("The name of the collection to add"))
                              .arg(Arg::with_name("collection-add-path")
                                   .index(2)
                                   .takes_value(true)
                                   .required(true)
                                   .multiple(false)
                                   .value_name("path")
                                   .help("The path of the collection to add"))
                              )

                   .subcommand(SubCommand::with_name("remove")
                              .about("Remove calendar collection")
                              .version("0.1")
                              .arg(Arg::with_name("collection-remove-name")
                                   .index(1)
                                   .takes_value(true)
                                   .required(true)
                                   .multiple(false)
                                   .value_name("name")
                                   .help("The name of the collection to remove"))
                              )

                   .subcommand(SubCommand::with_name("show")
                              .about("Show events in calendar collection")
                              .version("0.1")
                              .arg(Arg::with_name("collection-show-name")
                                   .index(1)
                                   .takes_value(true)
                                   .required(true)
                                   .multiple(false)
                                   .value_name("name")
                                   .help("The name of the collection to show"))
                              )

                   .subcommand(SubCommand::with_name("list")
                              .about("List events in calendar collection")
                              .version("0.1")
                              .arg(Arg::with_name("collection-list-name")
                                   .index(1)
                                   .takes_value(true)
                                   .required(true)
                                   .multiple(false)
                                   .value_name("name")
                                   .help("The name of the collection to list"))
                              )

                   .subcommand(SubCommand::with_name("find")
                              .about("Find entries in calendar collection")
                              .version("0.1")
                              .arg(Arg::with_name("collection-find-past")
                                   .long("past")
                                   .short("p")
                                   .takes_value(false)
                                   .required(false)
                                   .multiple(false)
                                   .help("Find not only from today and future, but also from past"))

                              .arg(Arg::with_name("collection-find-show")
                                   .long("show")
                                   .short("s")
                                   .takes_value(false)
                                   .required(false)
                                   .multiple(false)
                                   .help("Do not list found entries, but 'show' them."))

                              .arg(Arg::with_name("collection-find-name")
                                   .index(1)
                                   .takes_value(true)
                                   .required(false)
                                   .multiple(false)
                                   .value_name("name")
                                   .help("The name of the collection to search for entries in."))

                              .arg(Arg::with_name("collection-find-grep")
                                   .index(1)
                                   .takes_value(true)
                                   .required(false)
                                   .multiple(false)
                                   .value_name("pattern")
                                   .help("Grep for this pattern in entry meta data."))

                              )
                   )

        .subcommand(SubCommand::with_name("entry")
                   .about("Manage calendar entries")
                   .version("0.1")
                   .subcommand(SubCommand::with_name("add")
                              .about("Add entry to an existing collection. Opens Editor to edit entry data")
                              .version("0.1")
                              .arg(Arg::with_name("entry-add-collectionname")
                                   .long("collection")
                                   .short("c")
                                   .takes_value(true)
                                   .required(true)
                                   .multiple(false)
                                   .value_name("collectionname")
                                   .help("Add an entry to an existing collection."))
                              )

                   .subcommand(SubCommand::with_name("remove")
                              .about("Remove entry")
                              .version("0.1")
                              .arg(Arg::with_name("entry-remove-uid")
                                   .index(1)
                                   .takes_value(true)
                                   .required(true)
                                   .multiple(false)
                                   .value_name("UID")
                                   .help("The UID of the entry to remove"))
                              )

                   .subcommand(SubCommand::with_name("show")
                              .about("Show entry")
                              .version("0.1")
                              .arg(Arg::with_name("entry-show-uid")
                                   .index(1)
                                   .takes_value(true)
                                   .required(true)
                                   .multiple(false)
                                   .value_name("UID")
                                   .help("The UID of the entry to show"))
                              )

                   )

        .subcommand(SubCommand::with_name("find")
                   .about("Find entries in from past as well")
                   .version("0.1")
                   .arg(Arg::with_name("find-past")
                        .long("past")
                        .short("p")
                        .takes_value(false)
                        .required(false)
                        .multiple(false)
                        .help("Find not only from today and future, but also from past"))

                   .arg(Arg::with_name("collection-find-grep")
                        .long("grep")
                        .short("g")
                        .takes_value(true)
                        .required(false)
                        .multiple(false)
                        .help("Find by grepping through calendar entry meta data"))
                   )

        .subcommand(SubCommand::with_name("show")
                   .about("Show entries from all collections in calendar (default command)")
                   .version("0.1")
                   .arg(Arg::with_name("show-past")
                        .long("past")
                        .short("p")
                        .takes_value(false)
                        .required(false)
                        .multiple(false)
                        .help("Show not only from today and future, but also from past"))
                   )

        .subcommand(SubCommand::with_name("list")
                   .about("List entries from all collections in calendar")
                   .version("0.1")
                   .arg(Arg::with_name("list-past")
                        .long("past")
                        .short("p")
                        .takes_value(false)
                        .required(false)
                        .multiple(false)
                        .help("Show not only from today and future, but also from past"))
                   )
}

