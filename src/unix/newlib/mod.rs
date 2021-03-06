use dox::mem;

pub type blkcnt_t = i32;
pub type blksize_t = i32;
pub type clock_t = i32;
pub type clockid_t = ::c_ulong;
pub type dev_t = u32;
pub type fsblkcnt_t = u64;
pub type fsfilcnt_t = u32;
pub type id_t = u32;
pub type ino_t = u32;
pub type key_t = ::c_int;
pub type loff_t = ::c_longlong;
pub type mode_t = ::c_uint;
pub type nfds_t = u32;
pub type nlink_t = ::c_ushort;
pub type off_t = i64;
pub type pthread_t = ::c_ulong;
pub type pthread_key_t = ::c_uint;
pub type rlim_t = u32;
pub type sa_family_t = u8;
pub type socklen_t = u32;
pub type speed_t = u32;
pub type suseconds_t = i32;
pub type tcflag_t = ::c_uint;
pub type time_t = i32;
pub type useconds_t = u32;

s! {
    pub struct sockaddr {
        pub sa_family: sa_family_t,
        pub sa_data: [::c_char; 14],
    }

    pub struct sockaddr_in {
        pub sin_family: sa_family_t,
        pub sin_port: ::in_port_t,
        pub sin_addr: ::in_addr,
        pub sin_zero: [u8; 8],
    }

    pub struct sockaddr_in6 { // Unverified
        pub sin6_family: sa_family_t,
        pub sin6_port: ::in_port_t,
        pub sin6_flowinfo: u32,
        pub sin6_addr: ::in6_addr,
        pub sin6_scope_id: u32,
    }

    pub struct sockaddr_storage {
        pub ss_family: sa_family_t,
        pub __ss_padding: [u8; 26],
    }

    pub struct addrinfo {
        pub ai_flags: ::c_int,
        pub ai_family: ::c_int,
        pub ai_socktype: ::c_int,
        pub ai_protocol: ::c_int,
        pub ai_addrlen: socklen_t,
        pub ai_canonname: *mut ::c_char,
        pub ai_addr: *mut sockaddr,
        pub ai_next: *mut addrinfo,
    }

    pub struct ip_mreq {
        pub imr_multiaddr: in_addr,
        pub imr_interface: in_addr,
    }

    pub struct linger {
        pub l_onoff: ::c_int,
        pub l_linger: ::c_int,
    }

    pub struct in_addr {
            pub s_addr: ::in_addr_t,
    }

    pub struct hostent {
            pub h_name: *mut ::c_char,
            pub h_aliases: *mut *mut ::c_char,
            pub h_addrtype: ::c_int,
            pub h_length: ::c_int,
            pub h_addr_list: *mut *mut ::c_char,
            pub h_addr: *mut ::c_char,
    }

    pub struct pollfd {
        pub fd: ::c_int,
        pub events: ::c_int,
        pub revents: ::c_int,
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
        pub int_n_cs_precedes: ::c_char,
        pub int_n_sep_by_space: ::c_char,
        pub int_n_sign_posn: ::c_char,
        pub int_p_cs_precedes: ::c_char,
        pub int_p_sep_by_space: ::c_char,
        pub int_p_sign_posn: ::c_char,
    }

    pub struct tm {
        pub tm_sec: ::c_int,
        pub tm_min: ::c_int,
        pub tm_hour: ::c_int,
        pub tm_mday: ::c_int,
        pub tm_mon: ::c_int,
        pub tm_year: ::c_int,
        pub tm_wday: ::c_int,
        pub tm_yday: ::c_int,
        pub tm_isdst: ::c_int,
    }

    pub struct stat {
        pub st_dev: ::dev_t,
        pub st_ino: ::ino_t,
        pub st_mode: ::mode_t,
        pub st_nlink: ::nlink_t,
        pub st_uid: ::uid_t,
        pub st_gid: ::gid_t,
        pub st_rdev: dev_t,
        pub st_size: off_t,
        pub st_atime: time_t,
        pub st_spare1: ::c_long,
        pub st_mtime: time_t,
        pub st_spare2: ::c_long,
        pub st_ctime: time_t,
        pub st_spare3: ::c_long,
        pub st_blksize: blksize_t,
        pub st_blocks: blkcnt_t,
        pub st_spare4: [::c_long; 2usize],
    }

    pub struct statvfs {
        pub f_bsize: ::c_ulong,
        pub f_frsize: ::c_ulong,
        pub f_blocks: fsblkcnt_t,
        pub f_bfree: fsblkcnt_t,
        pub f_bavail: fsblkcnt_t,
        pub f_files: fsfilcnt_t,
        pub f_ffree: fsfilcnt_t,
        pub f_favail: fsfilcnt_t,
        pub f_fsid: ::c_ulong,
        pub f_flag: ::c_ulong,
        pub f_namemax: ::c_ulong,
    }

    pub struct sigset_t {
        __val: [::c_ulong; 16],
    }

    pub struct sigaction {
        pub sa_handler: extern fn(arg1: ::c_int),
        pub sa_mask: sigset_t,
        pub sa_flags: ::c_int,
    }

    pub struct dirent {
        pub d_ino: ino_t,
        pub d_type: ::c_uchar,
        pub d_name: [::c_char; 256usize],
    }

    pub struct stack_t {
        pub ss_sp: *mut ::c_void,
        pub ss_flags: ::c_int,
        pub ss_size: usize,
    }

    pub struct fd_set { // Unverified
        fds_bits: [::c_ulong; FD_SETSIZE / ULONG_SIZE],
    }

    pub struct passwd { // Unverified
        pub pw_name: *mut ::c_char,
        pub pw_passwd: *mut ::c_char,
        pub pw_uid: ::uid_t,
        pub pw_gid: ::gid_t,
        pub pw_gecos: *mut ::c_char,
        pub pw_dir: *mut ::c_char,
        pub pw_shell: *mut ::c_char,
    }

    pub struct termios { // Unverified
        pub c_iflag: ::tcflag_t,
        pub c_oflag: ::tcflag_t,
        pub c_cflag: ::tcflag_t,
        pub c_lflag: ::tcflag_t,
        pub c_line: ::cc_t,
        pub c_cc: [::cc_t; ::NCCS],
    }

    pub struct sem_t { // Unverified
        __size: [::c_char; 16],
    }

    pub struct Dl_info { // Unverified
        pub dli_fname: *const ::c_char,
        pub dli_fbase: *mut ::c_void,
        pub dli_sname: *const ::c_char,
        pub dli_saddr: *mut ::c_void,
    }

    pub struct utsname { // Unverified
        pub sysname: [::c_char; 65],
        pub nodename: [::c_char; 65],
        pub release: [::c_char; 65],
        pub version: [::c_char; 65],
        pub machine: [::c_char; 65],
        pub domainname: [::c_char; 65]
    }

    pub struct cpu_set_t { // Unverified
        bits: [u32; 32],
    }

    pub struct pthread_attr_t { // Unverified
        __size: [u64; 7]
    }

    pub struct pthread_mutex_t { // Unverified
        #[cfg(any(target_arch = "mips", target_arch = "arm",
                  target_arch = "powerpc"))]
        __align: [::c_long; 0],
        #[cfg(not(any(target_arch = "mips", target_arch = "arm",
                      target_arch = "powerpc")))]
        __align: [::c_longlong; 0],
        size: [u8; __SIZEOF_PTHREAD_MUTEX_T],
    }

    pub struct pthread_rwlock_t { // Unverified
        #[cfg(any(target_arch = "mips", target_arch = "arm",
                  target_arch = "powerpc"))]
        __align: [::c_long; 0],
        #[cfg(not(any(target_arch = "mips", target_arch = "arm",
                      target_arch = "powerpc")))]
        __align: [::c_longlong; 0],
        size: [u8; __SIZEOF_PTHREAD_RWLOCK_T],
    }

    pub struct pthread_mutexattr_t { // Unverified
        #[cfg(any(target_arch = "x86_64", target_arch = "powerpc64",
                  target_arch = "mips64", target_arch = "s390x",
                  target_arch = "sparc64"))]
        __align: [::c_int; 0],
        #[cfg(not(any(target_arch = "x86_64", target_arch = "powerpc64",
                      target_arch = "mips64", target_arch = "s390x",
                      target_arch = "sparc64")))]
        __align: [::c_long; 0],
        size: [u8; __SIZEOF_PTHREAD_MUTEXATTR_T],
    }

    pub struct pthread_rwlockattr_t { // Unverified
        __lockkind: ::c_int,
        __pshared: ::c_int,
    }

    pub struct pthread_cond_t { // Unverified
        __align: [::c_longlong; 0],
        size: [u8; __SIZEOF_PTHREAD_COND_T],
    }

    pub struct pthread_condattr_t { // Unverified
        __align: [::c_int; 0],
        size: [u8; __SIZEOF_PTHREAD_CONDATTR_T],
    }

}

// unverified constants
pub const PTHREAD_MUTEX_INITIALIZER: pthread_mutex_t = pthread_mutex_t {
    __align: [],
    size: [0; __SIZEOF_PTHREAD_MUTEX_T],
};
pub const PTHREAD_COND_INITIALIZER: pthread_cond_t = pthread_cond_t {
    __align: [],
    size: [0; __SIZEOF_PTHREAD_COND_T],
};
pub const PTHREAD_RWLOCK_INITIALIZER: pthread_rwlock_t = pthread_rwlock_t {
    __align: [],
    size: [0; __SIZEOF_PTHREAD_RWLOCK_T],
};
pub const NCCS: usize = 32;
pub const __SIZEOF_PTHREAD_ATTR_T: usize = 56;
pub const __SIZEOF_PTHREAD_MUTEX_T: usize = 40;
pub const __SIZEOF_PTHREAD_MUTEXATTR_T: usize = 4;
pub const __SIZEOF_PTHREAD_COND_T: usize = 48;
pub const __SIZEOF_PTHREAD_CONDATTR_T: usize = 4;
pub const __SIZEOF_PTHREAD_RWLOCK_T: usize = 56;
pub const __SIZEOF_PTHREAD_RWLOCKATTR_T: usize = 8;
pub const __SIZEOF_PTHREAD_BARRIER_T: usize = 32;
pub const __SIZEOF_PTHREAD_BARRIERATTR_T: usize = 4;
pub const __PTHREAD_MUTEX_HAVE_PREV: usize = 1;
pub const __PTHREAD_RWLOCK_INT_FLAGS_SHARED: usize = 1;
pub const PTHREAD_MUTEX_NORMAL: ::c_int = 0;
pub const PTHREAD_MUTEX_RECURSIVE: ::c_int = 1;
pub const PTHREAD_MUTEX_ERRORCHECK: ::c_int = 2;
pub const FD_SETSIZE: usize = 1024;
// intentionally not public, only used for fd_set
const ULONG_SIZE: usize = 32;

// Other constants
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
pub const EAGAIN: ::c_int = 11;
pub const ENOMEM: ::c_int = 12;
pub const EACCES: ::c_int = 13;
pub const EFAULT: ::c_int = 14;
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
pub const ENOMSG: ::c_int = 35;
pub const EIDRM: ::c_int = 36;
pub const EDEADLK: ::c_int = 45;
pub const ENOLCK: ::c_int = 46;
pub const ENOSTR: ::c_int = 60;
pub const ENODATA: ::c_int = 61;
pub const ETIME: ::c_int = 62;
pub const ENOSR: ::c_int = 63;
pub const ENOLINK: ::c_int = 67;
pub const EPROTO: ::c_int = 71;
pub const EMULTIHOP: ::c_int = 74;
pub const EBADMSG: ::c_int = 77;
pub const EFTYPE: ::c_int = 79;
pub const ENOSYS: ::c_int = 88;
pub const ENOTEMPTY: ::c_int = 90;
pub const ENAMETOOLONG: ::c_int = 91;
pub const ELOOP: ::c_int = 92;
pub const EOPNOTSUPP: ::c_int = 95;
pub const EPFNOSUPPORT: ::c_int = 96;
pub const ECONNRESET: ::c_int = 104;
pub const ENOBUFS: ::c_int = 105;
pub const EAFNOSUPPORT: ::c_int = 106;
pub const EPROTOTYPE: ::c_int = 107;
pub const ENOTSOCK: ::c_int = 108;
pub const ENOPROTOOPT: ::c_int = 109;
pub const ECONNREFUSED: ::c_int = 111;
pub const EADDRINUSE: ::c_int = 112;
pub const ECONNABORTED: ::c_int = 113;
pub const ENETUNREACH: ::c_int = 114;
pub const ENETDOWN: ::c_int = 115;
pub const ETIMEDOUT: ::c_int = 116;
pub const EHOSTDOWN: ::c_int = 117;
pub const EHOSTUNREACH: ::c_int = 118;
pub const EINPROGRESS: ::c_int = 119;
pub const EALREADY: ::c_int = 120;
pub const EDESTADDRREQ: ::c_int = 121;
pub const EMSGSIZE: ::c_int = 122;
pub const EPROTONOSUPPORT: ::c_int = 123;
pub const EADDRNOTAVAIL: ::c_int = 125;
pub const ENETRESET: ::c_int = 126;
pub const EISCONN: ::c_int = 127;
pub const ENOTCONN: ::c_int = 128;
pub const ETOOMANYREFS: ::c_int = 129;
pub const EDQUOT: ::c_int = 132;
pub const ESTALE: ::c_int = 133;
pub const ENOTSUP: ::c_int = 134;
pub const EILSEQ: ::c_int = 138;
pub const EOVERFLOW: ::c_int = 139;
pub const ECANCELED: ::c_int = 140;
pub const ENOTRECOVERABLE: ::c_int = 141;
pub const EOWNERDEAD: ::c_int = 142;
pub const EWOULDBLOCK: ::c_int = 11;

pub const F_DUPFD: ::c_int = 0;
pub const F_GETFD: ::c_int = 1;
pub const F_SETFD: ::c_int = 2;
pub const F_GETFL: ::c_int = 3;
pub const F_SETFL: ::c_int = 4;
pub const F_GETOWN: ::c_int = 5;
pub const F_SETOWN: ::c_int = 6;
pub const F_GETLK: ::c_int = 7;
pub const F_SETLK: ::c_int = 8;
pub const F_SETLKW: ::c_int = 9;
pub const F_RGETLK: ::c_int = 10;
pub const F_RSETLK: ::c_int = 11;
pub const F_CNVT: ::c_int = 12;
pub const F_RSETLKW: ::c_int = 13;
pub const F_DUPFD_CLOEXEC: ::c_int = 14;

pub const O_RDONLY: ::c_int = 0;
pub const O_WRONLY: ::c_int = 1;
pub const O_RDWR: ::c_int = 2;
pub const O_APPEND: ::c_int = 8;
pub const O_CREAT: ::c_int = 512;
pub const O_TRUNC: ::c_int = 1024;
pub const O_EXCL: ::c_int = 2048;
pub const O_SYNC: ::c_int = 8192;
pub const O_NONBLOCK: ::c_int = 16384;

pub const O_ACCMODE: ::c_int = 3;
pub const O_CLOEXEC: ::c_int = 0x80000;

pub const STDIN_FILENO: ::c_int = 0;
pub const STDOUT_FILENO: ::c_int = 1;
pub const STDERR_FILENO: ::c_int = 2;

pub const SEEK_SET: ::c_int = 0;
pub const SEEK_CUR: ::c_int = 1;
pub const SEEK_END: ::c_int = 2;

pub const FIONBIO: ::c_int = 1;

pub const S_BLKSIZE: ::mode_t = 1024;
pub const S_IREAD: ::mode_t = 256;
pub const S_IWRITE: ::mode_t = 128;
pub const S_IEXEC: ::mode_t = 64;
pub const S_ENFMT: ::mode_t = 1024;
pub const S_IFMT: ::mode_t = 61440;
pub const S_IFDIR: ::mode_t = 16384;
pub const S_IFCHR: ::mode_t = 8192;
pub const S_IFBLK: ::mode_t = 24576;
pub const S_IFREG: ::mode_t = 32768;
pub const S_IFLNK: ::mode_t = 40960;
pub const S_IFSOCK: ::mode_t = 49152;
pub const S_IFIFO: ::mode_t = 4096;
pub const S_IRUSR: ::mode_t = 256;
pub const S_IWUSR: ::mode_t = 128;
pub const S_IXUSR: ::mode_t = 64;
pub const S_IRGRP: ::mode_t = 32;
pub const S_IWGRP: ::mode_t = 16;
pub const S_IXGRP: ::mode_t = 8;
pub const S_IROTH: ::mode_t = 4;
pub const S_IWOTH: ::mode_t = 2;
pub const S_IXOTH: ::mode_t = 1;

pub const SOL_SOCKET: ::c_int = 65535;
pub const SOL_TCP: ::c_int = 6;

pub const PF_UNSPEC: ::c_int = 0;
pub const PF_INET: ::c_int = 2;
pub const PF_INET6: ::c_int = 23;

pub const AF_UNSPEC: ::c_int = 0;
pub const AF_INET: ::c_int = 2;
pub const AF_INET6: ::c_int = 23;

pub const SOCK_STREAM: ::c_int = 1;
pub const SOCK_DGRAM: ::c_int = 2;

pub const MSG_OOB: ::c_int = 1;
pub const MSG_PEEK: ::c_int = 2;
pub const MSG_DONTWAIT: ::c_int = 4;
pub const MSG_DONTROUTE: ::c_int = 0;
pub const MSG_WAITALL: ::c_int = 0;
pub const MSG_MORE: ::c_int = 0;
pub const MSG_NOSIGNAL: ::c_int = 0;

pub const SHUT_RD: ::c_int = 0;
pub const SHUT_WR: ::c_int = 1;
pub const SHUT_RDWR: ::c_int = 2;

pub const SO_REUSEADDR: ::c_int = 4;
pub const SO_LINGER: ::c_int = 128;
pub const SO_OOBINLINE: ::c_int = 256;
pub const SO_SNDBUF: ::c_int = 4097;
pub const SO_RCVBUF: ::c_int = 4098;
pub const SO_SNDLOWAT: ::c_int = 4099;
pub const SO_RCVLOWAT: ::c_int = 4100;
pub const SO_TYPE: ::c_int = 4104;
pub const SO_ERROR: ::c_int = 4105;

pub const SOCK_CLOEXEC: ::c_int = O_CLOEXEC;

pub const INADDR_LOOPBACK: ::c_int = 2130706433;
pub const INADDR_ANY: ::c_int = 0;
pub const INADDR_BROADCAST: ::c_int = 4294967295;
pub const INADDR_NONE: ::c_int = 4294967295;

pub const INET_ADDRSTRLEN: ::c_int = 16;

pub const IPPROTO_IP: ::c_int = 0;
pub const IPPROTO_UDP: ::c_int = 17;
pub const IPPROTO_TCP: ::c_int = 6;

pub const TCP_NODELAY: ::c_int = 8193;
pub const TCP_MAXSEG: ::c_int = 8194;

pub const IP_TOS: ::c_int = 7;
pub const IP_TTL: ::c_int = 8;
pub const IP_MULTICAST_LOOP: ::c_int = 9;
pub const IP_MULTICAST_TTL: ::c_int = 10;
pub const IP_ADD_MEMBERSHIP: ::c_int = 11;
pub const IP_DROP_MEMBERSHIP: ::c_int = 12;

pub const HOST_NOT_FOUND: ::c_int = 1;
pub const NO_DATA: ::c_int = 2;
pub const NO_ADDRESS: ::c_int = 2;
pub const NO_RECOVERY: ::c_int = 3;
pub const TRY_AGAIN: ::c_int = 4;

pub const AI_PASSIVE: ::c_int = 1;
pub const AI_CANONNAME: ::c_int = 2;
pub const AI_NUMERICHOST: ::c_int = 4;
pub const AI_NUMERICSERV: ::c_int = 0;
pub const AI_ADDRCONFIG: ::c_int = 0;

pub const NI_MAXHOST: ::c_int = 1025;
pub const NI_MAXSERV: ::c_int = 32;
pub const NI_NOFQDN: ::c_int = 1;
pub const NI_NUMERICHOST: ::c_int = 2;
pub const NI_NAMEREQD: ::c_int = 4;
pub const NI_NUMERICSERV: ::c_int = 0;
pub const NI_DGRAM: ::c_int = 0;

pub const EAI_FAMILY: ::c_int = -303;
pub const EAI_MEMORY: ::c_int = -304;
pub const EAI_NONAME: ::c_int = -305;
pub const EAI_SOCKTYPE: ::c_int = -307;

f! {
    pub fn FD_CLR(fd: ::c_int, set: *mut fd_set) -> () {
        let bits = mem::size_of_val(&(*set).fds_bits[0]) * 8;
        let fd = fd as usize;
        (*set).fds_bits[fd / bits] &= !(1 << (fd % bits));
        return
    }

    pub fn FD_ISSET(fd: ::c_int, set: *mut fd_set) -> bool {
        let bits = mem::size_of_val(&(*set).fds_bits[0]) * 8;
        let fd = fd as usize;
        return ((*set).fds_bits[fd / bits] & (1 << (fd % bits))) != 0
    }

    pub fn FD_SET(fd: ::c_int, set: *mut fd_set) -> () {
        let bits = mem::size_of_val(&(*set).fds_bits[0]) * 8;
        let fd = fd as usize;
        (*set).fds_bits[fd / bits] |= 1 << (fd % bits);
        return
    }

    pub fn FD_ZERO(set: *mut fd_set) -> () {
        for slot in (*set).fds_bits.iter_mut() {
            *slot = 0;
        }
    }
}

extern {
    pub fn bind(fd: ::c_int, addr: *const sockaddr, len: socklen_t) -> ::c_int;
    pub fn closesocket(sockfd: ::c_int) -> ::c_int;
    pub fn ioctl(fd: ::c_int, request: ::c_ulong, ...) -> ::c_int;
    pub fn recvfrom(fd: ::c_int, buf: *mut ::c_void, n: usize, flags: ::c_int,
                    addr: *mut sockaddr, addr_len: *mut socklen_t) -> isize;
    pub fn getnameinfo(sa: *const sockaddr, salen: socklen_t,
                       host: *mut ::c_char, hostlen: socklen_t,
                       serv: *mut ::c_char, servlen: socklen_t,
                       flags: ::c_int) -> ::c_int;
}

cfg_if! {
    if #[cfg(target_arch = "arm")] {
        mod arm;
        pub use self::arm::*;
    } else {
        // Only tested on ARM so far. Other platforms might have different
        // definitions for types and constants.
        pub use target_arch_not_implemented;
    }
}
