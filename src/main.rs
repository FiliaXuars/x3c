use hex::FromHex;

pub struct NewComputer
{
    pub power:              bool,
    pub memory:             [u128; 16],
    pub current_bank:       u8,
    pub program_position:   u8,

    pub buffer:             u8,
}

impl NewComputer
{
    pub fn memory_access( &mut self, address: u8, write: bool, value: u8) -> u8
    {

        let mut result: [u8; 16] = self.memory[((address & 0xf0) >> 4) as usize].to_be_bytes();
        if write
        {
            result[(address & 0x0f) as usize] = value;
            self.memory[((address & 0xf0) >> 4) as usize] = u128::from_be_bytes(result);
        }
        result[(address & 0x0f) as usize]
    }

    pub fn processor_instructions( &mut self, opcode: u8, address: u8 )
    {
        if address > 0xf || opcode > 0xf
        {
            print!("  Attempt to address memory or opcode out of range\n");
        }

        match opcode
        {
            0x0 =>
                // noop
                (),
            0x1 =>
                // jump
                self.program_position = address.wrapping_sub(0x1),
            0x2 =>
                // skip
                if self.memory_access(address, false, 0x0) == 0xff
                {
                    self.program_position = self.program_position.saturating_add(0x1);
                },
            0x3 =>
                // take
                self.buffer = self.memory_access(address, false, 0x0),
            0x4 =>
                // place
                _ = self.memory_access(address, true, self.buffer),
            0x5 =>
                // bank up
                {
                    self.current_bank = self.current_bank.saturating_add(0x1);
                    self.program_position = self.program_position.wrapping_sub(0x1);
                },

            0x6 =>
                // bank down
                {
                    self.current_bank = self.current_bank.saturating_sub(0x1);
                    self.program_position = self.program_position.wrapping_sub(0x1);
                },
            0x7 =>
                // and
                {
                    let read = self.memory_access(address, false, 0x0);
                    self.buffer = self.buffer & read;
                },
            0x8 =>
                // or
                {
                    let read = self.memory_access(address, false, 0x0);
                    self.buffer = self.buffer | read;
                },
            0x9 =>
                // xor
                {
                    let read = self.memory_access(address, false, 0x0);
                    self.buffer = self.buffer ^ read;
                },
            0xa =>
                // nor
                {
                    let read = self.memory_access(address, false, 0x0);
                    self.buffer = !self.buffer & !read;
                },
            0xb =>
                // save
                {
                    let file = std::fs::File::create("store_".to_string() + &hex::encode([address]) + ".mem");
                    if file.is_ok()
                    {
                        let mut file = file.unwrap();
                        let _ = std::io::Write::write_all(&mut file, &self.memory[self.current_bank as usize].to_be_bytes());
                        return;
                    }
                    print!("    failed to save!\n");
                },
            0xc =>
                // load
                {
                    let file = std::fs::read_to_string("store_".to_string() + &hex::encode([self.current_bank]) + ".mem");
                    if file.is_ok()
                    {
                        let file = hex::decode(file.unwrap());
                        if file.is_ok()
                        {
                            let mut unpacked_file = [0x0; 16];
                            let file = file.unwrap();

                            for iteration in 0x0..0xf
                            {
                                unpacked_file[iteration] = file[iteration];
                            }
                            self.memory[address as usize] = u128::from_be_bytes(unpacked_file);
                        }
                    }
                    print!("    failed to load!\n")
                },
            0xd =>
                // add
                {
                    let read = self.memory_access(address, false, 0x0);
                    self.buffer = self.buffer.wrapping_add(read);
                },
            0xe =>
                // subtract
                {
                    let read = self.memory_access(address, false, 0x0);
                    self.buffer = self.buffer.wrapping_sub(read);
                },
            0xf =>
                // display
                {
                    if cfg!(unix)
                    {
                        println!("   {}", hex::encode(vec![self.memory_access(address, false, 0x0)]).as_str())
                    }
                    if cfg!(windows)
                    {
                        println!("   {}", hex::encode(vec![self.memory_access(address, false, 0x0)]).as_str())
                    }
                },
            0x10..=0xff =>
                print!("    instruction out of bounds\n")

        }
    }

}

fn main() {
    let mut computer = NewComputer
    {
        power:              true,
        memory:             [0x0; 16],
        current_bank:       0x0,
        program_position:   0x0,

        buffer:             0x0,

    };

    computer.memory[0] = 0x0030e010000000000000000000000000;
    for iteration in 1..=0xf
    {
        computer.memory[iteration] = 0x60000000000000000000000000000000
    }

    let (input_tx, input_rx) = std::sync::mpsc::channel();
    let (input_receieved_tx, input_recieved_rx) = std::sync::mpsc::channel();
    let _ = std::thread::spawn(move ||
    {
        while computer.power
        {
            let mut input_buffer = String::new();
            let stdin = std::io::stdin();
            if cfg!(windows)
            {
                print!("\n");
            }
            let _ = stdin.read_line(&mut input_buffer).unwrap();
            let buffer_length = input_buffer.len();
            if  buffer_length >= 5 && buffer_length <= 6
            {
                if cfg!(windows)
                {
                    let _ = input_buffer.remove(4);
                    let _ = input_buffer.remove(4);
                }
                else if cfg!(unix)
                {
                    let _ = input_buffer.remove(4);
                }

                let input = <[u8; 2]>::from_hex(input_buffer);
                if input.is_ok()
                {
                    let input = input.unwrap();
                    if input.len() == 2
                    {
                        while input_recieved_rx.recv_timeout(std::time::Duration::from_millis(1)).is_err()
                        {
                            input_tx.send(input).unwrap()
                        }
        
                    }
                }
                
            }
        }
    });
    
    while computer.power
    {
        if computer.program_position == 0
        {
            let input = input_rx.recv_timeout(std::time::Duration::from_millis(1));
            if input.is_ok()
            {
                input_receieved_tx.send(true).unwrap();
                let input = input.unwrap();
                computer.memory_access(input[0], true, input[1]);
            }
        }
        if computer.current_bank == 16
        {
            computer.current_bank = 0;
        }
        let read = computer.memory_access(
            ((computer.current_bank & 0x0f) << 4) + (computer.program_position)
            , false, 0x0);
        let opcode = (read & 0xf0) >> 4;
        let address = read & 0x0f;
        computer.processor_instructions(opcode, address);
        if computer.program_position == 15
        {
            computer.program_position = 0;
            computer.current_bank = computer.current_bank.saturating_add(0x01);
        }
        else 
        {
            computer.program_position = computer.program_position.wrapping_add(0x1);
        }

    }
}
