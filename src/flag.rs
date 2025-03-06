use std::{
	collections::HashMap
};

#[derive(Debug)]
pub struct FlagSet{
	pub flag_map:HashMap<String,Option<String>>,
	pub result_map:HashMap<String,Option<String>>,
	pub non_flag:Vec<String>
}
impl FlagSet{
	pub fn new()->FlagSet{
		FlagSet{
			flag_map:HashMap::new(),
			result_map:HashMap::new(),
			non_flag:Vec::new()
		}
	}
	pub fn add_flag(&mut self,flag:&str,value:Option<String>){
		if let Some(v)=&value{
			self.result_map.insert(String::from(flag),Some(v.clone()));
		}
		self.flag_map.insert(String::from(flag),value);
	}
	pub fn parse(&mut self,input:&[String]){
		let mut i=0;
		while i<input.len(){
			let mut arg=input[i].clone();
			if arg.len()<2{
				self.non_flag.push(arg);
				i+=1;
				continue;
			}else if arg.chars().nth(0)!=Some('-'){
				self.non_flag.push(arg);
				i+=1;
				continue;
			}

			arg=String::from(&arg[1..]);
			match self.flag_map.get(&arg){
				None=>self.non_flag.push(arg),
				Some(&Some(_))=>{
					i+=1;
					if i<input.len(){
						self.result_map.insert(arg,Some(input[i].clone()));
					}
					()
				},
				Some(&None)=>{
					self.result_map.insert(arg,None);
					()
				}
			}
			i+=1;
		}
	}
}