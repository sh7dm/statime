mod control_message;
mod interface;
pub mod linux;
mod raw_udp_socket;
mod timestamped_udp_socket;

/// Turn a C failure (-1 is returned) into a rust Result
pub(crate) fn cerr(t: libc::c_int) -> std::io::Result<libc::c_int> {
    match t {
        -1 => Err(std::io::Error::last_os_error()),
        _ => Ok(t),
    }
}
