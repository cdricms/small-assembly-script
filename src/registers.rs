macro_rules! impl_register {
    ($name: ident) => {
        pub struct $name {
            value: u8
        }    

        impl $name {
            pub fn new() -> Self {
                Self { value: 0 }
            }

            pub fn store(&mut self, value: u8) {
                self.value = value;
            }

            pub fn retrieve(&self) -> u8{
                self.value
            }

            pub fn erase(&mut self) {
                self.store(0);
            }
        }
    };
}

impl_register!(ARegister);
impl_register!(BRegister);
impl_register!(EqualityRegister);
impl_register!(MainRegister);
