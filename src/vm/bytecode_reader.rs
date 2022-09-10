use std::cell::RefCell;
use std::rc::Rc;

struct BytecodeReader{
    code:Rc<Vec<u8>>,
    pc: RefCell<usize>,
}

impl BytecodeReader{
    fn new(code:Vec<u8>)->Self{
        Self{
            code: Rc::new(code),
            pc: RefCell::new(0)
        }
    }
    fn from(code: Rc<Vec<u8>>) ->Self {
        Self{
            code,
            pc:RefCell::new(0)
        }
    }
    fn peek(&self, pc:usize) {
        *self.pc.borrow_mut() = pc;
    }
    fn read_u8(&self) -> u8 {
        let mut pc = self.pc.borrow_mut();
        let b = self.code[*pc];
        *pc+=1;
        b
    }
    fn read_i8(&self) -> i8{
        self.read_u8().try_into().unwrap()
    }
}