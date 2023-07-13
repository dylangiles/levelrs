use std::{
    ffi::c_char,
    ptr::{null, null_mut},
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
    pub fn leveldb_open(
        options: *const leveldb_options_t,
        name: *const c_char,
        errptr: *mut *mut c_char,
    ) -> *mut leveldb_t;

    pub fn leveldb_options_create() -> *mut leveldb_options_t;
}

pub fn test() {
    let options = unsafe { leveldb_options_create() };
    println!("{:#?}", options);
}
#[cfg(test)]
mod tests {
    #[test]
    fn test1() {
        super::test();
    }
}
