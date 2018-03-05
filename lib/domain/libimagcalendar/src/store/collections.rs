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

use std::path::Path;

use libimagstore::store::FileLockEntry;
use libimagstore::store::Store;
use libimagentryref::generators::sha1::Sha1;
use libimagentryref::refstore::RefStore;

use error::Result;
use error::CalendarError as CE;

/// A interface to the store which offers CRUD functionality for calendar collections
pub struct CalendarCollectionStore<'a>(&'a Store);

make_unique_ref_path_generator! (
    pub CalendarCollectionPathHasher
    over Sha1
    => with error CE
    => with collection name "calendar/collection"
    => |path| {
        Sha1::hash_path(path).map_err(CE::from)
    }
);

impl<'a> CalendarCollectionStore<'a> {

    fn get_calendar_collection<H: AsRef<str>>(&self, hash: H) -> Result<Option<FileLockEntry<'a>>> {
        self.0.get_ref::<CalendarCollectionPathHasher, H>(hash).map_err(CE::from)
    }

    fn create_calendar_collection<P: AsRef<Path>>(&self, p: P) -> Result<FileLockEntry<'a>> {
        self.0.create_ref::<CalendarCollectionPathHasher, P>(p).map_err(CE::from)
    }

    fn retrieve_calendar_collection<P: AsRef<Path>>(&self, p: P) -> Result<FileLockEntry<'a>> {
        self.0.retrieve_ref::<CalendarCollectionPathHasher, P>(p).map_err(CE::from)
    }

    fn delete_calendar_collection_by_hash(&self, hash: String) -> Result<()> {
        unimplemented!()
    }

}
