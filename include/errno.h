#pragma once
extern int errno;

// AI GENERATED
// I was to lazy to write this myself and also I did a very naive review soo....

#define EPERM                0x13  /* MissingPermissions */
#define ENOTSUP              0x02  /* OperationNotSupported */
#define EOPNOTSUPP           0x02  /* Alias */
#define ENOSYS               0x05  /* InvalidSyscall */
#define EINVAL               0x1B  /* InvalidArgument */
#define EFAULT               0x09  /* InvalidPtr */
#define EILSEQ               0x0A  /* InvalidStr */
#define ENAMETOOLONG         0x0B  /* StrTooLong */
#define ENOENT               0x0D  /* NoSuchAFileOrDirectory */
#define ENOTDIR              0x0F  /* NotADirectory */
#define EISDIR               0x0E  /* NotAFile */
#define EEXIST               0x10  /* AlreadyExists */
#define ENOEXEC              0x11  /* NotExecutable */
#define ENOTEMPTY            0x12  /* DirectoryNotEmpty */
#define EBUSY                0x15  /* Busy */
#define ENOMEM               0x17  /* OutOfMemory */
#define ETIMEDOUT            0x19  /* Timeout */
#define EPIPE                0x20  /* ConnectionClosed */
#define ECONNREFUSED         0x21  /* ConnectionRefused */
#define EWOULDBLOCK          0x1F  /* WouldBlock */
#define EAGAIN               0x1F  /* Alias */
#define EMSGSIZE             0x25  /* TooShort */
#define EADDRNOTAVAIL        0x26  /* AddressNotFound */
#define EADDRINUSE           0x29  /* AddressAlreadyInUse */
#define ENOTCONN             0x2A  /* NotBound */
#define EHOSTUNREACH         0x2B  /* HostUnreachable */
#define ENETUNREACH          0x2C  /* NetworkUnreachable */
#define EPROTONOSUPPORT      0x2D  /* ProtocolNotSupported */

/* SafaOS-specific errno extensions
 * These have NO POSIX equivalent.
 */
#define EGENERIC             0x01
#define ENOTSUPPORTED        0x03
#define ECORRUPTED           0x04
#define EUNKNOWNRESOURCE     0x06
#define EINVALIDPID          0x07
#define EINVALIDOFFSET       0x08
#define EINVALIDPATH         0x0C
#define EMMAP                0x14
#define ENOTENOUGHARGS       0x16
#define EINVALIDTID          0x18
#define EINVALIDCOMMAND      0x1A
#define EUNKNOWNERROR        0x1C
#define EPANIC               0x1D
#define ENOTDEVICE           0x1E
#define EUNSUPPORTEDRESOURCE 0x22
#define ERESOURCECLONEFAILED 0x23
#define ETYPEMISMATCH        0x24
#define EINVALIDSIZE         0x27
#define EFORCETERMINATED     0x28
