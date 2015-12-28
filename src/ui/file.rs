use std::cell::RefCell;
use std::iter::Iterator;
use std::rc::Rc;
use std::ops::Deref;

use storage::file::File;

pub trait FilePrinter {

    fn new(verbose: bool, debug: bool) -> Self;

    /*
     * Print a single file
     */
    fn print_file(&self, Rc<RefCell<File>>);

    /*
     * Print a list of files
     */
    fn print_files<I: Iterator<Item = Rc<RefCell<File>>>>(&self, files: I) {
        for file in files {
            self.print_file(file);
        }
    }

    fn print_file_custom<F>(&self, file: Rc<RefCell<File>>, f: &F)
        where F: Fn(Rc<RefCell<File>>) -> String
    {
        info!("{}", f(file));
    }

    fn print_files_custom<F, I>(&self, files: I, f: &F)
        where I: Iterator<Item = Rc<RefCell<File>>>,
              F: Fn(Rc<RefCell<File>>) -> String
    {
        for file in files {
            self.print_file_custom(file, f);
        }
    }

}

struct DebugPrinter {
    debug: bool,
}

impl FilePrinter for DebugPrinter {

    fn new(_: bool, debug: bool) -> DebugPrinter {
        DebugPrinter {
            debug: debug,
        }
    }

    fn print_file(&self, f: Rc<RefCell<File>>) {
        if self.debug {
            debug!("[DebugPrinter] ->\n{:?}", f);
        }
    }

    fn print_file_custom<F>(&self, file: Rc<RefCell<File>>, f: &F)
        where F: Fn(Rc<RefCell<File>>) -> String
    {
        if self.debug {
            debug!("[DebugPrinter] ->\n{:?}", f(file));
        }
    }

}

struct SimplePrinter {
    verbose:    bool,
    debug:      bool,
}

impl FilePrinter for SimplePrinter {

    fn new(verbose: bool, debug: bool) -> SimplePrinter {
        SimplePrinter {
            debug:      debug,
            verbose:    verbose,
        }
    }

    fn print_file(&self, f: Rc<RefCell<File>>) {
        if self.debug {
            debug!("{:?}", f);
        } else if self.verbose {
            info!("{}", &*f.deref().borrow());
        } else {
            info!("[File]: {}", f.deref().borrow().id());
        }
    }

    fn print_file_custom<F>(&self, file: Rc<RefCell<File>>, f: &F)
        where F: Fn(Rc<RefCell<File>>) -> String
    {
        let s = f(file);
        if self.debug {
            debug!("{:?}", s);
        } else if self.verbose {
            info!("{}", s);
        } else {
            info!("[File]: {}", s);
        }
    }

}

pub struct TablePrinter {
    verbose:    bool,
    debug:      bool,
    sp:         SimplePrinter,
}

impl FilePrinter for TablePrinter {

    fn new(verbose: bool, debug: bool) -> TablePrinter {
        TablePrinter {
            debug:      debug,
            verbose:    verbose,
            sp:         SimplePrinter::new(verbose, debug),
        }
    }

    fn print_file(&self, f: Rc<RefCell<File>>) {
        self.sp.print_file(f);
    }

    fn print_files<I: Iterator<Item = Rc<RefCell<File>>>>(&self, files: I) {
        use prettytable::Table;
        use prettytable::row::Row;
        use prettytable::cell::Cell;

        let titles = row!["File#", "Owner", "ID"];

        let mut tab = Table::new();
        tab.set_titles(titles);

        let mut i = 0;
        for file in files {
            debug!("Printing file: {:?}", file);
            i += 1;
            let cell_i  = Cell::new(&format!("{}", i)[..]);
            let cell_o  = Cell::new(&format!("{}", file.deref().borrow().owner_name())[..]);

            let id : String = file.deref().borrow().id().clone().into();
            let cell_id = Cell::new(&id[..]);
            let row = Row::new(vec![cell_i, cell_o, cell_id]);
            tab.add_row(row);
        }

        if i != 0 {
            debug!("Printing {} table entries", i);
            tab.printstd();
        } else {
            debug!("Not printing table because there are zero entries");
        }
    }

    fn print_files_custom<F, I>(&self, files: I, f: &F)
        where I: Iterator<Item = Rc<RefCell<File>>>,
              F: Fn(Rc<RefCell<File>>) -> String
    {
        use prettytable::Table;
        use prettytable::row::Row;
        use prettytable::cell::Cell;

        let titles = row!["File#", "Owner", "ID", "..."];

        let mut tab = Table::new();
        tab.set_titles(titles);

        let mut i = 0;
        for file in files {
            debug!("Printing file: {:?}", file);
            i += 1;
            let cell_i  = Cell::new(&format!("{}", i)[..]);
            let cell_o  = Cell::new(&format!("{}", file.deref().borrow().owner_name())[..]);

            let id : String = file.deref().borrow().id().clone().into();
            let cell_id = Cell::new(&id[..]);

            let cell_extra = Cell::new(&f(file)[..]);

            let row = Row::new(vec![cell_i, cell_o, cell_id, cell_extra]);
            tab.add_row(row);
        }

        if i != 0 {
            debug!("Printing {} table entries", i);
            tab.printstd();
        } else {
            debug!("Not printing table because there are zero entries");
        }
    }

}
