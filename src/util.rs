use std::{
	io,
	os::fd::AsRawFd,
	ptr
};

pub fn read_u16_ne(src:&[u8])->u16{
	u16::from_ne_bytes(src.try_into().unwrap())
}
pub fn write_u16_ne(dst:&mut [u8],num:u16){
	dst.copy_from_slice(&num.to_ne_bytes());
}
pub fn write_u64_ne(dst:&mut [u8],num:u64){
	dst.copy_from_slice(&num.to_ne_bytes());
}
pub fn write_u32_ne(dst:&mut [u8],num:u32){
	dst.copy_from_slice(&num.to_ne_bytes());
}
pub fn set_ns<T:AsRawFd>(file:&T,ns_type:i32)->io::Result<()>{
	let r=unsafe{
		libc::setns(file.as_raw_fd(),ns_type)
	};
	if r==-1{
		Err(io::Error::last_os_error())
	}else{
		Ok(())
	}
}
pub fn ioctl_bytes<T:AsRawFd>(file:&T,req:u64,data:&mut [u8])->io::Result<i32>{
	let r=unsafe{
		libc::ioctl(file.as_raw_fd(),req,data.as_mut_ptr())
	};
	if r==-1{
		Err(io::Error::last_os_error())
	}else{
		Ok(r)
	}
}
pub fn send_msg<T:AsRawFd>(
	sock:&T,
	data:&[u8],
	cmsg:&[u8],
	flags:i32,
	addr:Option<&[u8]>
)->io::Result<usize>{
	let mut iov=libc::iovec{
		iov_base:data.as_ptr() as _,
		iov_len:data.len()
	};
	let iov_ptr:*mut _=&mut iov;
	let mut h=libc::msghdr{
		msg_name:ptr::null_mut(),
		msg_namelen:0,
		msg_iov:iov_ptr as _,
		msg_iovlen:1,
		msg_control:cmsg.as_ptr() as _,
		msg_controllen:cmsg.len(),
		msg_flags:0
	};
	if let Some(a)=addr{
		h.msg_name=a.as_ptr() as _;
		h.msg_namelen=a.len() as _;
	}
	let h_ptr:*const _=&h;
	let r=unsafe{
		libc::sendmsg(sock.as_raw_fd(),h_ptr,flags)
	};
	if r==-1{
		Err(io::Error::last_os_error())
	}else{
		Ok(r as usize)
	}
}