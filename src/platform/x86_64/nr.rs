pub const SYS_READ: usize = 0;
pub const SYS_WRITE: usize = 1;
pub const SYS_OPEN: usize = 2;
pub const SYS_CLOSE: usize = 3;
pub const SYS_STAT: usize = 4;
pub const SYS_FSTAT: usize = 5;

pub const SYS_POLL: usize = 7;
pub const SYS_LSEEK: usize = 8;
pub const SYS_MMAP: usize = 9;
pub const SYS_MPROTECT: usize = 10;
pub const SYS_MUNMAP: usize = 11;
pub const SYS_BRK: usize = 12;
pub const SYS_SIGACTION: usize = 13;
pub const SYS_RT_SIGPROCMASK: usize = 14;
pub const SYS_RT_SIGRETURN: usize = 15;
pub const SYS_IOCTL: usize = 16;
pub const SYS_WRITEV: usize = 20;

pub const SYS_MADVISE: usize = 28;

pub const SYS_DUP: usize = 32;
pub const SYS_DUP2: usize = 33;

pub const SYS_NANOSLEEP: usize = 35;

pub const SYS_GETPID: usize = 39;

pub const SYS_SOCKET: usize = 41;
pub const SYS_CONNECT: usize = 42;
pub const SYS_ACCEPT: usize = 43;
pub const SYS_SENDTO: usize = 44;
pub const SYS_RECVFROM: usize = 45;

pub const SYS_RECVMSG: usize = 47;
pub const SYS_SHUTDOWN: usize = 48;
pub const SYS_BIND: usize = 49;
pub const SYS_LISTEN: usize = 50;
pub const SYS_GETSOCKNAME: usize = 51;
pub const SYS_GETPEERNAME: usize = 52;
pub const SYS_SOCKET_PAIR: usize = 53;
pub const SYS_SETSOCKOPT: usize = 54;
pub const SYS_GETSOCKOPT: usize = 55;
pub const SYS_CLONE: usize = 56;
pub const SYS_FORK: usize = 57;
pub const SYS_VFORK: usize = 58;
pub const SYS_EXECVE: usize = 59;
pub const SYS_EXIT: usize = 60;
pub const SYS_WAIT4: usize = 61;
pub const SYS_KILL: usize = 62;

pub const SYS_FCNTL: usize = 72;

pub const SYS_FTRUNCATE: usize = 77;
pub const SYS_GET_DENTS: usize = 78;

pub const SYS_GETCWD: usize = 79;

pub const SYS_CHDIR: usize = 80;

pub const SYS_MKDIR: usize = 83;

pub const SYS_READLINK: usize = 89;

pub const SYS_GETTIMEOFDAY: usize = 96;
pub const SYS_GETRUSAGE: usize = 98;

pub const SYS_GETUID: usize = 102;
pub const SYS_SYSLOG: usize = 103;
pub const SYS_GETGID: usize = 104;
pub const SYS_SETUID: usize = 105;

pub const SYS_SETGID: usize = 106;
pub const SYS_GETEUID: usize = 107;
pub const SYS_GETEGID: usize = 108;

pub const SYS_GETPPID: usize = 110;
pub const SYS_GETPGID: usize = 121;
pub const SYS_SIGALTSTACK: usize = 131;
pub const SYS_MKNOD: usize = 133;

pub const SYS_ARCH_PRCTL: usize = 158;

pub const SYS_REBOOT: usize = 169;

pub const SYS_GETTID: usize = 186;

pub const SYS_FUTEX: usize = 202;

pub const SYS_GET_DENTS_64: usize = 217;
pub const SYS_SET_TID_ADDR: usize = 218;

pub const SYS_UNLINK_AT: usize = 263;
pub const SYS_READLINK_AT: usize = 267;
pub const SYS_ACCEPT4: usize = 288;
pub const SYS_PIPE: usize = 293;

pub const SYS_GET_RANDOM: usize = 318;

// 与linux不一致的调用，在linux基础上累加
// 独立系统调用号大于100000
pub const SYS_PUT_STRING: usize = 100000;

// TODO： 和linux不一致
pub const SYS_SBRK: usize = 100001;

/// todo: 该系统调用与Linux不一致，将来需要删除该系统调用！！！ 删的时候记得改C版本的libc
pub const SYS_CLOCK: usize = 100002;
pub const SYS_SCHED: usize = 100003;
