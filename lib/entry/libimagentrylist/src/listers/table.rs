//
// imag - the personal information management suite for the commandline
// Copyright (C) 2015, 2016 Matthias Beyer <mail@beyermatthias.de> and contributors
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

use std::io::stdout;

use lister::Lister;
use error::Result;
use error::ResultExt;

use libimagstore::store::FileLockEntry;

use prettytable::Table;
use prettytable::cell::Cell;
use prettytable::row::Row;

pub struct TableLister<F: Fn(&FileLockEntry) -> Vec<String>> {
    line_generator: F,
    header: Option<Vec<String>>,

    with_idx: bool,
    print_empty: bool,
}

impl<F: Fn(&FileLockEntry) -> Vec<String>> TableLister<F> {

    pub fn new(gen: F) -> TableLister<F> {
        TableLister {
            line_generator: gen,
            header: None,
            with_idx: true,
            print_empty: false,
        }
    }

    pub fn with_header(mut self, hdr: Vec<String>) -> TableLister<F> {
        self.header = Some(hdr);
        self
    }

    pub fn with_idx(mut self, b: bool) -> TableLister<F> {
        self.with_idx = b;
        self
    }

    pub fn print_empty(mut self, b: bool) -> TableLister<F> {
        self.print_empty = b;
        self
    }

}

impl<F: Fn(&FileLockEntry) -> Vec<String>> Lister for TableLister<F> {

    fn list<'b, I: Iterator<Item = FileLockEntry<'b>>>(&self, entries: I) -> Result<()> {
        use error::ListErrorKind as LEK;
        use error::ListError as LE;

        let mut table = Table::new();
        let mut header_len : Option<usize> = None;
        match self.header {
            Some(ref s) => {
                debug!("We have a header... preparing");
                let mut cells : Vec<Cell> = s.iter().map(|s| Cell::new(s)).collect();
                if self.with_idx {
                    cells.insert(0, Cell::new("#"));
                }
                table.set_titles(Row::new(cells));
                header_len = Some(s.len());
            },
            None => {
                debug!("No header for table found... continuing without");
            },
        }

        let mut entries_added = 0;

        entries.enumerate().fold(Ok(table), |table, (i, entry)| {
            table.and_then(|mut table| {
                let mut v = (self.line_generator)(&entry);
                {
                    let v_len = v.len();
                    if header_len.is_none() {
                        header_len = Some(v_len);
                    }
                    if header_len.map(|l| v_len > l).unwrap_or(false) {
                        return Err(LE::from_kind(LEK::FormatError));
                    }
                    while header_len.map(|l| v.len() != l).unwrap_or(false) {
                        v.push(String::from(""));
                    }
                }

                if self.with_idx {
                    v.insert(0, format!("{}", i));
                }

                table.add_row(v.iter().map(|s| Cell::new(s)).collect());
                entries_added += 1;
                Ok(table)
            })
        })
        .and_then(|tbl| {
            if entries_added != 0 && !self.print_empty {
                let mut io = stdout();
                tbl.print(&mut io).chain_err(|| LEK::IOError)
            } else {
                Ok(())
            }
        })
    }

}
