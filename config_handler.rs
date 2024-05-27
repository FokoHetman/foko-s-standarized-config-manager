use std::{
  io,
  fs,
  collections::HashMap,
};
#[derive(Clone, Debug)]
enum ConfigValues {
  Empty,
  string,
  int,
  tuple,
  list,
}
#[derive(Clone, Debug)]
enum ConfigValue {
  Nullus,
  string(String),
  int(i32),
  tuple(Vec<ConfigObject>),
  list(Vec<ConfigObject>),
}


#[derive(Clone, Debug)]
pub struct ConfigObject {
  valuetype: ConfigValues,
  value: ConfigValue,
}
#[derive(Clone, Debug)]
pub struct Config {
  keys: Vec<String>,
  values: Vec<ConfigObject>,
}



impl ConfigObject {
  fn new() -> ConfigObject {
    return ConfigObject{ valuetype: ConfigValues::Empty, value: ConfigValue::Nullus}
  }
  
  fn from(value: String) -> Self {
    if value.starts_with("\"") && value.ends_with("\"") {
      let mut chars = value.chars(); chars.next(); chars.next_back();
      return ConfigObject{valuetype: ConfigValues::string, value: ConfigValue::string(chars.as_str().to_string())};
    }
 
    else if value.starts_with("(") && value.ends_with(")") {
      let mut tuple: Vec<ConfigObject> = [].to_vec();
      let mut chars = value.chars(); chars.next(); chars.next_back();


      for i in chars.as_str().to_string().split(",") {
        tuple.push(ConfigObject::from(fix_string(i.to_string())));
      }

      return ConfigObject{valuetype: ConfigValues::tuple, value: ConfigValue::tuple(tuple)};
    }
    
    else if value.starts_with("[") && value.ends_with("]") {
      let mut vector: Vec<ConfigObject> = [].to_vec();
      let mut chars = value.chars(); chars.next(); chars.next_back();

      for i in chars.as_str().to_string().split(",") {
        vector.push(ConfigObject::from(fix_string(i.to_string())));
      }
      return ConfigObject{valuetype: ConfigValues::list, value: ConfigValue::list(vector)};
    }
    else {
      let mut tmp = value.chars().collect::<Vec<char>>();
      if tmp.len()>0 {
        while tmp[0].is_numeric() && tmp.len()>0 {
          tmp.remove(0);
          if tmp.len()==0 {
            return ConfigObject{valuetype: ConfigValues::int, value: ConfigValue::int(value.parse::<i32>().unwrap())};
          }
        }
      }
      return ConfigObject{valuetype: ConfigValues::Empty, value: ConfigValue::Nullus};
    }
  }
}

impl Config {
  fn new() -> Config {
    return Config {keys: [].to_vec(), values: [].to_vec()};
  }
  fn push(&mut self, key: String, value: ConfigObject) -> &mut Self {
    self.keys.push(key);
    self.values.push(value);
    return self
  }
}





pub fn parse(string: String) -> Config {
  let mut config = Config::new();
  for i in string.split(";") {
    config.push(i.split("=").collect::<Vec<&str>>()[0].to_string().replace("\n","").replace(" ",""),
        ConfigObject::from(fix_string(i.split("=").collect::<Vec<&str>>()[1].to_string())));
  }
  return config
}


fn fix_string(string: String) -> String {
  let mut chars = string.chars().collect::<Vec<char>>();

  while chars[0]==' ' {
    chars.remove(0);
  }
  let mut result = String::new();
  for i in chars {
    result.push_str(&i.to_string())
  }
  return result.replace("\n","");
}
