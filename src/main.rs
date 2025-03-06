#[path="util.rs"]
mod util;
use util::{send_msg,set_ns,ioctl_bytes,read_u16_ne,write_u16_ne,write_u32_ne,write_u64_ne};

#[path="flag.rs"]
mod flag;
use flag::FlagSet;

use libc::{
	CLONE_NEWUSER,CLONE_NEWNET,SIOCGIFFLAGS,IFF_POINTOPOINT,IFF_NO_PI,IFF_TUN,IFF_TAP,
	SOL_SOCKET,SCM_RIGHTS
};
use std::{
	fs::{File,OpenOptions},
	net::UdpSocket,
	os::fd::{AsRawFd,FromRawFd,OwnedFd}
};

const TUNSETIFF:u64=0x400454ca;

fn main(){
	let args:Vec<String>=std::env::args().collect();
	let mut flags=FlagSet::new();
	flags.add_flag("sock-fd",Some(String::from("3")));
	flags.add_flag("pid",Some(String::from("-1")));
	flags.add_flag("netns-only",None);
	flags.add_flag("if-name",Some(String::from("tun0")));
	if args.len()==2{
		if args[1]=="-h"{
			println!("flags and default values:\n{:#?}",&flags.flag_map);
			return;
		}
	}
	flags.parse(&args[1..]);

	
	let pid_str=flags.result_map.remove("pid").unwrap().unwrap();
	if !flags.result_map.contains_key("netns-only"){
		let user_ns=File::open(format!("/proc/{}/ns/user",&pid_str)).unwrap();
		set_ns(&user_ns,CLONE_NEWUSER).unwrap();
	}
	let net_ns=File::open(format!("/proc/{}/ns/net",&pid_str)).unwrap();
	set_ns(&net_ns,CLONE_NEWNET).unwrap();

	let if_name=flags.result_map.remove("if-name").unwrap().unwrap();
	if if_name.len()>16{
		println!("interface name too long");
		return;
	}
	let mut ioctl_data=[0u8;64];
	ioctl_data[..if_name.len()].copy_from_slice(if_name.as_bytes());
	let dummy_sock=UdpSocket::bind("127.0.0.1:0").unwrap();
	ioctl_bytes(&dummy_sock,SIOCGIFFLAGS,&mut ioctl_data).unwrap();
	let if_flags=read_u16_ne(&ioctl_data[16..18]);
	println!("ifr_flags: 0x{:x}",if_flags);

	let mut config_flags=IFF_NO_PI as u16;
	if if_flags&(IFF_POINTOPOINT as u16) != 0{
		config_flags|=IFF_TUN as u16;
	}else{
		config_flags|=IFF_TAP as u16;
	}
	write_u16_ne(&mut ioctl_data[16..18],config_flags);
	let tuntap=OpenOptions::new()
		.read(true)
		.write(true)
		.open("/dev/net/tun")
		.unwrap();
	ioctl_bytes(&tuntap,TUNSETIFF,&mut ioctl_data).unwrap();
	println!("TUN/TAP fd: {}",tuntap.as_raw_fd());

	let sock_fd_str=flags.result_map.remove("sock-fd").unwrap().unwrap();
	let raw_fd=i32::from_str_radix(&sock_fd_str,10).unwrap();
	let sock=unsafe{ OwnedFd::from_raw_fd(raw_fd) };
	let dummy_data=[0u8;1];
	let mut cmsg=[0u8;20];
	write_u64_ne(&mut cmsg[..8],20);
	write_u32_ne(&mut cmsg[8..12],SOL_SOCKET as u32);
	write_u32_ne(&mut cmsg[12..16],SCM_RIGHTS as u32);
	write_u32_ne(&mut cmsg[16..20],tuntap.as_raw_fd() as u32);
	send_msg(&sock,&dummy_data,&cmsg,0,None).unwrap();
}