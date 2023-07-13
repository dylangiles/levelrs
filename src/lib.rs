use std::{
    ffi::{c_char, CString},
    ptr::{self, null, null_mut},
};

use ffi_opaque::opaque;

#[allow(non_camel_case_types)]
opaque! {
    /// Opaque handle representing an opened database. The handle is thread-safe.
    pub struct leveldb_t;
    pub struct leveldb_cache_t;
    pub struct leveldb_comparator_t;
    pub struct leveldb_env_t;
    pub struct leveldb_filelock_t;
    pub struct leveldb_filterpolicy_t;
    /// Opaque handle representing an ongoing iteration process through the database.
    /// This handle is not thread safe.
    pub struct leveldb_iterator_t;
    pub struct leveldb_logger_t;
    /// Opaque handle representing options used when opening a database. May be discarded after use,
    /// using `leveldb_free`.
    pub struct leveldb_options_t;
    pub struct leveldb_randomfile_t;
    /// Opaque handle representing options used during a read operation. May be discarded after use,
    /// using `leveldb_free`.
    pub struct leveldb_readoptions_t;
    pub struct leveldb_seqfile_t;
    pub struct leveldb_snapshot_t;
    pub struct leveldb_writablefile_t;
    pub struct leveldb_writebatch_t;
    /// Opaque handle representing options used during a read operation. May be discarded after use,
    /// using `leveldb_free`.
    pub struct leveldb_writeoptions_t;
}

extern "C" {
    // DB operations

    /// Open the database at path `name` with the configurations set in `options`.
    /// In case of success, the return value represents an open database.
    ///
    /// If this operation fails,
    /// - `leveldb_t` is a nullpointer
    /// - `errptr` contains more information about the error reason
    pub(crate) fn leveldb_open(
        options: *const leveldb_options_t,
        name: *const c_char,
        errptr: *mut *mut c_char,
    ) -> *mut leveldb_t;

    pub(crate) fn leveldb_options_create() -> *mut leveldb_options_t;
    pub(crate) fn leveldb_options_set_create_if_missing(options: *mut leveldb_options_t, value: u8);
    pub(crate) fn leveldb_close(db: *mut leveldb_t);
}

pub fn test() {
    let options = unsafe { leveldb_options_create() };
    unsafe { leveldb_options_set_create_if_missing(options, 1) };
    // let error = String::with_capacity(100);
    let mut errptr = ptr::null_mut();
    let db_name = CString::new("db/test").unwrap();
    let db = unsafe { leveldb_open(options, db_name.as_ptr(), &mut errptr) };
    unsafe { leveldb_close(db) };
    println!("{:#?}", options);
}

#[cfg(test)]
mod tests {
    #[test]
    fn test1() {
        super::test();
    }
}
