use std::io::{Read, Write};
use std::net::TcpStream;
use std::net::Ipv4Addr;
use std::str::FromStr;
use std::string::FromUtf8Error;

pub const LOCATION:Ipv4Addr = Ipv4Addr::new(127,0,0,1);

#[derive(Debug)]
pub struct User{
    account:String,
    password:String

}

#[derive(Debug)]
pub struct Link{
    user:User,
    ip:Ipv4Addr,
}
impl Link{
    pub fn new(ip:String) -> Link {
        Self{
            user: User {
                account: "".to_string(),
                password: "".to_string(),
            },
            ip: Ipv4Addr::from_str(ip.as_str()).unwrap(),
        }
    }
    pub fn with_user(&mut self,user:String){
        self.user.account = user;
    }
    pub fn with_password(&mut self,password:String){
        self.user.password = password;
    }
    pub fn socket(&self, statement:String) -> Result<String, FromUtf8Error> {
        let mut tcp = TcpStream::connect((self.ip,3045)).unwrap();
        let input = format!("{} {} {}",self.user.account,self.user.password,statement).as_bytes().to_vec();
        let mut output = [0;100000];
        tcp.write(input.as_slice()).unwrap();
        tcp.read(output.as_mut_slice()).unwrap();
        let mut nxs = vec![];
        for i in output{
            if i != 0{
                nxs.push(i);
            }
        }
        let xs = String::from_utf8(nxs);
        return xs
    }
}