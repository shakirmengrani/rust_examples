mod sey;

enum Property{
    Init(String),
    Scale
}

enum IPAddress{
    V4(u32, u32, u32, u32),
    V6(String),
    Property(Property)
}

struct Mode{
    r#type: String,
    kind: String
}

struct Meme{
    name: String,
    ip_kind: IPAddress,
    mode: Mode
}

impl Meme{
    fn get_name(&self) -> String{
        return self.name.to_string()
    }
}

fn get_property(prop: Property) -> String {
    match prop {
        Property::Init(text) => format!("{}", text),
        Property::Scale => String::from("24")  
    }
}

fn get_ip(ip: IPAddress, prop: Option<Property>) -> String {
    let _prop = prop.unwrap_or(Property::Init(String::from("")));
    match ip {
        IPAddress::V4(n1, n2, n3, n4) => String::from(format!("{}.{}.{}.{}", n1, n2,n3,n4)),
        IPAddress::V6(ip) => String::from(format!("{}", ip)),
        IPAddress::Property(_prop) => get_property(_prop)
    }
}



fn main() {
    let new_meme_mode = Mode{r#type: String::from("ipv4"), kind: String::from("laptop")};
    let new_meme = Meme{
        name: String::from("Hello"), 
        ip_kind: IPAddress::V4(127,0,0,1),
        mode: new_meme_mode
    };
    println!("Name: {}", new_meme.get_name());
    println!("Type: {}", new_meme.mode.r#type);
    println!("Ip: {}", get_ip(new_meme.ip_kind, Some(Property::Scale)));
    println!("Kind: {}", new_meme.mode.kind);
    println!("Ipv6: {}", get_ip(IPAddress::V6("::1".to_string()), Some(Property::Init(String::from("Shakir")))));
    println!("Init: {}", get_property(Property::Init(String::from("Hello"))));
    println!("Scale: {}", get_property(Property::Scale));
    println!("Hello {} ", sey::sey(String::from("Shakir")));
    println!("Area of circle is {} ", sey::get_circle_area(20f64, 20f64, 90f64));
    sey::unsafe_code();
}
