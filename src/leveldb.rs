use ffi_opaque::opaque;
use std::ffi::{c_char, c_void};

opaque! {
    #[allow(non_camel_case_types)]
    pub struct leveldb_t;

    #[allow(non_camel_case_types)]
    pub struct leveldb_cache_t;

    #[allow(non_camel_case_types)]
    pub struct leveldb_comparator_t;

    #[allow(non_camel_case_types)]
    pub struct leveldb_env_t;

    #[allow(non_camel_case_types)]
    pub struct leveldb_filelock_t;

    #[allow(non_camel_case_types)]
    pub struct leveldb_filterpolicy_t;

    #[allow(non_camel_case_types)]
    pub struct leveldb_iterator_t;

    #[allow(non_camel_case_types)]
    pub struct leveldb_logger_t;

    #[allow(non_camel_case_types)]
    pub struct leveldb_options_t;

    #[allow(non_camel_case_types)]
    pub struct leveldb_randomfile_t;

    #[allow(non_camel_case_types)]
    pub struct leveldb_readoptions_t;

    #[allow(non_camel_case_types)]
    pub struct leveldb_seqfile_t;

    #[allow(non_camel_case_types)]
    pub struct leveldb_snapshot_t;

    #[allow(non_camel_case_types)]
    pub struct leveldb_writablefile_t;

    #[allow(non_camel_case_types)]
    pub struct leveldb_writebatch_t;

    #[allow(non_camel_case_types)]
    pub struct leveldb_writeoptions_t;
}

#[allow(dead_code)]
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
    pub(crate) fn leveldb_writeoptions_create() -> *const leveldb_writeoptions_t;
    pub(crate) fn leveldb_writeoptions_set_sync(woptions: *const leveldb_writeoptions_t, value: u8);
    pub(crate) fn leveldb_put(
        db: *mut leveldb_t,
        options: *const leveldb_writeoptions_t,
        key: *const u8,
        keylen: usize,
        val: *const u8,
        vallen: usize,
        errptr: *mut *mut c_char,
    );

    // LEVELDB_EXPORT char* leveldb_get(leveldb_t* db,
    //     const leveldb_readoptions_t* options,
    //     const char* key, size_t keylen, size_t* vallen,
    //     char** errptr);

    // LEVELDB_EXPORT leveldb_readoptions_t* leveldb_readoptions_create(void);
    pub(crate) fn leveldb_readoptions_create() -> *const leveldb_readoptions_t;

    pub(crate) fn leveldb_readoptions_set_verify_checksums(
        roptions: *const leveldb_readoptions_t,
        value: u8,
    );
    pub(crate) fn leveldb_readoptions_set_fill_cache(
        roptions: *const leveldb_readoptions_t,
        value: u8,
    );
    pub(crate) fn leveldb_readoptions_set_snapshot(
        roptions: *const leveldb_readoptions_t,
        value: u8,
    );

    pub(crate) fn leveldb_get(
        db: *const leveldb_t,
        options: *const leveldb_readoptions_t,
        key: *const u8,
        keylen: usize,
        vallen: &mut usize,
        errptr: *mut *mut c_char,
    ) -> *const u8;

    pub(crate) fn leveldb_free(ptr: *mut c_void);
}

pub(crate) type SetReadOptionFunction = unsafe extern "C" fn(*const leveldb_readoptions_t, u8);
