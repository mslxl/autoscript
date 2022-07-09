use std::{
    borrow::Borrow, cell::RefCell, collections::HashMap, env, hash::Hash,
    rc::Rc,
};

struct RcStr(Rc<String>);

impl PartialEq for RcStr {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl Eq for RcStr {}

impl Hash for RcStr {
    fn hash<H: std::hash::Hasher>(&self, state: &mut H) {
        self.0.hash(state);
    }
}

impl Borrow<String> for RcStr {
    fn borrow(&self) -> &String {
        &self.0
    }
}

impl Borrow<str> for RcStr {
    fn borrow(&self) -> &str {
        &self.0
    }
}

#[derive(PartialEq, Eq, Debug)]
pub enum CmdReceiveType {
    None,
    Bool,
    Text,
    Number,
    Path,
}

pub enum CmdReceiveValue {
    Bool(bool),
    Text(String),
    Number(i32),
    Path(String),
}

pub struct CmdOption {
    name: Rc<String>,
    shortcut: Option<Rc<String>>,
    desc: String,
    value_type: CmdReceiveType,
    def: Option<CmdReceiveValue>,
}

impl CmdOption {
    pub fn new(
        name: String,
        shortcut: Option<String>,
        desc: String,
        value_type: CmdReceiveType,
        default: Option<CmdReceiveValue>,
    ) -> Self {
        CmdOption {
            name: Rc::new(name),
            shortcut: shortcut.map(|x| Rc::new(x)),
            desc: desc,
            value_type: value_type,
            def: default,
        }
    }
}

pub struct CmdParser {
    opts: Vec<Rc<CmdOption>>,
    opt_map: HashMap<RcStr, Rc<CmdOption>>,
    max_name_len: usize,
    value: RefCell<HashMap<RcStr, CmdReceiveValue>>,
    suffix: RefCell<Vec<String>>,
}

impl CmdParser {
    pub fn new() -> Self {
        CmdParser {
            opts: Vec::new(),
            opt_map: HashMap::new(),
            max_name_len: 0,
            value: RefCell::new(HashMap::new()),
            suffix: RefCell::new(Vec::new()),
        }
    }
    /// Add a specific option to parser
    pub fn option(&mut self, opt: CmdOption) {
        self.opts.push(Rc::new(opt));
        let ref_opt = self.opts.last().unwrap();
        self.opt_map
            .insert(RcStr(Rc::clone(&ref_opt.name)), Rc::clone(ref_opt));
        match ref_opt.shortcut {
            Some(ref sc) => {
                self.opt_map
                    .insert(RcStr(Rc::clone(sc)), Rc::clone(ref_opt));
                ()
            }
            _ => (),
        }
        self.max_name_len = self.max_name_len.max(ref_opt.name.len())
    }

    /// generate help text by options
    pub fn help_str(&self) -> String {
        let mut help = String::from(format!(
            "Usage: {} [OPTIONS]... <FILE>\n",
            env::current_exe()
                .unwrap()
                .file_name()
                .unwrap()
                .to_str()
                .unwrap()
        ));

        help.push_str("\nMandatory arguments to long options are mandatory for short options too.");
        for opt in &self.opts {
            help.push_str("\n  ");
            match opt.shortcut {
                Some(ref shortcut) => help.push_str(&format!("-{}, ", shortcut)),
                _ => help.push_str("    "),
            }
            help.push_str(&format!(
                "--{:pad$}\t",
                opt.name,
                pad = self.max_name_len + 2
            ));
            help.push_str(&opt.desc)
        }
        help
    }

    /// insert value to &self.value after value's type be checked
    /// 
    /// it should only be called by parse method
    fn insert_value(
        &self,
        opt: &Rc<CmdOption>,
        value: &mut impl Iterator<Item = String>,
    ) -> Result<(), String> {
        let mut map = self.value.borrow_mut();

        // type is None, deal it first since it needn't use next args
        if opt.value_type == CmdReceiveType::None {
            map.insert(RcStr(Rc::clone(&opt.name)), CmdReceiveValue::Bool(true));
            return Ok(());
        }
        let value = value.next();
        if value == None {
            return Err(format!(
                "{} expect a {:?} value",
                &opt.name, &opt.value_type
            ));
        }
        let value = value.unwrap();

        // check type and insert
        match opt.value_type {
            CmdReceiveType::Bool => {
                let v: bool = value
                    .parse()
                    .map_err(|_| format!("can't convert {} to bool", value))?;
                map.insert(RcStr(Rc::clone(&opt.name)), CmdReceiveValue::Bool(v));
                Ok(())
            }
            CmdReceiveType::Number => {
                let v: i32 = value
                    .parse()
                    .map_err(|_| format!("can't convert {} to i32", value))?;
                map.insert(RcStr(Rc::clone(&opt.name)), CmdReceiveValue::Number(v));
                Ok(())
            }
            CmdReceiveType::Text => {
                map.insert(
                    RcStr(Rc::clone(&opt.name)),
                    CmdReceiveValue::Text(value.to_string()),
                );
                Ok(())
            }
            CmdReceiveType::Path => {
                map.insert(
                    RcStr(Rc::clone(&opt.name)),
                    CmdReceiveValue::Text(value.to_string()),
                );
                Ok(())
            }
            _ => panic!("Not supported"), // it never happed normally
        }
    }

    pub fn parse(&self, mut args: impl Iterator<Item = String>) -> Result<(), String> {
        let key = args.next();
        if key == None {
            return Ok(());
        }
        let input = key.unwrap();
        if input.starts_with("-") {
            match self.opt_map.get(remove_key_prefix(&input)) {
                Some(opt) => {
                    self.insert_value(opt, &mut args)?;
                    self.parse(args)
                }
                None => Err(format!("Found argument '{}' which wasn't expected", input)),
            }
        }else{
            self.suffix.borrow_mut().push(input);
            self.parse(args)
        }
    }

    pub fn is_empty(&self) -> bool {
        self.value.borrow().is_empty() && self.suffix.borrow().is_empty()
    }

    fn get_def(&self, k: &str) -> Option<&CmdReceiveValue> {
        match self.opt_map.get(k) {
            None => None,
            Some(v) => match v.def {
                None => None,
                Some(ref x) => Some(x),
            },
        }
    }

    pub fn get_bool(&self, k: &str) -> Option<bool> {
        let input = self
            .value
            .borrow()
            .get(k)
            .or(self.get_def(k))
            .map(|x| match x {
                CmdReceiveValue::Bool(b) => *b,
                _ => panic!("Type didn't match!"),
            });
        input
    }

    pub fn get_suffix(&self)->Vec<String>{
        self.suffix.borrow().clone()
    }
}
fn remove_key_prefix(k: &str) -> &str {
    if k.starts_with("--") {
        &k[2..]
    } else if k.starts_with("-") {
        &k[1..]
    } else {
        k
    }
}
