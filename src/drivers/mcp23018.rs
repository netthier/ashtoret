// https://ww1.microchip.com/downloads/en/devicedoc/22103a.pdf
// Note: Default config is BANK = 0; SEQOP = 0

use embedded_hal_async::i2c::I2c;

pub const MCP23018_DEFAULT_ADDR: u8 = 0b0100000;

#[allow(dead_code)]
#[allow(non_camel_case_types)]
#[derive(Copy, Clone)]
#[repr(u8)]
pub enum Port {
    A = 0x0,
    B = 0x1,
}

#[allow(dead_code)]
#[allow(non_camel_case_types)]
#[derive(Copy, Clone)]
#[repr(u8)]
pub enum Register {
    IODIR = 0x00,
    IPOL = 0x02,
    GPINTEN = 0x04,
    DEFVAL = 0x06,
    INTCON = 0x08,
    IOCON = 0x0A,
    GPPU = 0x0C,
    INTF = 0x0E,
    INTCAP = 0x10,
    GPIO = 0x12,
    OLAT = 0x14,
}

pub struct Mcp23018<I2C> {
    i2c: I2C,
    addr: u8,
}

#[allow(dead_code)]
impl<E, I2C: I2c<Error = E>> Mcp23018<I2C> {
    pub fn new(i2c: I2C) -> Self {
        Self {
            i2c,
            addr: MCP23018_DEFAULT_ADDR,
        }
    }

    pub fn with_addr(mut self, addr: u8) -> Self {
        self.addr = addr & !0x01;
        self
    }

    // 0 -> Output, no pullup
    // 1 -> Input, pullup
    pub async fn config_port(&mut self, port: Port, io_dir: u8, pullup: u8) -> Result<(), E> {
        self.write_reg(Register::IODIR, port, io_dir).await?;
        self.write_reg(Register::GPPU, port, pullup).await
    }

    pub async fn set_port(&mut self, port: Port, data: u8) -> Result<(), E> {
        self.write_reg(Register::GPIO, port, data).await
    }

    pub async fn set_all(&mut self, data: [u8; 2]) -> Result<(), E> {
        self.write_reg_seq(Register::GPIO, data).await
    }

    pub async fn read_port(&mut self, port: Port) -> Result<u8, E> {
        self.read_reg(Register::GPIO, port).await
    }

    async fn write_reg(&mut self, reg: Register, port: Port, data: u8) -> Result<(), E> {
        self.i2c
            .write(self.addr, &[reg as u8 | port as u8, data])
            .await
    }

    async fn write_reg_seq(&mut self, reg: Register, data: [u8; 2]) -> Result<(), E> {
        self.i2c
            .write(self.addr, &[reg as u8, data[0], data[1]])
            .await
    }

    async fn read_reg(&mut self, reg: Register, port: Port) -> Result<u8, E> {
        let mut buf = [0];
        self.i2c
            .write_read(self.addr, &[reg as u8], &mut buf)
            .await?;
        Ok(buf[0])
    }
}
