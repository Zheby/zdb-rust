pub struct Ops{
    pub op:String
}
pub enum Type{
    String,
    Int,
    Float,
    Vec,
    Other
}
impl Ops{
    pub fn new() ->Ops{
        Self{
            op: "".to_string()
        }
    }
    pub fn show_all(&mut self) {
       return self.op.push_str("show all")
    }

    pub fn show_pool(&mut self,pool:String) {
        return self.op.push_str(format!("show {}",pool).as_str())
    }

    pub fn show_pool_key(&mut self,pool:String,key:String){
        return self.op.push_str(format!("show {} {}",pool,key).as_str())
    }

    pub fn add_pool(&mut self,pool:String){
        return self.op.push_str(format!("add pool {}",pool).as_str())
    }

    pub fn add_key(&mut self,pool:String,key:String,ty:Type,value:String){
        return self.op.push_str(format!("add {} {} {} {}",pool,key,ty.go_string(),value).as_str())
    }
    pub fn del_pool(&mut self,pool:String){
        return self.op.push_str(format!("del pool {}",pool).as_str());
    }
     pub fn del_key(&mut self,pool:String,key:String){
         return self.op.push_str(format!("del {} {}",pool,key).as_str())
     }

    pub fn del_pool_all(&mut self,pool:String){
        self.op.push_str(format!("del {} all",pool).as_str())
    }

    pub fn clear(&mut self){
        self.op.clear();
    }

    pub fn go_byte(&self) ->Vec<u8>{
        let mut xs = self.op.as_bytes().to_vec();
        xs.push(230);
        xs
    }


}

impl Type {
    pub fn go_string(&self) -> String {
        match  self{
            Type::String => {"string".to_string()}
            Type::Int => {"int".to_string()}
            Type::Float => {"float".to_string()}
            Type::Vec => {"vec".to_string()}
            Type::Other => {"other".to_string()}
        }
    }
}
