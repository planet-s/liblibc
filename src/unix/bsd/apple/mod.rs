//! Apple (ios/darwin)-specific definitions
//!
//! This covers *-apple-* triples currently

pub type clock_t = c_ulong;
pub type time_t = c_long;
pub type suseconds_t = i32;
pub type dev_t = i32;
pub type ino_t = u64;
pub type mode_t = u16;
pub type nlink_t = u16;
pub type blksize_t = i32;
pub type rlim_t = u64;
pub type mach_timebase_info_data_t = mach_timebase_info;
pub type pthread_key_t = c_ulong;
pub type sigset_t = u32;
pub type fsblkcnt_t = ::c_uint;
pub type fsfilcnt_t = ::c_uint;
pub type speed_t = ::c_ulong;
pub type tcflag_t = ::c_ulong;
pub type nl_item = ::c_int;
pub type id_t = ::c_uint;
pub type sem_t = ::c_int;
pub type idtype_t = ::c_uint;

pub enum timezone {}

s! {
    pub struct aiocb {
        pub aio_fildes: ::c_int,
        pub aio_offset: ::off_t,
        pub aio_buf: *mut ::c_void,
        pub aio_nbytes: ::size_t,
        pub aio_reqprio: ::c_int,
        pub aio_sigevent: sigevent,
        pub aio_lio_opcode: ::c_int
    }

    pub struct utmpx {
        pub ut_user: [::c_char; _UTX_USERSIZE],
        pub ut_id: [::c_char; _UTX_IDSIZE],
        pub ut_line: [::c_char; _UTX_LINESIZE],
        pub ut_pid: ::pid_t,
        pub ut_type: ::c_short,
        pub ut_tv: ::timeval,
        pub ut_host: [::c_char; _UTX_HOSTSIZE],
        ut_pad: [::uint32_t; 16],
    }

    pub struct glob_t {
        pub gl_pathc:  ::size_t,
        __unused1: ::c_int,
        pub gl_offs:   ::size_t,
        __unused2: ::c_int,
        pub gl_pathv:  *mut *mut ::c_char,

        __unused3: *mut ::c_void,

        __unused4: *mut ::c_void,
        __unused5: *mut ::c_void,
        __unused6: *mut ::c_void,
        __unused7: *mut ::c_void,
        __unused8: *mut ::c_void,
    }

    pub struct sockaddr_storage {
        pub ss_len: u8,
        pub ss_family: ::sa_family_t,
        __ss_pad1: [u8; 6],
        __ss_align: i64,
        __ss_pad2: [u8; 112],
    }

    pub struct addrinfo {
        pub ai_flags: ::c_int,
        pub ai_family: ::c_int,
        pub ai_socktype: ::c_int,
        pub ai_protocol: ::c_int,
        pub ai_addrlen: ::socklen_t,
        pub ai_canonname: *mut ::c_char,
        pub ai_addr: *mut ::sockaddr,
        pub ai_next: *mut addrinfo,
    }

    pub struct mach_timebase_info {
        pub numer: u32,
        pub denom: u32,
    }

    pub struct stat {
        pub st_dev: dev_t,
        pub st_mode: mode_t,
        pub st_nlink: nlink_t,
        pub st_ino: ino_t,
        pub st_uid: ::uid_t,
        pub st_gid: ::gid_t,
        pub st_rdev: dev_t,
        pub st_atime: time_t,
        pub st_atime_nsec: c_long,
        pub st_mtime: time_t,
        pub st_mtime_nsec: c_long,
        pub st_ctime: time_t,
        pub st_ctime_nsec: c_long,
        pub st_birthtime: time_t,
        pub st_birthtime_nsec: c_long,
        pub st_size: ::off_t,
        pub st_blocks: ::blkcnt_t,
        pub st_blksize: blksize_t,
        pub st_flags: ::uint32_t,
        pub st_gen: ::uint32_t,
        pub st_lspare: ::int32_t,
        pub st_qspare: [::int64_t; 2],
    }

    pub struct dirent {
        pub d_ino: u64,
        pub d_seekoff: u64,
        pub d_reclen: u16,
        pub d_namlen: u16,
        pub d_type: u8,
        pub d_name: [::c_char; 1024],
    }

    pub struct pthread_mutex_t {
        __sig: ::c_long,
        __opaque: [u8; __PTHREAD_MUTEX_SIZE__],
    }

    pub struct pthread_mutexattr_t {
        __sig: ::c_long,
        __opaque: [u8; 8],
    }

    pub struct pthread_cond_t {
        __sig: ::c_long,
        __opaque: [u8; __PTHREAD_COND_SIZE__],
    }

    pub struct pthread_condattr_t {
        __sig: ::c_long,
        __opaque: [u8; __PTHREAD_CONDATTR_SIZE__],
    }

    pub struct pthread_rwlock_t {
        __sig: ::c_long,
        __opaque: [u8; __PTHREAD_RWLOCK_SIZE__],
    }

    pub struct pthread_rwlockattr_t {
        __sig: ::c_long,
        __opaque: [u8; __PTHREAD_RWLOCKATTR_SIZE__],
    }

    pub struct siginfo_t {
        pub si_signo: ::c_int,
        pub si_errno: ::c_int,
        pub si_code: ::c_int,
        pub si_pid: ::pid_t,
        pub si_uid: ::uid_t,
        pub si_status: ::c_int,
        pub si_addr: *mut ::c_void,
        _pad: [usize; 9],
    }

    pub struct sigaction {
        pub sa_sigaction: ::sighandler_t,
        pub sa_mask: sigset_t,
        pub sa_flags: ::c_int,
    }

    pub struct stack_t {
        pub ss_sp: *mut ::c_void,
        pub ss_size: ::size_t,
        pub ss_flags: ::c_int,
    }

    pub struct fstore_t {
        pub fst_flags: ::c_uint,
        pub fst_posmode: ::c_int,
        pub fst_offset: ::off_t,
        pub fst_length: ::off_t,
        pub fst_bytesalloc: ::off_t,
    }

    pub struct radvisory {
        pub ra_offset: ::off_t,
        pub ra_count: ::c_int,
    }

    pub struct statvfs {
        pub f_bsize: ::c_ulong,
        pub f_frsize: ::c_ulong,
        pub f_blocks: ::fsblkcnt_t,
        pub f_bfree: ::fsblkcnt_t,
        pub f_bavail: ::fsblkcnt_t,
        pub f_files: ::fsfilcnt_t,
        pub f_ffree: ::fsfilcnt_t,
        pub f_favail: ::fsfilcnt_t,
        pub f_fsid: ::c_ulong,
        pub f_flag: ::c_ulong,
        pub f_namemax: ::c_ulong,
    }

    pub struct Dl_info {
        pub dli_fname: *const ::c_char,
        pub dli_fbase: *mut ::c_void,
        pub dli_sname: *const ::c_char,
        pub dli_saddr: *mut ::c_void,
    }

    pub struct sockaddr_in {
        pub sin_len: u8,
        pub sin_family: ::sa_family_t,
        pub sin_port: ::in_port_t,
        pub sin_addr: ::in_addr,
        pub sin_zero: [::c_char; 8],
    }

    pub struct statfs {
        pub f_bsize: ::uint32_t,
        pub f_iosize: ::int32_t,
        pub f_blocks: ::uint64_t,
        pub f_bfree: ::uint64_t,
        pub f_bavail: ::uint64_t,
        pub f_files: ::uint64_t,
        pub f_ffree: ::uint64_t,
        pub f_fsid: ::fsid_t,
        pub f_owner: ::uid_t,
        pub f_type: ::uint32_t,
        pub f_flags: ::uint32_t,
        pub f_fssubtype: ::uint32_t,
        pub f_fstypename: [::c_char; 16],
        pub f_mntonname: [::c_char; 1024],
        pub f_mntfromname: [::c_char; 1024],
        pub f_reserved: [::uint32_t; 8],
    }

    // FIXME: this should have align 4 but it's got align 8 on 64-bit
    pub struct kevent {
        pub ident: ::uintptr_t,
        pub filter: ::int16_t,
        pub flags: ::uint16_t,
        pub fflags: ::uint32_t,
        pub data: ::intptr_t,
        pub udata: *mut ::c_void,
    }

    pub struct kevent64_s {
        pub ident: ::uint64_t,
        pub filter: ::int16_t,
        pub flags: ::uint16_t,
        pub fflags: ::uint32_t,
        pub data: ::int64_t,
        pub udata: ::uint64_t,
        pub ext: [::uint64_t; 2],
    }

    pub struct dqblk {
        pub dqb_bhardlimit: ::uint64_t,
        pub dqb_bsoftlimit: ::uint64_t,
        pub dqb_curbytes: ::uint64_t,
        pub dqb_ihardlimit: ::uint32_t,
        pub dqb_isoftlimit: ::uint32_t,
        pub dqb_curinodes: ::uint32_t,
        pub dqb_btime: ::uint32_t,
        pub dqb_itime: ::uint32_t,
        pub dqb_id: ::uint32_t,
        pub dqb_spare: [::uint32_t; 4],
    }

    pub struct termios {
        pub c_iflag: ::tcflag_t,
        pub c_oflag: ::tcflag_t,
        pub c_cflag: ::tcflag_t,
        pub c_lflag: ::tcflag_t,
        pub c_cc: [::cc_t; ::NCCS],
        pub c_ispeed: ::speed_t,
        pub c_ospeed: ::speed_t,
    }

    pub struct flock {
        pub l_start: ::off_t,
        pub l_len: ::off_t,
        pub l_pid: ::pid_t,
        pub l_type: ::c_short,
        pub l_whence: ::c_short,
    }

    pub struct sf_hdtr {
        pub headers: *mut ::iovec,
        pub hdr_cnt: ::c_int,
        pub trailers: *mut ::iovec,
        pub trl_cnt: ::c_int,
    }

    pub struct lconv {
        pub decimal_point: *mut ::c_char,
        pub thousands_sep: *mut ::c_char,
        pub grouping: *mut ::c_char,
        pub int_curr_symbol: *mut ::c_char,
        pub currency_symbol: *mut ::c_char,
        pub mon_decimal_point: *mut ::c_char,
        pub mon_thousands_sep: *mut ::c_char,
        pub mon_grouping: *mut ::c_char,
        pub positive_sign: *mut ::c_char,
        pub negative_sign: *mut ::c_char,
        pub int_frac_digits: ::c_char,
        pub frac_digits: ::c_char,
        pub p_cs_precedes: ::c_char,
        pub p_sep_by_space: ::c_char,
        pub n_cs_precedes: ::c_char,
        pub n_sep_by_space: ::c_char,
        pub p_sign_posn: ::c_char,
        pub n_sign_posn: ::c_char,
        pub int_p_cs_precedes: ::c_char,
        pub int_n_cs_precedes: ::c_char,
        pub int_p_sep_by_space: ::c_char,
        pub int_n_sep_by_space: ::c_char,
        pub int_p_sign_posn: ::c_char,
        pub int_n_sign_posn: ::c_char,
    }

    pub struct sigevent {
        pub sigev_notify: ::c_int,
        pub sigev_signo: ::c_int,
        pub sigev_value: ::sigval,
        __unused1: *mut ::c_void,       //actually a function pointer
        pub sigev_notify_attributes: *mut ::pthread_attr_t
    }
}

pub const _UTX_USERSIZE: usize = 256;
pub const _UTX_LINESIZE: usize = 32;
pub const _UTX_IDSIZE: usize = 4;
pub const _UTX_HOSTSIZE: usize = 256;

pub const EMPTY: ::c_short = 0;
pub const RUN_LVL: ::c_short = 1;
pub const BOOT_TIME: ::c_short = 2;
pub const OLD_TIME: ::c_short = 3;
pub const NEW_TIME: ::c_short = 4;
pub const INIT_PROCESS: ::c_short = 5;
pub const LOGIN_PROCESS: ::c_short = 6;
pub const USER_PROCESS: ::c_short = 7;
pub const DEAD_PROCESS: ::c_short = 8;
pub const ACCOUNTING: ::c_short = 9;
pub const SIGNATURE: ::c_short = 10;
pub const SHUTDOWN_TIME: ::c_short = 11;

pub const LC_COLLATE_MASK: ::c_int = (1 << 0);
pub const LC_CTYPE_MASK: ::c_int = (1 << 1);
pub const LC_MESSAGES_MASK: ::c_int = (1 << 2);
pub const LC_MONETARY_MASK: ::c_int = (1 << 3);
pub const LC_NUMERIC_MASK: ::c_int = (1 << 4);
pub const LC_TIME_MASK: ::c_int = (1 << 5);
pub const LC_ALL_MASK: ::c_int = LC_COLLATE_MASK
                               | LC_CTYPE_MASK
                               | LC_MESSAGES_MASK
                               | LC_MONETARY_MASK
                               | LC_NUMERIC_MASK
                               | LC_TIME_MASK;

pub const CODESET: ::nl_item = 0;
pub const D_T_FMT: ::nl_item = 1;
pub const D_FMT: ::nl_item = 2;
pub const T_FMT: ::nl_item = 3;
pub const T_FMT_AMPM: ::nl_item = 4;
pub const AM_STR: ::nl_item = 5;
pub const PM_STR: ::nl_item = 6;

pub const DAY_1: ::nl_item = 7;
pub const DAY_2: ::nl_item = 8;
pub const DAY_3: ::nl_item = 9;
pub const DAY_4: ::nl_item = 10;
pub const DAY_5: ::nl_item = 11;
pub const DAY_6: ::nl_item = 12;
pub const DAY_7: ::nl_item = 13;

pub const ABDAY_1: ::nl_item = 14;
pub const ABDAY_2: ::nl_item = 15;
pub const ABDAY_3: ::nl_item = 16;
pub const ABDAY_4: ::nl_item = 17;
pub const ABDAY_5: ::nl_item = 18;
pub const ABDAY_6: ::nl_item = 19;
pub const ABDAY_7: ::nl_item = 20;

pub const MON_1: ::nl_item = 21;
pub const MON_2: ::nl_item = 22;
pub const MON_3: ::nl_item = 23;
pub const MON_4: ::nl_item = 24;
pub const MON_5: ::nl_item = 25;
pub const MON_6: ::nl_item = 26;
pub const MON_7: ::nl_item = 27;
pub const MON_8: ::nl_item = 28;
pub const MON_9: ::nl_item = 29;
pub const MON_10: ::nl_item = 30;
pub const MON_11: ::nl_item = 31;
pub const MON_12: ::nl_item = 32;

pub const ABMON_1: ::nl_item = 33;
pub const ABMON_2: ::nl_item = 34;
pub const ABMON_3: ::nl_item = 35;
pub const ABMON_4: ::nl_item = 36;
pub const ABMON_5: ::nl_item = 37;
pub const ABMON_6: ::nl_item = 38;
pub const ABMON_7: ::nl_item = 39;
pub const ABMON_8: ::nl_item = 40;
pub const ABMON_9: ::nl_item = 41;
pub const ABMON_10: ::nl_item = 42;
pub const ABMON_11: ::nl_item = 43;
pub const ABMON_12: ::nl_item = 44;

pub const ERA: ::nl_item = 45;
pub const ERA_D_FMT: ::nl_item = 46;
pub const ERA_D_T_FMT: ::nl_item = 47;
pub const ERA_T_FMT: ::nl_item = 48;
pub const ALT_DIGITS: ::nl_item = 49;

pub const RADIXCHAR: ::nl_item = 50;
pub const THOUSEP: ::nl_item = 51;

pub const YESEXPR: ::nl_item = 52;
pub const NOEXPR: ::nl_item = 53;

pub const YESSTR: ::nl_item = 54;
pub const NOSTR: ::nl_item = 55;

pub const CRNCYSTR: ::nl_item = 56;

pub const D_MD_ORDER: ::nl_item = 57;

pub const EXIT_FAILURE: ::c_int = 1;
pub const EXIT_SUCCESS: ::c_int = 0;
pub const RAND_MAX: ::c_int = 2147483647;
pub const EOF: ::c_int = -1;
pub const SEEK_SET: ::c_int = 0;
pub const SEEK_CUR: ::c_int = 1;
pub const SEEK_END: ::c_int = 2;
pub const _IOFBF: ::c_int = 0;
pub const _IONBF: ::c_int = 2;
pub const _IOLBF: ::c_int = 1;
pub const BUFSIZ: ::c_uint = 1024;
pub const FOPEN_MAX: ::c_uint = 20;
pub const FILENAME_MAX: ::c_uint = 1024;
pub const L_tmpnam: ::c_uint = 1024;
pub const TMP_MAX: ::c_uint = 308915776;
pub const _PC_LINK_MAX: ::c_int = 1;
pub const _PC_MAX_CANON: ::c_int = 2;
pub const _PC_MAX_INPUT: ::c_int = 3;
pub const _PC_NAME_MAX: ::c_int = 4;
pub const _PC_PATH_MAX: ::c_int = 5;
pub const _PC_PIPE_BUF: ::c_int = 6;
pub const _PC_CHOWN_RESTRICTED: ::c_int = 7;
pub const _PC_NO_TRUNC: ::c_int = 8;
pub const _PC_VDISABLE: ::c_int = 9;
pub const O_DSYNC: ::c_int = 0x400000;
pub const O_NOCTTY: ::c_int = 0x20000;
pub const O_CLOEXEC: ::c_int = 0x1000000;
pub const O_DIRECTORY: ::c_int = 0x100000;
pub const S_IFIFO: mode_t = 4096;
pub const S_IFCHR: mode_t = 8192;
pub const S_IFBLK: mode_t = 24576;
pub const S_IFDIR: mode_t = 16384;
pub const S_IFREG: mode_t = 32768;
pub const S_IFLNK: mode_t = 40960;
pub const S_IFSOCK: mode_t = 49152;
pub const S_IFMT: mode_t = 61440;
pub const S_IEXEC: mode_t = 64;
pub const S_IWRITE: mode_t = 128;
pub const S_IREAD: mode_t = 256;
pub const S_IRWXU: mode_t = 448;
pub const S_IXUSR: mode_t = 64;
pub const S_IWUSR: mode_t = 128;
pub const S_IRUSR: mode_t = 256;
pub const S_IRWXG: mode_t = 56;
pub const S_IXGRP: mode_t = 8;
pub const S_IWGRP: mode_t = 16;
pub const S_IRGRP: mode_t = 32;
pub const S_IRWXO: mode_t = 7;
pub const S_IXOTH: mode_t = 1;
pub const S_IWOTH: mode_t = 2;
pub const S_IROTH: mode_t = 4;
pub const F_OK: ::c_int = 0;
pub const R_OK: ::c_int = 4;
pub const W_OK: ::c_int = 2;
pub const X_OK: ::c_int = 1;
pub const STDIN_FILENO: ::c_int = 0;
pub const STDOUT_FILENO: ::c_int = 1;
pub const STDERR_FILENO: ::c_int = 2;
pub const F_LOCK: ::c_int = 1;
pub const F_TEST: ::c_int = 3;
pub const F_TLOCK: ::c_int = 2;
pub const F_ULOCK: ::c_int = 0;
pub const F_GETLK: ::c_int = 7;
pub const F_SETLK: ::c_int = 8;
pub const F_SETLKW: ::c_int = 9;
pub const SIGHUP: ::c_int = 1;
pub const SIGINT: ::c_int = 2;
pub const SIGQUIT: ::c_int = 3;
pub const SIGILL: ::c_int = 4;
pub const SIGABRT: ::c_int = 6;
pub const SIGEMT: ::c_int = 7;
pub const SIGFPE: ::c_int = 8;
pub const SIGKILL: ::c_int = 9;
pub const SIGSEGV: ::c_int = 11;
pub const SIGPIPE: ::c_int = 13;
pub const SIGALRM: ::c_int = 14;
pub const SIGTERM: ::c_int = 15;

pub const PROT_NONE: ::c_int = 0;
pub const PROT_READ: ::c_int = 1;
pub const PROT_WRITE: ::c_int = 2;
pub const PROT_EXEC: ::c_int = 4;

pub const MAP_FILE: ::c_int = 0x0000;
pub const MAP_SHARED: ::c_int = 0x0001;
pub const MAP_PRIVATE: ::c_int = 0x0002;
pub const MAP_FIXED: ::c_int = 0x0010;
pub const MAP_ANON: ::c_int = 0x1000;

pub const MAP_FAILED: *mut ::c_void = !0 as *mut ::c_void;

pub const MCL_CURRENT: ::c_int = 0x0001;
pub const MCL_FUTURE: ::c_int = 0x0002;

pub const MS_ASYNC: ::c_int = 0x0001;
pub const MS_INVALIDATE: ::c_int = 0x0002;
pub const MS_SYNC: ::c_int = 0x0010;

pub const MS_KILLPAGES: ::c_int = 0x0004;
pub const MS_DEACTIVATE: ::c_int = 0x0008;

pub const EPERM: ::c_int = 1;
pub const ENOENT: ::c_int = 2;
pub const ESRCH: ::c_int = 3;
pub const EINTR: ::c_int = 4;
pub const EIO: ::c_int = 5;
pub const ENXIO: ::c_int = 6;
pub const E2BIG: ::c_int = 7;
pub const ENOEXEC: ::c_int = 8;
pub const EBADF: ::c_int = 9;
pub const ECHILD: ::c_int = 10;
pub const EDEADLK: ::c_int = 11;
pub const ENOMEM: ::c_int = 12;
pub const EACCES: ::c_int = 13;
pub const EFAULT: ::c_int = 14;
pub const ENOTBLK: ::c_int = 15;
pub const EBUSY: ::c_int = 16;
pub const EEXIST: ::c_int = 17;
pub const EXDEV: ::c_int = 18;
pub const ENODEV: ::c_int = 19;
pub const ENOTDIR: ::c_int = 20;
pub const EISDIR: ::c_int = 21;
pub const EINVAL: ::c_int = 22;
pub const ENFILE: ::c_int = 23;
pub const EMFILE: ::c_int = 24;
pub const ENOTTY: ::c_int = 25;
pub const ETXTBSY: ::c_int = 26;
pub const EFBIG: ::c_int = 27;
pub const ENOSPC: ::c_int = 28;
pub const ESPIPE: ::c_int = 29;
pub const EROFS: ::c_int = 30;
pub const EMLINK: ::c_int = 31;
pub const EPIPE: ::c_int = 32;
pub const EDOM: ::c_int = 33;
pub const ERANGE: ::c_int = 34;
pub const EAGAIN: ::c_int = 35;
pub const EWOULDBLOCK: ::c_int = EAGAIN;
pub const EINPROGRESS: ::c_int = 36;
pub const EALREADY: ::c_int = 37;
pub const ENOTSOCK: ::c_int = 38;
pub const EDESTADDRREQ: ::c_int = 39;
pub const EMSGSIZE: ::c_int = 40;
pub const EPROTOTYPE: ::c_int = 41;
pub const ENOPROTOOPT: ::c_int = 42;
pub const EPROTONOSUPPORT: ::c_int = 43;
pub const ESOCKTNOSUPPORT: ::c_int = 44;
pub const ENOTSUP: ::c_int = 45;
pub const EPFNOSUPPORT: ::c_int = 46;
pub const EAFNOSUPPORT: ::c_int = 47;
pub const EADDRINUSE: ::c_int = 48;
pub const EADDRNOTAVAIL: ::c_int = 49;
pub const ENETDOWN: ::c_int = 50;
pub const ENETUNREACH: ::c_int = 51;
pub const ENETRESET: ::c_int = 52;
pub const ECONNABORTED: ::c_int = 53;
pub const ECONNRESET: ::c_int = 54;
pub const ENOBUFS: ::c_int = 55;
pub const EISCONN: ::c_int = 56;
pub const ENOTCONN: ::c_int = 57;
pub const ESHUTDOWN: ::c_int = 58;
pub const ETOOMANYREFS: ::c_int = 59;
pub const ETIMEDOUT: ::c_int = 60;
pub const ECONNREFUSED: ::c_int = 61;
pub const ELOOP: ::c_int = 62;
pub const ENAMETOOLONG: ::c_int = 63;
pub const EHOSTDOWN: ::c_int = 64;
pub const EHOSTUNREACH: ::c_int = 65;
pub const ENOTEMPTY: ::c_int = 66;
pub const EPROCLIM: ::c_int = 67;
pub const EUSERS: ::c_int = 68;
pub const EDQUOT: ::c_int = 69;
pub const ESTALE: ::c_int = 70;
pub const EREMOTE: ::c_int = 71;
pub const EBADRPC: ::c_int = 72;
pub const ERPCMISMATCH: ::c_int = 73;
pub const EPROGUNAVAIL: ::c_int = 74;
pub const EPROGMISMATCH: ::c_int = 75;
pub const EPROCUNAVAIL: ::c_int = 76;
pub const ENOLCK: ::c_int = 77;
pub const ENOSYS: ::c_int = 78;
pub const EFTYPE: ::c_int = 79;
pub const EAUTH: ::c_int = 80;
pub const ENEEDAUTH: ::c_int = 81;
pub const EPWROFF: ::c_int = 82;
pub const EDEVERR: ::c_int = 83;
pub const EOVERFLOW: ::c_int = 84;
pub const EBADEXEC: ::c_int = 85;
pub const EBADARCH: ::c_int = 86;
pub const ESHLIBVERS: ::c_int = 87;
pub const EBADMACHO: ::c_int = 88;
pub const ECANCELED: ::c_int = 89;
pub const EIDRM: ::c_int = 90;
pub const ENOMSG: ::c_int = 91;
pub const EILSEQ: ::c_int = 92;
pub const ENOATTR: ::c_int = 93;
pub const EBADMSG: ::c_int = 94;
pub const EMULTIHOP: ::c_int = 95;
pub const ENODATA: ::c_int = 96;
pub const ENOLINK: ::c_int = 97;
pub const ENOSR: ::c_int = 98;
pub const ENOSTR: ::c_int = 99;
pub const EPROTO: ::c_int = 100;
pub const ETIME: ::c_int = 101;
pub const EOPNOTSUPP: ::c_int = 102;
pub const ENOPOLICY: ::c_int = 103;
pub const ENOTRECOVERABLE: ::c_int = 104;
pub const EOWNERDEAD: ::c_int = 105;
pub const EQFULL: ::c_int = 106;
pub const ELAST: ::c_int = 106;

pub const EAI_SYSTEM: ::c_int = 11;

pub const F_DUPFD: ::c_int = 0;
pub const F_DUPFD_CLOEXEC: ::c_int = 67;
pub const F_GETFD: ::c_int = 1;
pub const F_SETFD: ::c_int = 2;
pub const F_GETFL: ::c_int = 3;
pub const F_SETFL: ::c_int = 4;
pub const F_PREALLOCATE: ::c_int = 42;
pub const F_RDADVISE: ::c_int = 44;
pub const F_RDAHEAD: ::c_int = 45;
pub const F_NOCACHE: ::c_int = 48;
pub const F_GETPATH: ::c_int = 50;
pub const F_FULLFSYNC: ::c_int = 51;
pub const F_FREEZE_FS: ::c_int = 53;
pub const F_THAW_FS: ::c_int = 54;
pub const F_GLOBAL_NOCACHE: ::c_int = 55;
pub const F_NODIRECT: ::c_int = 62;

pub const F_ALLOCATECONTIG: ::c_uint = 0x02;
pub const F_ALLOCATEALL: ::c_uint = 0x04;

pub const F_PEOFPOSMODE: ::c_int = 3;
pub const F_VOLPOSMODE: ::c_int = 4;

pub const AT_FDCWD: ::c_int = -2;
pub const AT_EACCESS: ::c_int = 0x0010;
pub const AT_SYMLINK_NOFOLLOW: ::c_int = 0x0020;
pub const AT_SYMLINK_FOLLOW: ::c_int = 0x0040;
pub const AT_REMOVEDIR: ::c_int = 0x0080;

pub const TIOCMODG: ::c_ulong = 0x40047403;
pub const TIOCMODS: ::c_ulong = 0x80047404;
pub const TIOCM_LE: ::c_int = 0x1;
pub const TIOCM_DTR: ::c_int = 0x2;
pub const TIOCM_RTS: ::c_int = 0x4;
pub const TIOCM_ST: ::c_int = 0x8;
pub const TIOCM_SR: ::c_int = 0x10;
pub const TIOCM_CTS: ::c_int = 0x20;
pub const TIOCM_CAR: ::c_int = 0x40;
pub const TIOCM_CD: ::c_int = 0x40;
pub const TIOCM_RNG: ::c_int = 0x80;
pub const TIOCM_RI: ::c_int = 0x80;
pub const TIOCM_DSR: ::c_int = 0x100;
pub const TIOCEXCL: ::c_int = 0x2000740d;
pub const TIOCNXCL: ::c_int = 0x2000740e;
pub const TIOCFLUSH: ::c_ulong = 0x80047410;
pub const TIOCGETD: ::c_ulong = 0x4004741a;
pub const TIOCSETD: ::c_ulong = 0x8004741b;
pub const TIOCIXON: ::c_uint = 0x20007481;
pub const TIOCIXOFF: ::c_uint = 0x20007480;
pub const TIOCSBRK: ::c_uint = 0x2000747b;
pub const TIOCCBRK: ::c_uint = 0x2000747a;
pub const TIOCSDTR: ::c_uint = 0x20007479;
pub const TIOCCDTR: ::c_uint = 0x20007478;
pub const TIOCGPGRP: ::c_ulong = 0x40047477;
pub const TIOCSPGRP: ::c_ulong = 0x80047476;
pub const TIOCOUTQ: ::c_ulong = 0x40047473;
pub const TIOCSTI: ::c_ulong = 0x80017472;
pub const TIOCNOTTY: ::c_uint = 0x20007471;
pub const TIOCPKT: ::c_ulong = 0x80047470;
pub const TIOCPKT_DATA: ::c_int = 0x0;
pub const TIOCPKT_FLUSHREAD: ::c_int = 0x1;
pub const TIOCPKT_FLUSHWRITE: ::c_int = 0x2;
pub const TIOCPKT_STOP: ::c_int = 0x4;
pub const TIOCPKT_START: ::c_int = 0x8;
pub const TIOCPKT_NOSTOP: ::c_int = 0x10;
pub const TIOCPKT_DOSTOP: ::c_int = 0x20;
pub const TIOCPKT_IOCTL: ::c_int = 0x40;
pub const TIOCSTOP: ::c_uint = 0x2000746f;
pub const TIOCSTART: ::c_uint = 0x2000746e;
pub const TIOCMSET: ::c_ulong = 0x8004746d;
pub const TIOCMBIS: ::c_ulong = 0x8004746c;
pub const TIOCMBIC: ::c_ulong = 0x8004746b;
pub const TIOCMGET: ::c_ulong = 0x4004746a;
pub const TIOCREMOTE: ::c_ulong = 0x80047469;
pub const TIOCGWINSZ: ::c_ulong = 0x40087468;
pub const TIOCSWINSZ: ::c_ulong = 0x80087467;
pub const TIOCUCNTL: ::c_ulong = 0x80047466;
pub const TIOCSTAT: ::c_uint = 0x20007465;
pub const TIOCSCONS: ::c_uint = 0x20007463;
pub const TIOCCONS: ::c_ulong = 0x80047462;
pub const TIOCSCTTY: ::c_uint = 0x20007461;
pub const TIOCEXT: ::c_ulong = 0x80047460;
pub const TIOCSIG: ::c_uint = 0x2000745f;
pub const TIOCDRAIN: ::c_uint = 0x2000745e;
pub const TIOCMSDTRWAIT: ::c_ulong = 0x8004745b;
pub const TIOCMGDTRWAIT: ::c_ulong = 0x4004745a;
pub const TIOCSDRAINWAIT: ::c_ulong = 0x80047457;
pub const TIOCGDRAINWAIT: ::c_ulong = 0x40047456;
pub const TIOCDSIMICROCODE: ::c_uint = 0x20007455;
pub const TIOCPTYGRANT: ::c_uint = 0x20007454;
pub const TIOCPTYGNAME: ::c_uint = 0x40807453;
pub const TIOCPTYUNLK: ::c_uint = 0x20007452;

pub const B0: speed_t = 0;
pub const B50: speed_t = 50;
pub const B75: speed_t = 75;
pub const B110: speed_t = 110;
pub const B134: speed_t = 134;
pub const B150: speed_t = 150;
pub const B200: speed_t = 200;
pub const B300: speed_t = 300;
pub const B600: speed_t = 600;
pub const B1200: speed_t = 1200;
pub const B1800: speed_t = 1800;
pub const B2400: speed_t = 2400;
pub const B4800: speed_t = 4800;
pub const B9600: speed_t = 9600;
pub const B19200: speed_t = 19200;
pub const B38400: speed_t = 38400;
pub const B7200: speed_t = 7200;
pub const B14400: speed_t = 14400;
pub const B28800: speed_t = 28800;
pub const B57600: speed_t = 57600;
pub const B76800: speed_t = 76800;
pub const B115200: speed_t = 115200;
pub const B230400: speed_t = 230400;
pub const EXTA: speed_t = 19200;
pub const EXTB: speed_t = 38400;

pub const SIGTRAP: ::c_int = 5;

pub const GLOB_APPEND  : ::c_int = 0x0001;
pub const GLOB_DOOFFS  : ::c_int = 0x0002;
pub const GLOB_ERR     : ::c_int = 0x0004;
pub const GLOB_MARK    : ::c_int = 0x0008;
pub const GLOB_NOCHECK : ::c_int = 0x0010;
pub const GLOB_NOSORT  : ::c_int = 0x0020;
pub const GLOB_NOESCAPE: ::c_int = 0x2000;

pub const GLOB_NOSPACE : ::c_int = -1;
pub const GLOB_ABORTED : ::c_int = -2;
pub const GLOB_NOMATCH : ::c_int = -3;

pub const POSIX_MADV_NORMAL: ::c_int = 0;
pub const POSIX_MADV_RANDOM: ::c_int = 1;
pub const POSIX_MADV_SEQUENTIAL: ::c_int = 2;
pub const POSIX_MADV_WILLNEED: ::c_int = 3;
pub const POSIX_MADV_DONTNEED: ::c_int = 4;

pub const _SC_IOV_MAX: ::c_int = 56;
pub const _SC_GETGR_R_SIZE_MAX: ::c_int = 70;
pub const _SC_GETPW_R_SIZE_MAX: ::c_int = 71;
pub const _SC_LOGIN_NAME_MAX: ::c_int = 73;
pub const _SC_MQ_PRIO_MAX: ::c_int = 75;
pub const _SC_THREAD_ATTR_STACKADDR: ::c_int = 82;
pub const _SC_THREAD_ATTR_STACKSIZE: ::c_int = 83;
pub const _SC_THREAD_DESTRUCTOR_ITERATIONS: ::c_int = 85;
pub const _SC_THREAD_KEYS_MAX: ::c_int = 86;
pub const _SC_THREAD_PRIO_INHERIT: ::c_int = 87;
pub const _SC_THREAD_PRIO_PROTECT: ::c_int = 88;
pub const _SC_THREAD_PRIORITY_SCHEDULING: ::c_int = 89;
pub const _SC_THREAD_PROCESS_SHARED: ::c_int = 90;
pub const _SC_THREAD_SAFE_FUNCTIONS: ::c_int = 91;
pub const _SC_THREAD_STACK_MIN: ::c_int = 93;
pub const _SC_THREAD_THREADS_MAX: ::c_int = 94;
pub const _SC_THREADS: ::c_int = 96;
pub const _SC_TTY_NAME_MAX: ::c_int = 101;
pub const _SC_ATEXIT_MAX: ::c_int = 107;
pub const _SC_XOPEN_CRYPT: ::c_int = 108;
pub const _SC_XOPEN_ENH_I18N: ::c_int = 109;
pub const _SC_XOPEN_LEGACY: ::c_int = 110;
pub const _SC_XOPEN_REALTIME: ::c_int = 111;
pub const _SC_XOPEN_REALTIME_THREADS: ::c_int = 112;
pub const _SC_XOPEN_SHM: ::c_int = 113;
pub const _SC_XOPEN_UNIX: ::c_int = 115;
pub const _SC_XOPEN_VERSION: ::c_int = 116;
pub const _SC_XOPEN_XCU_VERSION: ::c_int = 121;

pub const PTHREAD_PROCESS_PRIVATE: ::c_int = 2;
pub const PTHREAD_PROCESS_SHARED: ::c_int = 1;
pub const PTHREAD_CREATE_JOINABLE: ::c_int = 1;
pub const PTHREAD_CREATE_DETACHED: ::c_int = 2;
pub const PTHREAD_STACK_MIN: ::size_t = 8192;

pub const RLIMIT_CPU: ::c_int = 0;
pub const RLIMIT_FSIZE: ::c_int = 1;
pub const RLIMIT_DATA: ::c_int = 2;
pub const RLIMIT_STACK: ::c_int = 3;
pub const RLIMIT_CORE: ::c_int = 4;
pub const RLIMIT_AS: ::c_int = 5;
pub const RLIMIT_RSS: ::c_int = RLIMIT_AS;
pub const RLIMIT_MEMLOCK: ::c_int = 6;
pub const RLIMIT_NPROC: ::c_int = 7;
pub const RLIMIT_NOFILE: ::c_int = 8;
pub const RLIM_NLIMITS: ::c_int = 9;
pub const _RLIMIT_POSIX_FLAG: ::c_int = 0x1000;

pub const RLIM_INFINITY: rlim_t = 0x7fff_ffff_ffff_ffff;

pub const RUSAGE_SELF: ::c_int = 0;
pub const RUSAGE_CHILDREN: ::c_int = -1;

pub const MADV_NORMAL: ::c_int = 0;
pub const MADV_RANDOM: ::c_int = 1;
pub const MADV_SEQUENTIAL: ::c_int = 2;
pub const MADV_WILLNEED: ::c_int = 3;
pub const MADV_DONTNEED: ::c_int = 4;
pub const MADV_FREE: ::c_int = 5;
pub const MADV_ZERO_WIRED_PAGES: ::c_int = 6;
pub const MADV_FREE_REUSABLE: ::c_int = 7;
pub const MADV_FREE_REUSE: ::c_int = 8;
pub const MADV_CAN_REUSE: ::c_int = 9;

pub const MINCORE_INCORE: ::c_int =  0x1;
pub const MINCORE_REFERENCED: ::c_int = 0x2;
pub const MINCORE_MODIFIED: ::c_int = 0x4;
pub const MINCORE_REFERENCED_OTHER: ::c_int = 0x8;
pub const MINCORE_MODIFIED_OTHER: ::c_int = 0x10;

pub const AF_UNSPEC: ::c_int = 0;
pub const AF_LOCAL: ::c_int = 1;
pub const AF_UNIX: ::c_int = AF_LOCAL;
pub const AF_INET: ::c_int = 2;
pub const AF_IMPLINK: ::c_int = 3;
pub const AF_PUP: ::c_int = 4;
pub const AF_CHAOS: ::c_int = 5;
pub const AF_NS: ::c_int = 6;
pub const AF_ISO: ::c_int = 7;
pub const AF_OSI: ::c_int = AF_ISO;
pub const AF_ECMA: ::c_int = 8;
pub const AF_DATAKIT: ::c_int = 9;
pub const AF_CCITT: ::c_int = 10;
pub const AF_SNA: ::c_int = 11;
pub const AF_DECnet: ::c_int = 12;
pub const AF_DLI: ::c_int = 13;
pub const AF_LAT: ::c_int = 14;
pub const AF_HYLINK: ::c_int = 15;
pub const AF_APPLETALK: ::c_int = 16;
pub const AF_ROUTE: ::c_int = 17;
pub const AF_LINK: ::c_int = 18;
pub const pseudo_AF_XTP: ::c_int = 19;
pub const AF_COIP: ::c_int = 20;
pub const AF_CNT: ::c_int = 21;
pub const pseudo_AF_RTIP: ::c_int = 22;
pub const AF_IPX: ::c_int = 23;
pub const AF_SIP: ::c_int = 24;
pub const pseudo_AF_PIP: ::c_int = 25;
pub const AF_ISDN: ::c_int = 28;
pub const AF_E164: ::c_int = AF_ISDN;
pub const pseudo_AF_KEY: ::c_int = 29;
pub const AF_INET6: ::c_int = 30;
pub const AF_NATM: ::c_int = 31;
pub const AF_SYSTEM: ::c_int = 32;
pub const AF_NETBIOS: ::c_int = 33;
pub const AF_PPP: ::c_int = 34;
pub const pseudo_AF_HDRCMPLT: ::c_int = 35;
#[doc(hidden)]
pub const AF_MAX: ::c_int = 40;

pub const PF_UNSPEC: ::c_int = AF_UNSPEC;
pub const PF_LOCAL: ::c_int = AF_LOCAL;
pub const PF_UNIX: ::c_int =  PF_LOCAL;
pub const PF_INET: ::c_int =  AF_INET;
pub const PF_IMPLINK: ::c_int = AF_IMPLINK;
pub const PF_PUP: ::c_int =  AF_PUP;
pub const PF_CHAOS: ::c_int = AF_CHAOS;
pub const PF_NS: ::c_int =  AF_NS;
pub const PF_ISO: ::c_int =  AF_ISO;
pub const PF_OSI: ::c_int =  AF_ISO;
pub const PF_ECMA: ::c_int =  AF_ECMA;
pub const PF_DATAKIT: ::c_int = AF_DATAKIT;
pub const PF_CCITT: ::c_int = AF_CCITT;
pub const PF_SNA: ::c_int =  AF_SNA;
pub const PF_DECnet: ::c_int = AF_DECnet;
pub const PF_DLI: ::c_int =  AF_DLI;
pub const PF_LAT: ::c_int =  AF_LAT;
pub const PF_HYLINK: ::c_int = AF_HYLINK;
pub const PF_APPLETALK: ::c_int = AF_APPLETALK;
pub const PF_ROUTE: ::c_int = AF_ROUTE;
pub const PF_LINK: ::c_int =  AF_LINK;
pub const PF_XTP: ::c_int =  pseudo_AF_XTP;
pub const PF_COIP: ::c_int =  AF_COIP;
pub const PF_CNT: ::c_int =  AF_CNT;
pub const PF_SIP: ::c_int =  AF_SIP;
pub const PF_IPX: ::c_int =  AF_IPX;
pub const PF_RTIP: ::c_int =  pseudo_AF_RTIP;
pub const PF_PIP: ::c_int =  pseudo_AF_PIP;
pub const PF_ISDN: ::c_int =  AF_ISDN;
pub const PF_KEY: ::c_int =  pseudo_AF_KEY;
pub const PF_INET6: ::c_int = AF_INET6;
pub const PF_NATM: ::c_int =  AF_NATM;
pub const PF_SYSTEM: ::c_int = AF_SYSTEM;
pub const PF_NETBIOS: ::c_int = AF_NETBIOS;
pub const PF_PPP: ::c_int =  AF_PPP;
#[doc(hidden)]
pub const PF_MAX: ::c_int =  AF_MAX;

#[doc(hidden)]
pub const NET_MAXID: ::c_int = AF_MAX;

pub const NET_RT_DUMP: ::c_int = 1;
pub const NET_RT_FLAGS: ::c_int = 2;
pub const NET_RT_IFLIST: ::c_int = 3;
#[doc(hidden)]
pub const NET_RT_MAXID: ::c_int = 10;

pub const SOMAXCONN: ::c_int = 128;

pub const SOCK_MAXADDRLEN: ::c_int = 255;

pub const SOCK_STREAM: ::c_int = 1;
pub const SOCK_DGRAM: ::c_int = 2;
pub const SOCK_RAW: ::c_int = 3;
pub const SOCK_RDM: ::c_int = 4;
pub const SOCK_SEQPACKET: ::c_int = 5;
pub const IPPROTO_ICMP: ::c_int = 1;
pub const IPPROTO_ICMPV6: ::c_int = 58;
pub const IPPROTO_TCP: ::c_int = 6;
pub const IPPROTO_IP: ::c_int = 0;
pub const IPPROTO_IPV6: ::c_int = 41;
pub const IP_MULTICAST_TTL: ::c_int = 10;
pub const IP_MULTICAST_LOOP: ::c_int = 11;
pub const IP_TTL: ::c_int = 4;
pub const IP_HDRINCL: ::c_int = 2;
pub const IP_ADD_MEMBERSHIP: ::c_int = 12;
pub const IP_DROP_MEMBERSHIP: ::c_int = 13;
pub const IPV6_JOIN_GROUP: ::c_int = 12;
pub const IPV6_LEAVE_GROUP: ::c_int = 13;

pub const TCP_NODELAY: ::c_int = 0x01;
pub const TCP_KEEPALIVE: ::c_int = 0x10;
pub const SOL_SOCKET: ::c_int = 0xffff;

pub const SO_DEBUG: ::c_int = 0x01;
pub const SO_ACCEPTCONN: ::c_int = 0x0002;
pub const SO_REUSEADDR: ::c_int = 0x0004;
pub const SO_KEEPALIVE: ::c_int = 0x0008;
pub const SO_DONTROUTE: ::c_int = 0x0010;
pub const SO_BROADCAST: ::c_int = 0x0020;
pub const SO_USELOOPBACK: ::c_int = 0x0040;
pub const SO_LINGER: ::c_int = 0x0080;
pub const SO_OOBINLINE: ::c_int = 0x0100;
pub const SO_REUSEPORT: ::c_int = 0x0200;
pub const SO_TIMESTAMP: ::c_int = 0x0400;
pub const SO_DONTTRUNC: ::c_int = 0x2000;
pub const SO_WANTMORE: ::c_int = 0x4000;
pub const SO_WANTOOBFLAG: ::c_int = 0x8000;
pub const SO_SNDBUF: ::c_int = 0x1001;
pub const SO_RCVBUF: ::c_int = 0x1002;
pub const SO_SNDLOWAT: ::c_int = 0x1003;
pub const SO_RCVLOWAT: ::c_int = 0x1004;
pub const SO_SNDTIMEO: ::c_int = 0x1005;
pub const SO_RCVTIMEO: ::c_int = 0x1006;
pub const SO_ERROR: ::c_int = 0x1007;
pub const SO_TYPE: ::c_int = 0x1008;
pub const SO_NREAD: ::c_int = 0x1020;
pub const SO_NKE: ::c_int = 0x1021;
pub const SO_NOSIGPIPE: ::c_int = 0x1022;
pub const SO_NOADDRERR: ::c_int = 0x1023;
pub const SO_NWRITE: ::c_int = 0x1024;

pub const MSG_OOB: ::c_int =  0x1;
pub const MSG_PEEK: ::c_int = 0x2;
pub const MSG_DONTROUTE: ::c_int = 0x4;
pub const MSG_EOR: ::c_int =  0x8;
pub const MSG_TRUNC: ::c_int = 0x10;
pub const MSG_CTRUNC: ::c_int = 0x20;
pub const MSG_WAITALL: ::c_int = 0x40;
pub const MSG_DONTWAIT: ::c_int = 0x80;
pub const MSG_EOF: ::c_int =  0x100;
pub const MSG_FLUSH: ::c_int = 0x400;
pub const MSG_HOLD: ::c_int = 0x800;
pub const MSG_SEND: ::c_int = 0x1000;
pub const MSG_HAVEMORE: ::c_int = 0x2000;
pub const MSG_RCVMORE: ::c_int = 0x4000;
// pub const MSG_COMPAT: ::c_int = 0x8000;

pub const SCM_RIGHTS: ::c_int = 0x01;
pub const SCM_TIMESTAMP: ::c_int = 0x02;
pub const SCM_CREDS: ::c_int = 0x03;

pub const IFF_LOOPBACK: ::c_int = 0x8;

pub const SHUT_RD: ::c_int = 0;
pub const SHUT_WR: ::c_int = 1;
pub const SHUT_RDWR: ::c_int = 2;

pub const LOCK_SH: ::c_int = 1;
pub const LOCK_EX: ::c_int = 2;
pub const LOCK_NB: ::c_int = 4;
pub const LOCK_UN: ::c_int = 8;

pub const MAP_COPY: ::c_int = 0x0002;
pub const MAP_RENAME: ::c_int = 0x0020;
pub const MAP_NORESERVE: ::c_int = 0x0040;
pub const MAP_NOEXTEND: ::c_int = 0x0100;
pub const MAP_HASSEMAPHORE: ::c_int = 0x0200;
pub const MAP_NOCACHE: ::c_int = 0x0400;
pub const MAP_JIT: ::c_int = 0x0800;

pub const IPPROTO_RAW: ::c_int = 255;

pub const _SC_ARG_MAX: ::c_int = 1;
pub const _SC_CHILD_MAX: ::c_int = 2;
pub const _SC_CLK_TCK: ::c_int = 3;
pub const _SC_NGROUPS_MAX: ::c_int = 4;
pub const _SC_OPEN_MAX: ::c_int = 5;
pub const _SC_JOB_CONTROL: ::c_int = 6;
pub const _SC_SAVED_IDS: ::c_int = 7;
pub const _SC_VERSION: ::c_int = 8;
pub const _SC_BC_BASE_MAX: ::c_int = 9;
pub const _SC_BC_DIM_MAX: ::c_int = 10;
pub const _SC_BC_SCALE_MAX: ::c_int = 11;
pub const _SC_BC_STRING_MAX: ::c_int = 12;
pub const _SC_COLL_WEIGHTS_MAX: ::c_int = 13;
pub const _SC_EXPR_NEST_MAX: ::c_int = 14;
pub const _SC_LINE_MAX: ::c_int = 15;
pub const _SC_RE_DUP_MAX: ::c_int = 16;
pub const _SC_2_VERSION: ::c_int = 17;
pub const _SC_2_C_BIND: ::c_int = 18;
pub const _SC_2_C_DEV: ::c_int = 19;
pub const _SC_2_CHAR_TERM: ::c_int = 20;
pub const _SC_2_FORT_DEV: ::c_int = 21;
pub const _SC_2_FORT_RUN: ::c_int = 22;
pub const _SC_2_LOCALEDEF: ::c_int = 23;
pub const _SC_2_SW_DEV: ::c_int = 24;
pub const _SC_2_UPE: ::c_int = 25;
pub const _SC_STREAM_MAX: ::c_int = 26;
pub const _SC_TZNAME_MAX: ::c_int = 27;
pub const _SC_ASYNCHRONOUS_IO: ::c_int = 28;
pub const _SC_PAGESIZE: ::c_int = 29;
pub const _SC_MEMLOCK: ::c_int = 30;
pub const _SC_MEMLOCK_RANGE: ::c_int = 31;
pub const _SC_MEMORY_PROTECTION: ::c_int = 32;
pub const _SC_MESSAGE_PASSING: ::c_int = 33;
pub const _SC_PRIORITIZED_IO: ::c_int = 34;
pub const _SC_PRIORITY_SCHEDULING: ::c_int = 35;
pub const _SC_REALTIME_SIGNALS: ::c_int = 36;
pub const _SC_SEMAPHORES: ::c_int = 37;
pub const _SC_FSYNC: ::c_int = 38;
pub const _SC_SHARED_MEMORY_OBJECTS: ::c_int = 39;
pub const _SC_SYNCHRONIZED_IO: ::c_int = 40;
pub const _SC_TIMERS: ::c_int = 41;
pub const _SC_AIO_LISTIO_MAX: ::c_int = 42;
pub const _SC_AIO_MAX: ::c_int = 43;
pub const _SC_AIO_PRIO_DELTA_MAX: ::c_int = 44;
pub const _SC_DELAYTIMER_MAX: ::c_int = 45;
pub const _SC_MQ_OPEN_MAX: ::c_int = 46;
pub const _SC_MAPPED_FILES: ::c_int = 47;
pub const _SC_RTSIG_MAX: ::c_int = 48;
pub const _SC_SEM_NSEMS_MAX: ::c_int = 49;
pub const _SC_SEM_VALUE_MAX: ::c_int = 50;
pub const _SC_SIGQUEUE_MAX: ::c_int = 51;
pub const _SC_TIMER_MAX: ::c_int = 52;
pub const _SC_NPROCESSORS_CONF: ::c_int = 57;
pub const _SC_NPROCESSORS_ONLN: ::c_int = 58;
pub const _SC_2_PBS: ::c_int = 59;
pub const _SC_2_PBS_ACCOUNTING: ::c_int = 60;
pub const _SC_2_PBS_CHECKPOINT: ::c_int = 61;
pub const _SC_2_PBS_LOCATE: ::c_int = 62;
pub const _SC_2_PBS_MESSAGE: ::c_int = 63;
pub const _SC_2_PBS_TRACK: ::c_int = 64;
pub const _SC_ADVISORY_INFO: ::c_int = 65;
pub const _SC_BARRIERS: ::c_int = 66;
pub const _SC_CLOCK_SELECTION: ::c_int = 67;
pub const _SC_CPUTIME: ::c_int = 68;
pub const _SC_FILE_LOCKING: ::c_int = 69;
pub const _SC_HOST_NAME_MAX: ::c_int = 72;
pub const _SC_MONOTONIC_CLOCK: ::c_int = 74;
pub const _SC_READER_WRITER_LOCKS: ::c_int = 76;
pub const _SC_REGEXP: ::c_int = 77;
pub const _SC_SHELL: ::c_int = 78;
pub const _SC_SPAWN: ::c_int = 79;
pub const _SC_SPIN_LOCKS: ::c_int = 80;
pub const _SC_SPORADIC_SERVER: ::c_int = 81;
pub const _SC_THREAD_CPUTIME: ::c_int = 84;
pub const _SC_THREAD_SPORADIC_SERVER: ::c_int = 92;
pub const _SC_TIMEOUTS: ::c_int = 95;
pub const _SC_TRACE: ::c_int = 97;
pub const _SC_TRACE_EVENT_FILTER: ::c_int = 98;
pub const _SC_TRACE_INHERIT: ::c_int = 99;
pub const _SC_TRACE_LOG: ::c_int = 100;
pub const _SC_TYPED_MEMORY_OBJECTS: ::c_int = 102;
pub const _SC_V6_ILP32_OFF32: ::c_int = 103;
pub const _SC_V6_ILP32_OFFBIG: ::c_int = 104;
pub const _SC_V6_LP64_OFF64: ::c_int = 105;
pub const _SC_V6_LPBIG_OFFBIG: ::c_int = 106;
pub const _SC_IPV6: ::c_int = 118;
pub const _SC_RAW_SOCKETS: ::c_int = 119;
pub const _SC_SYMLOOP_MAX: ::c_int = 120;
pub const _SC_PAGE_SIZE: ::c_int = _SC_PAGESIZE;
pub const _SC_XOPEN_STREAMS: ::c_int = 114;
pub const _SC_XBS5_ILP32_OFF32: ::c_int = 122;
pub const _SC_XBS5_ILP32_OFFBIG: ::c_int = 123;
pub const _SC_XBS5_LP64_OFF64: ::c_int = 124;
pub const _SC_XBS5_LPBIG_OFFBIG: ::c_int = 125;
pub const _SC_SS_REPL_MAX: ::c_int = 126;
pub const _SC_TRACE_EVENT_NAME_MAX: ::c_int = 127;
pub const _SC_TRACE_NAME_MAX: ::c_int = 128;
pub const _SC_TRACE_SYS_MAX: ::c_int = 129;
pub const _SC_TRACE_USER_EVENT_MAX: ::c_int = 130;
pub const _SC_PASS_MAX: ::c_int = 131;

pub const PTHREAD_MUTEX_NORMAL: ::c_int = 0;
pub const PTHREAD_MUTEX_ERRORCHECK: ::c_int = 1;
pub const PTHREAD_MUTEX_RECURSIVE: ::c_int = 2;
pub const PTHREAD_MUTEX_DEFAULT: ::c_int = PTHREAD_MUTEX_NORMAL;
pub const _PTHREAD_MUTEX_SIG_init: ::c_long = 0x32AAABA7;
pub const _PTHREAD_COND_SIG_init: ::c_long = 0x3CB0B1BB;
pub const _PTHREAD_RWLOCK_SIG_init: ::c_long = 0x2DA8B3B4;
pub const PTHREAD_MUTEX_INITIALIZER: pthread_mutex_t = pthread_mutex_t {
    __sig: _PTHREAD_MUTEX_SIG_init,
    __opaque: [0; __PTHREAD_MUTEX_SIZE__],
};
pub const PTHREAD_COND_INITIALIZER: pthread_cond_t = pthread_cond_t {
    __sig: _PTHREAD_COND_SIG_init,
    __opaque: [0; __PTHREAD_COND_SIZE__],
};
pub const PTHREAD_RWLOCK_INITIALIZER: pthread_rwlock_t = pthread_rwlock_t {
    __sig: _PTHREAD_RWLOCK_SIG_init,
    __opaque: [0; __PTHREAD_RWLOCK_SIZE__],
};

pub const SIGSTKSZ: ::size_t = 131072;

pub const FD_SETSIZE: usize = 1024;

pub const ST_NOSUID: ::c_ulong = 2;

pub const EVFILT_READ: ::int16_t = -1;
pub const EVFILT_WRITE: ::int16_t = -2;
pub const EVFILT_AIO: ::int16_t = -3;
pub const EVFILT_VNODE: ::int16_t = -4;
pub const EVFILT_PROC: ::int16_t = -5;
pub const EVFILT_SIGNAL: ::int16_t = -6;
pub const EVFILT_TIMER: ::int16_t = -7;
pub const EVFILT_MACHPORT: ::int16_t = -8;
pub const EVFILT_FS: ::int16_t = -9;
pub const EVFILT_USER: ::int16_t = -10;
pub const EVFILT_VM: ::int16_t = -12;

pub const EV_ADD: ::uint16_t = 0x1;
pub const EV_DELETE: ::uint16_t = 0x2;
pub const EV_ENABLE: ::uint16_t = 0x4;
pub const EV_DISABLE: ::uint16_t = 0x8;
pub const EV_ONESHOT: ::uint16_t = 0x10;
pub const EV_CLEAR: ::uint16_t = 0x20;
pub const EV_RECEIPT: ::uint16_t = 0x40;
pub const EV_DISPATCH: ::uint16_t = 0x80;
pub const EV_FLAG0: ::uint16_t = 0x1000;
pub const EV_POLL: ::uint16_t = 0x1000;
pub const EV_FLAG1: ::uint16_t = 0x2000;
pub const EV_OOBAND: ::uint16_t = 0x2000;
pub const EV_ERROR: ::uint16_t = 0x4000;
pub const EV_EOF: ::uint16_t = 0x8000;
pub const EV_SYSFLAGS: ::uint16_t = 0xf000;

pub const NOTE_TRIGGER: ::uint32_t = 0x01000000;
pub const NOTE_FFNOP: ::uint32_t = 0x00000000;
pub const NOTE_FFAND: ::uint32_t = 0x40000000;
pub const NOTE_FFOR: ::uint32_t = 0x80000000;
pub const NOTE_FFCOPY: ::uint32_t = 0xc0000000;
pub const NOTE_FFCTRLMASK: ::uint32_t = 0xc0000000;
pub const NOTE_FFLAGSMASK: ::uint32_t = 0x00ffffff;
pub const NOTE_LOWAT: ::uint32_t = 0x00000001;
pub const NOTE_DELETE: ::uint32_t = 0x00000001;
pub const NOTE_WRITE: ::uint32_t = 0x00000002;
pub const NOTE_EXTEND: ::uint32_t = 0x00000004;
pub const NOTE_ATTRIB: ::uint32_t = 0x00000008;
pub const NOTE_LINK: ::uint32_t = 0x00000010;
pub const NOTE_RENAME: ::uint32_t = 0x00000020;
pub const NOTE_REVOKE: ::uint32_t = 0x00000040;
pub const NOTE_NONE: ::uint32_t = 0x00000080;
pub const NOTE_EXIT: ::uint32_t = 0x80000000;
pub const NOTE_FORK: ::uint32_t = 0x40000000;
pub const NOTE_EXEC: ::uint32_t = 0x20000000;
pub const NOTE_REAP: ::uint32_t = 0x10000000;
pub const NOTE_SIGNAL: ::uint32_t = 0x08000000;
pub const NOTE_EXITSTATUS: ::uint32_t = 0x04000000;
pub const NOTE_EXIT_DETAIL: ::uint32_t = 0x02000000;
pub const NOTE_PDATAMASK: ::uint32_t = 0x000fffff;
pub const NOTE_PCTRLMASK: ::uint32_t = 0xfff00000;
pub const NOTE_EXIT_REPARENTED: ::uint32_t = 0x00080000;
pub const NOTE_EXIT_DETAIL_MASK: ::uint32_t = 0x00070000;
pub const NOTE_EXIT_DECRYPTFAIL: ::uint32_t = 0x00010000;
pub const NOTE_EXIT_MEMORY: ::uint32_t = 0x00020000;
pub const NOTE_EXIT_CSERROR: ::uint32_t = 0x00040000;
pub const NOTE_VM_PRESSURE: ::uint32_t = 0x80000000;
pub const NOTE_VM_PRESSURE_TERMINATE: ::uint32_t = 0x40000000;
pub const NOTE_VM_PRESSURE_SUDDEN_TERMINATE: ::uint32_t = 0x20000000;
pub const NOTE_VM_ERROR: ::uint32_t = 0x10000000;
pub const NOTE_SECONDS: ::uint32_t = 0x00000001;
pub const NOTE_USECONDS: ::uint32_t = 0x00000002;
pub const NOTE_NSECONDS: ::uint32_t = 0x00000004;
pub const NOTE_ABSOLUTE: ::uint32_t = 0x00000008;
pub const NOTE_LEEWAY: ::uint32_t = 0x00000010;
pub const NOTE_CRITICAL: ::uint32_t = 0x00000020;
pub const NOTE_BACKGROUND: ::uint32_t = 0x00000040;
pub const NOTE_TRACK: ::uint32_t = 0x00000001;
pub const NOTE_TRACKERR: ::uint32_t = 0x00000002;
pub const NOTE_CHILD: ::uint32_t = 0x00000004;

pub const OCRNL: ::c_int = 0x00000010;
pub const ONOCR: ::c_int = 0x00000020;
pub const ONLRET: ::c_int = 0x00000040;
pub const OFILL: ::c_int = 0x00000080;
pub const NLDLY: ::c_int = 0x00000300;
pub const TABDLY: ::c_int = 0x00000c04;
pub const CRDLY: ::c_int = 0x00003000;
pub const FFDLY: ::c_int = 0x00004000;
pub const BSDLY: ::c_int = 0x00008000;
pub const VTDLY: ::c_int = 0x00010000;
pub const OFDEL: ::c_int = 0x00020000;

pub const NL0: ::c_int  = 0x00000000;
pub const NL1: ::c_int  = 0x00000100;
pub const TAB0: ::c_int = 0x00000000;
pub const TAB1: ::c_int = 0x00000400;
pub const TAB2: ::c_int = 0x00000800;
pub const CR0: ::c_int  = 0x00000000;
pub const CR1: ::c_int  = 0x00001000;
pub const CR2: ::c_int  = 0x00002000;
pub const CR3: ::c_int  = 0x00003000;
pub const FF0: ::c_int  = 0x00000000;
pub const FF1: ::c_int  = 0x00004000;
pub const BS0: ::c_int  = 0x00000000;
pub const BS1: ::c_int  = 0x00008000;
pub const TAB3: ::c_int = 0x00000004;
pub const VT0: ::c_int  = 0x00000000;
pub const VT1: ::c_int  = 0x00010000;
pub const IUTF8: ::tcflag_t = 0x00004000;
pub const CRTSCTS: ::tcflag_t = 0x00030000;

pub const NI_MAXHOST: ::socklen_t = 1025;

pub const Q_GETQUOTA: ::c_int = 0x300;
pub const Q_SETQUOTA: ::c_int = 0x400;

pub const RTLD_LOCAL: ::c_int = 0x4;
pub const RTLD_FIRST: ::c_int = 0x100;
pub const RTLD_NODELETE: ::c_int = 0x80;
pub const RTLD_NOLOAD: ::c_int = 0x10;
pub const RTLD_GLOBAL: ::c_int = 0x8;

pub const _WSTOPPED: ::c_int = 0o177;

pub const LOG_NETINFO: ::c_int = 12 << 3;
pub const LOG_REMOTEAUTH: ::c_int = 13 << 3;
pub const LOG_INSTALL: ::c_int = 14 << 3;
pub const LOG_RAS: ::c_int = 15 << 3;
pub const LOG_LAUNCHD: ::c_int = 24 << 3;
pub const LOG_NFACILITIES: ::c_int = 25;

pub const CTLTYPE: ::c_int = 0xf;
pub const CTLTYPE_NODE: ::c_int = 1;
pub const CTLTYPE_INT: ::c_int = 2;
pub const CTLTYPE_STRING: ::c_int = 3;
pub const CTLTYPE_QUAD: ::c_int = 4;
pub const CTLTYPE_OPAQUE: ::c_int = 5;
pub const CTLTYPE_STRUCT: ::c_int = CTLTYPE_OPAQUE;
pub const CTLFLAG_RD: ::c_int = 0x80000000;
pub const CTLFLAG_WR: ::c_int = 0x40000000;
pub const CTLFLAG_RW: ::c_int = CTLFLAG_RD | CTLFLAG_WR;
pub const CTLFLAG_NOLOCK: ::c_int = 0x20000000;
pub const CTLFLAG_ANYBODY: ::c_int = 0x10000000;
pub const CTLFLAG_SECURE: ::c_int = 0x08000000;
pub const CTLFLAG_MASKED: ::c_int = 0x04000000;
pub const CTLFLAG_NOAUTO: ::c_int = 0x02000000;
pub const CTLFLAG_KERN: ::c_int = 0x01000000;
pub const CTLFLAG_LOCKED: ::c_int = 0x00800000;
pub const CTLFLAG_OID2: ::c_int = 0x00400000;
pub const CTL_UNSPEC: ::c_int = 0;
pub const CTL_KERN: ::c_int = 1;
pub const CTL_VM: ::c_int = 2;
pub const CTL_VFS: ::c_int = 3;
pub const CTL_NET: ::c_int = 4;
pub const CTL_DEBUG: ::c_int = 5;
pub const CTL_HW: ::c_int = 6;
pub const CTL_MACHDEP: ::c_int = 7;
pub const CTL_USER: ::c_int = 8;
pub const CTL_MAXID: ::c_int = 9;
pub const KERN_OSTYPE: ::c_int = 1;
pub const KERN_OSRELEASE: ::c_int = 2;
pub const KERN_OSREV: ::c_int = 3;
pub const KERN_VERSION: ::c_int = 4;
pub const KERN_MAXVNODES: ::c_int = 5;
pub const KERN_MAXPROC: ::c_int = 6;
pub const KERN_MAXFILES: ::c_int = 7;
pub const KERN_ARGMAX: ::c_int = 8;
pub const KERN_SECURELVL: ::c_int = 9;
pub const KERN_HOSTNAME: ::c_int = 10;
pub const KERN_HOSTID: ::c_int = 11;
pub const KERN_CLOCKRATE: ::c_int = 12;
pub const KERN_VNODE: ::c_int = 13;
pub const KERN_PROC: ::c_int = 14;
pub const KERN_FILE: ::c_int = 15;
pub const KERN_PROF: ::c_int = 16;
pub const KERN_POSIX1: ::c_int = 17;
pub const KERN_NGROUPS: ::c_int = 18;
pub const KERN_JOB_CONTROL: ::c_int = 19;
pub const KERN_SAVED_IDS: ::c_int = 20;
pub const KERN_BOOTTIME: ::c_int = 21;
pub const KERN_NISDOMAINNAME: ::c_int = 22;
pub const KERN_DOMAINNAME: ::c_int = KERN_NISDOMAINNAME;
pub const KERN_MAXPARTITIONS: ::c_int = 23;
pub const KERN_KDEBUG: ::c_int = 24;
pub const KERN_UPDATEINTERVAL: ::c_int = 25;
pub const KERN_OSRELDATE: ::c_int = 26;
pub const KERN_NTP_PLL: ::c_int = 27;
pub const KERN_BOOTFILE: ::c_int = 28;
pub const KERN_MAXFILESPERPROC: ::c_int = 29;
pub const KERN_MAXPROCPERUID: ::c_int = 30;
pub const KERN_DUMPDEV: ::c_int = 31;
pub const KERN_IPC: ::c_int = 32;
pub const KERN_DUMMY: ::c_int = 33;
pub const KERN_PS_STRINGS: ::c_int = 34;
pub const KERN_USRSTACK32: ::c_int = 35;
pub const KERN_LOGSIGEXIT: ::c_int = 36;
pub const KERN_SYMFILE: ::c_int = 37;
pub const KERN_PROCARGS: ::c_int = 38;
pub const KERN_NETBOOT: ::c_int = 40;
pub const KERN_SYSV: ::c_int = 42;
pub const KERN_AFFINITY: ::c_int = 43;
pub const KERN_TRANSLATE: ::c_int = 44;
pub const KERN_CLASSIC: ::c_int = KERN_TRANSLATE;
pub const KERN_EXEC: ::c_int = 45;
pub const KERN_CLASSICHANDLER: ::c_int = KERN_EXEC;
pub const KERN_AIOMAX: ::c_int = 46;
pub const KERN_AIOPROCMAX: ::c_int = 47;
pub const KERN_AIOTHREADS: ::c_int = 48;
pub const KERN_COREFILE: ::c_int = 50;
pub const KERN_COREDUMP: ::c_int = 51;
pub const KERN_SUGID_COREDUMP: ::c_int = 52;
pub const KERN_PROCDELAYTERM: ::c_int = 53;
pub const KERN_SHREG_PRIVATIZABLE: ::c_int = 54;
pub const KERN_LOW_PRI_WINDOW: ::c_int = 56;
pub const KERN_LOW_PRI_DELAY: ::c_int = 57;
pub const KERN_POSIX: ::c_int = 58;
pub const KERN_USRSTACK64: ::c_int = 59;
pub const KERN_NX_PROTECTION: ::c_int = 60;
pub const KERN_TFP: ::c_int = 61;
pub const KERN_PROCNAME: ::c_int = 62;
pub const KERN_THALTSTACK: ::c_int = 63;
pub const KERN_SPECULATIVE_READS: ::c_int = 64;
pub const KERN_OSVERSION: ::c_int = 65;
pub const KERN_SAFEBOOT: ::c_int = 66;
pub const KERN_RAGEVNODE: ::c_int = 68;
pub const KERN_TTY: ::c_int = 69;
pub const KERN_CHECKOPENEVT: ::c_int = 70;
pub const KERN_THREADNAME: ::c_int = 71;
pub const KERN_MAXID: ::c_int = 72;
pub const KERN_RAGE_PROC: ::c_int = 1;
pub const KERN_RAGE_THREAD: ::c_int = 2;
pub const KERN_UNRAGE_PROC: ::c_int = 3;
pub const KERN_UNRAGE_THREAD: ::c_int = 4;
pub const KERN_OPENEVT_PROC: ::c_int = 1;
pub const KERN_UNOPENEVT_PROC: ::c_int = 2;
pub const KERN_TFP_POLICY: ::c_int = 1;
pub const KERN_TFP_POLICY_DENY: ::c_int = 0;
pub const KERN_TFP_POLICY_DEFAULT: ::c_int = 2;
pub const KERN_KDEFLAGS: ::c_int = 1;
pub const KERN_KDDFLAGS: ::c_int = 2;
pub const KERN_KDENABLE: ::c_int = 3;
pub const KERN_KDSETBUF: ::c_int = 4;
pub const KERN_KDGETBUF: ::c_int = 5;
pub const KERN_KDSETUP: ::c_int = 6;
pub const KERN_KDREMOVE: ::c_int = 7;
pub const KERN_KDSETREG: ::c_int = 8;
pub const KERN_KDGETREG: ::c_int = 9;
pub const KERN_KDREADTR: ::c_int = 10;
pub const KERN_KDPIDTR: ::c_int = 11;
pub const KERN_KDTHRMAP: ::c_int = 12;
pub const KERN_KDPIDEX: ::c_int = 14;
pub const KERN_KDSETRTCDEC: ::c_int = 15;
pub const KERN_KDGETENTROPY: ::c_int = 16;
pub const KERN_KDWRITETR: ::c_int = 17;
pub const KERN_KDWRITEMAP: ::c_int = 18;
pub const KERN_KDENABLE_BG_TRACE: ::c_int = 19;
pub const KERN_KDDISABLE_BG_TRACE: ::c_int = 20;
pub const KERN_KDREADCURTHRMAP: ::c_int = 21;
pub const KERN_KDSET_TYPEFILTER: ::c_int = 22;
pub const KERN_KDBUFWAIT: ::c_int = 23;
pub const KERN_KDCPUMAP: ::c_int = 24;
pub const KERN_PROC_ALL: ::c_int = 0;
pub const KERN_PROC_PID: ::c_int = 1;
pub const KERN_PROC_PGRP: ::c_int = 2;
pub const KERN_PROC_SESSION: ::c_int = 3;
pub const KERN_PROC_TTY: ::c_int = 4;
pub const KERN_PROC_UID: ::c_int = 5;
pub const KERN_PROC_RUID: ::c_int = 6;
pub const KERN_PROC_LCID: ::c_int = 7;
pub const KIPC_MAXSOCKBUF: ::c_int = 1;
pub const KIPC_SOCKBUF_WASTE: ::c_int = 2;
pub const KIPC_SOMAXCONN: ::c_int = 3;
pub const KIPC_MAX_LINKHDR: ::c_int = 4;
pub const KIPC_MAX_PROTOHDR: ::c_int = 5;
pub const KIPC_MAX_HDR: ::c_int = 6;
pub const KIPC_MAX_DATALEN: ::c_int = 7;
pub const KIPC_MBSTAT: ::c_int = 8;
pub const KIPC_NMBCLUSTERS: ::c_int = 9;
pub const KIPC_SOQLIMITCOMPAT: ::c_int = 10;
pub const VM_METER: ::c_int = 1;
pub const VM_LOADAVG: ::c_int = 2;
pub const VM_MACHFACTOR: ::c_int = 4;
pub const VM_SWAPUSAGE: ::c_int = 5;
pub const VM_MAXID: ::c_int = 6;
pub const HW_MACHINE: ::c_int = 1;
pub const HW_MODEL: ::c_int = 2;
pub const HW_NCPU: ::c_int = 3;
pub const HW_BYTEORDER: ::c_int = 4;
pub const HW_PHYSMEM: ::c_int = 5;
pub const HW_USERMEM: ::c_int = 6;
pub const HW_PAGESIZE: ::c_int = 7;
pub const HW_DISKNAMES: ::c_int = 8;
pub const HW_DISKSTATS: ::c_int = 9;
pub const HW_EPOCH: ::c_int = 10;
pub const HW_FLOATINGPT: ::c_int = 11;
pub const HW_MACHINE_ARCH: ::c_int = 12;
pub const HW_VECTORUNIT: ::c_int = 13;
pub const HW_BUS_FREQ: ::c_int = 14;
pub const HW_CPU_FREQ: ::c_int = 15;
pub const HW_CACHELINE: ::c_int = 16;
pub const HW_L1ICACHESIZE: ::c_int = 17;
pub const HW_L1DCACHESIZE: ::c_int = 18;
pub const HW_L2SETTINGS: ::c_int = 19;
pub const HW_L2CACHESIZE: ::c_int = 20;
pub const HW_L3SETTINGS: ::c_int = 21;
pub const HW_L3CACHESIZE: ::c_int = 22;
pub const HW_TB_FREQ: ::c_int = 23;
pub const HW_MEMSIZE: ::c_int = 24;
pub const HW_AVAILCPU: ::c_int = 25;
pub const HW_MAXID: ::c_int = 26;
pub const USER_CS_PATH: ::c_int = 1;
pub const USER_BC_BASE_MAX: ::c_int = 2;
pub const USER_BC_DIM_MAX: ::c_int = 3;
pub const USER_BC_SCALE_MAX: ::c_int = 4;
pub const USER_BC_STRING_MAX: ::c_int = 5;
pub const USER_COLL_WEIGHTS_MAX: ::c_int = 6;
pub const USER_EXPR_NEST_MAX: ::c_int = 7;
pub const USER_LINE_MAX: ::c_int = 8;
pub const USER_RE_DUP_MAX: ::c_int = 9;
pub const USER_POSIX2_VERSION: ::c_int = 10;
pub const USER_POSIX2_C_BIND: ::c_int = 11;
pub const USER_POSIX2_C_DEV: ::c_int = 12;
pub const USER_POSIX2_CHAR_TERM: ::c_int = 13;
pub const USER_POSIX2_FORT_DEV: ::c_int = 14;
pub const USER_POSIX2_FORT_RUN: ::c_int = 15;
pub const USER_POSIX2_LOCALEDEF: ::c_int = 16;
pub const USER_POSIX2_SW_DEV: ::c_int = 17;
pub const USER_POSIX2_UPE: ::c_int = 18;
pub const USER_STREAM_MAX: ::c_int = 19;
pub const USER_TZNAME_MAX: ::c_int = 20;
pub const USER_MAXID: ::c_int = 21;
pub const CTL_DEBUG_NAME: ::c_int = 0;
pub const CTL_DEBUG_VALUE: ::c_int = 1;
pub const CTL_DEBUG_MAXID: ::c_int = 20;

pub const POLLRDNORM: ::c_short = 0x040;
pub const POLLWRNORM: ::c_short = 0x004;
pub const POLLRDBAND: ::c_short = 0x080;
pub const POLLWRBAND: ::c_short = 0x100;

pub const PRIO_DARWIN_THREAD: ::c_int = 3;
pub const PRIO_DARWIN_PROCESS: ::c_int = 4;
pub const PRIO_DARWIN_BG: ::c_int = 0x1000;
pub const PRIO_DARWIN_NONUI: ::c_int = 0x1001;

pub const SEM_FAILED: *mut sem_t = -1isize as *mut ::sem_t;

pub const SIGEV_NONE: ::c_int = 0;
pub const SIGEV_SIGNAL: ::c_int = 1;
pub const SIGEV_THREAD: ::c_int = 3;

pub const AIO_CANCELED: ::c_int = 2;
pub const AIO_NOTCANCELED: ::c_int = 4;
pub const AIO_ALLDONE: ::c_int = 1;
pub const AIO_LISTIO_MAX: ::c_int = 16;
pub const LIO_NOP: ::c_int = 0;
pub const LIO_WRITE: ::c_int = 2;
pub const LIO_READ: ::c_int = 1;
pub const LIO_WAIT: ::c_int = 2;
pub const LIO_NOWAIT: ::c_int = 1;

pub const WEXITED: ::c_int = 0x00000004;
pub const WSTOPPED: ::c_int = 0x00000008;
pub const WCONTINUED: ::c_int = 0x00000010;
pub const WNOWAIT: ::c_int = 0x00000020;

pub const P_ALL: idtype_t = 0;
pub const P_PID: idtype_t = 1;
pub const P_PGID: idtype_t = 2;

pub const XATTR_NOFOLLOW: ::c_int = 0x0001;
pub const XATTR_CREATE: ::c_int = 0x0002;
pub const XATTR_REPLACE: ::c_int = 0x0004;
pub const XATTR_NOSECURITY: ::c_int = 0x0008;
pub const XATTR_NODEFAULT: ::c_int = 0x0010;
pub const XATTR_SHOWCOMPRESSION: ::c_int = 0x0020;

f! {
    pub fn WSTOPSIG(status: ::c_int) -> ::c_int {
        status >> 8
    }

    pub fn _WSTATUS(status: ::c_int) -> ::c_int {
        status & 0x7f
    }

    pub fn WIFCONTINUED(status: ::c_int) -> bool {
        _WSTATUS(status) == _WSTOPPED && WSTOPSIG(status) == 0x13
    }

    pub fn WIFSIGNALED(status: ::c_int) -> bool {
        _WSTATUS(status) != _WSTOPPED && _WSTATUS(status) != 0
    }

    pub fn WIFSTOPPED(status: ::c_int) -> bool {
        _WSTATUS(status) == _WSTOPPED && WSTOPSIG(status) != 0x13
    }
}

extern {
    pub fn aio_read(aiocbp: *mut aiocb) -> ::c_int;
    pub fn aio_write(aiocbp: *mut aiocb) -> ::c_int;
    pub fn aio_fsync(op: ::c_int, aiocbp: *mut aiocb) -> ::c_int;
    pub fn aio_error(aiocbp: *const aiocb) -> ::c_int;
    pub fn aio_return(aiocbp: *mut aiocb) -> ::ssize_t;
    #[cfg_attr(all(target_os = "macos", target_arch = "x86"),
               link_name = "aio_suspend$UNIX2003")]
    pub fn aio_suspend(aiocb_list: *const *const aiocb, nitems: ::c_int,
                       timeout: *const ::timespec) -> ::c_int;
    pub fn aio_cancel(fd: ::c_int, aiocbp: *mut aiocb) -> ::c_int;
    pub fn lio_listio(mode: ::c_int, aiocb_list: *const *mut aiocb,
                      nitems: ::c_int, sevp: *mut sigevent) -> ::c_int;

    pub fn dirfd(dirp: *mut ::DIR) -> ::c_int;

    pub fn lutimes(file: *const ::c_char, times: *const ::timeval) -> ::c_int;

    pub fn getutxent() -> *mut utmpx;
    pub fn getutxid(ut: *const utmpx) -> *mut utmpx;
    pub fn getutxline(ut: *const utmpx) -> *mut utmpx;
    pub fn pututxline(ut: *const utmpx) -> *mut utmpx;
    pub fn setutxent();
    pub fn endutxent();
    pub fn utmpxname(file: *const ::c_char) -> ::c_int;

    pub fn getnameinfo(sa: *const ::sockaddr,
                       salen: ::socklen_t,
                       host: *mut ::c_char,
                       hostlen: ::socklen_t,
                       serv: *mut ::c_char,
                       sevlen: ::socklen_t,
                       flags: ::c_int) -> ::c_int;
    pub fn mincore(addr: *const ::c_void, len: ::size_t,
                   vec: *mut ::c_char) -> ::c_int;
    pub fn sysctlnametomib(name: *const ::c_char,
                           mibp: *mut ::c_int,
                           sizep: *mut ::size_t)
                           -> ::c_int;
    #[cfg_attr(all(target_os = "macos", target_arch = "x86"),
               link_name = "mprotect$UNIX2003")]
    pub fn mprotect(addr: *mut ::c_void, len: ::size_t, prot: ::c_int)
                    -> ::c_int;
    pub fn shm_open(name: *const ::c_char, oflag: ::c_int, ...) -> ::c_int;
    pub fn sysctl(name: *mut ::c_int,
                  namelen: ::c_uint,
                  oldp: *mut ::c_void,
                  oldlenp: *mut ::size_t,
                  newp: *mut ::c_void,
                  newlen: ::size_t)
                  -> ::c_int;
    pub fn sysctlbyname(name: *const ::c_char,
                        oldp: *mut ::c_void,
                        oldlenp: *mut ::size_t,
                        newp: *mut ::c_void,
                        newlen: ::size_t)
                        -> ::c_int;
    pub fn mach_absolute_time() -> u64;
    pub fn mach_timebase_info(info: *mut ::mach_timebase_info) -> ::c_int;
    pub fn pthread_setname_np(name: *const ::c_char) -> ::c_int;
    pub fn pthread_get_stackaddr_np(thread: ::pthread_t) -> *mut ::c_void;
    pub fn pthread_get_stacksize_np(thread: ::pthread_t) -> ::size_t;
    pub fn pthread_condattr_setpshared(attr: *mut pthread_condattr_t,
                                       pshared: ::c_int) -> ::c_int;
    pub fn pthread_condattr_getpshared(attr: *const pthread_condattr_t,
                                       pshared: *mut ::c_int) -> ::c_int;
    pub fn pthread_mutexattr_setpshared(attr: *mut pthread_mutexattr_t,
                                        pshared: ::c_int) -> ::c_int;
    pub fn pthread_mutexattr_getpshared(attr: *const pthread_mutexattr_t,
                                        pshared: *mut ::c_int) -> ::c_int;
    pub fn pthread_rwlockattr_getpshared(attr: *const pthread_rwlockattr_t,
                                         val: *mut ::c_int) -> ::c_int;
    pub fn pthread_rwlockattr_setpshared(attr: *mut pthread_rwlockattr_t,
                                         val: ::c_int) -> ::c_int;
    pub fn __error() -> *mut ::c_int;
    pub fn backtrace(buf: *mut *mut ::c_void,
                     sz: ::c_int) -> ::c_int;
    #[cfg_attr(target_os = "macos", link_name = "statfs$INODE64")]
    pub fn statfs(path: *const ::c_char, buf: *mut statfs) -> ::c_int;
    #[cfg_attr(target_os = "macos", link_name = "fstatfs$INODE64")]
    pub fn fstatfs(fd: ::c_int, buf: *mut statfs) -> ::c_int;
    pub fn kevent(kq: ::c_int,
                  changelist: *const ::kevent,
                  nchanges: ::c_int,
                  eventlist: *mut ::kevent,
                  nevents: ::c_int,
                  timeout: *const ::timespec) -> ::c_int;
    pub fn kevent64(kq: ::c_int,
                    changelist: *const ::kevent64_s,
                    nchanges: ::c_int,
                    eventlist: *mut ::kevent64_s,
                    nevents: ::c_int,
                    flags: ::c_uint,
                    timeout: *const ::timespec) -> ::c_int;
    pub fn mount(src: *const ::c_char,
                 target: *const ::c_char,
                 flags: ::c_int,
                 data: *mut ::c_void) -> ::c_int;
    pub fn ptrace(requeset: ::c_int,
                  pid: ::pid_t,
                  addr: *mut ::c_char,
                  data: ::c_int) -> ::c_int;
    pub fn quotactl(special: *const ::c_char,
                    cmd: ::c_int,
                    id: ::c_int,
                    data: *mut ::c_char) -> ::c_int;
    pub fn sethostname(name: *const ::c_char, len: ::c_int) -> ::c_int;
    pub fn sendfile(fd: ::c_int,
                    s: ::c_int,
                    offset: ::off_t,
                    len: *mut ::off_t,
                    hdtr: *mut ::sf_hdtr,
                    flags: ::c_int) -> ::c_int;
    pub fn openpty(amaster: *mut ::c_int,
                   aslave: *mut ::c_int,
                   name: *mut ::c_char,
                   termp: *mut termios,
                   winp: *mut ::winsize) -> ::c_int;
    pub fn forkpty(amaster: *mut ::c_int,
                   name: *mut ::c_char,
                   termp: *mut termios,
                   winp: *mut ::winsize) -> ::pid_t;
    pub fn duplocale(base: ::locale_t) -> ::locale_t;
    pub fn freelocale(loc: ::locale_t) -> ::c_int;
    pub fn localeconv_l(loc: ::locale_t) -> *mut lconv;
    pub fn newlocale(mask: ::c_int,
                     locale: *const ::c_char,
                     base: ::locale_t) -> ::locale_t;
    pub fn uselocale(loc: ::locale_t) -> ::locale_t;
    pub fn querylocale(mask: ::c_int, loc: ::locale_t) -> *const ::c_char;
    pub fn getpriority(which: ::c_int, who: ::id_t) -> ::c_int;
    pub fn setpriority(which: ::c_int, who: ::id_t, prio: ::c_int) -> ::c_int;

    pub fn getxattr(path: *const ::c_char, name: *const ::c_char,
                    value: *mut ::c_void, size: ::size_t, position: u32,
                    flags: ::c_int) -> ::ssize_t;
    pub fn fgetxattr(filedes: ::c_int, name: *const ::c_char,
                     value: *mut ::c_void, size: ::size_t, position: u32,
                     flags: ::c_int) -> ::ssize_t;
    pub fn setxattr(path: *const ::c_char, name: *const ::c_char,
                    value: *const ::c_void, size: ::size_t, position: u32,
                    flags: ::c_int) -> ::c_int;
    pub fn fsetxattr(filedes: ::c_int, name: *const ::c_char,
                     value: *const ::c_void, size: ::size_t, position: u32,
                     flags: ::c_int) -> ::c_int;
    pub fn listxattr(path: *const ::c_char, list: *mut ::c_char,
                     size: ::size_t, flags: ::c_int) -> ::ssize_t;
    pub fn flistxattr(filedes: ::c_int, list: *mut ::c_char,
                      size: ::size_t, flags: ::c_int) -> ::ssize_t;
    pub fn removexattr(path: *const ::c_char, name: *const ::c_char,
                       flags: ::c_int) -> ::c_int;
    pub fn fremovexattr(filedes: ::c_int, name: *const ::c_char,
                        flags: ::c_int) -> ::c_int;

    pub fn initgroups(user: *const ::c_char, basegroup: ::c_int) -> ::c_int;

    #[cfg_attr(all(target_os = "macos", target_arch = "x86"),
               link_name = "waitid$UNIX2003")]
    pub fn waitid(idtype: idtype_t, id: id_t, infop: *mut ::siginfo_t,
                  options: ::c_int) -> ::c_int;
    pub fn brk(addr: *const ::c_void) -> *mut ::c_void;
    pub fn sbrk(increment: ::c_int) -> *mut ::c_void;
    pub fn settimeofday(tv: *const ::timeval, tz: *const ::timezone) -> ::c_int;
}

cfg_if! {
    if #[cfg(any(target_arch = "arm", target_arch = "x86"))] {
        mod b32;
        pub use self::b32::*;
    } else if #[cfg(any(target_arch = "x86_64", target_arch = "aarch64"))] {
        mod b64;
        pub use self::b64::*;
    } else {
        // Unknown target_arch
    }
}
