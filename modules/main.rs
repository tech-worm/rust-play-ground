mod  c{
    pub fn c() {
        println!("c is a module");
    }
}

mod cp{
    pub fn cp(){
        println!("cp is a module");
    }
}

fn main(){
    c::c();
    cp::cp();
}
