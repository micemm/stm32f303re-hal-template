#![no_std]
#![no_main]

use core::result::Result;

use cortex_m::asm;
use cortex_m_rt::entry;
use hal::time::MilliSeconds;
use panic_halt as _;
use stm32f3xx_hal::{self as hal, pac, prelude::*};
use heapless::String;
use embedded_hal::serial::Read;

// read from uart (blocking mode, no DMA)
// also some examples for reading strings from the uart interface 
// https://github.com/stm32-rs/stm32f3xx-hal/blob/v0.6.0/examples/serial_dma.rs

const BAUDRATE: u32 = 9600;
const BUFFERSIZE: usize = 100;

enum UART_Error{
      Timeout,
}

fn read_string_until_timeout(delimiter: char, timeout: MilliSeconds, rx: &mut impl Read<u8>, timer: &hal::time::MonoTimer) -> Result<String<BUFFERSIZE>, UART_Error>{
      let starttime = timer.now();
      let max_time = timer.frequency().0 / 1000 * timeout.0; // convert timeout (ms) to ticks 
      
      let mut result = String::<BUFFERSIZE>::new();
      let mut c: char;
      loop{ 
            // check if timeout
            let current_time = starttime.elapsed();
            if current_time > max_time{
                  return Result::Err(UART_Error::Timeout);
            }
            // read character
            let read_result = rx.read();
            if read_result.is_err(){
                  continue;
            }
            else{
                  c = read_result.ok().unwrap() as char;
                  if c == delimiter{
                        break;
                  }
                  else{
                        let append_res = result.push(c);
                        if append_res.is_err(){
                              return Result::Ok(result); // string is full (max. buffer size reached -> return the string)
                        }
                  }
            }
      }
      Result::Ok(result)
}

/// read a string from the provided uart interface. Blocks until the delimiter is reached or the maximum buffer length is reached
fn read_string_until(delimiter: char, rx: &mut impl Read<u8>) -> String<BUFFERSIZE>{
      let mut result = String::<BUFFERSIZE>::new();
      let mut c: char;
      loop{
            let read_result = rx.read();
            if read_result.is_err(){
                  continue;
            }
            else{
                  c = read_result.ok().unwrap() as char;
                  if c == delimiter{
                        break;
                  }
                  else{
                        let append_res = result.push(c);
                        if append_res.is_err(){
                              return result; // string is full (max. buffer size reached -> return the string)
                        }
                  }
            }
      }
      result
}

/// read a line from the provided uart rx interface, blocks until new line ('\n') is received or maximum buffer length is reached
fn read_line(rx: &mut impl Read<u8>) -> String<BUFFERSIZE>{
      let mut result = String::<BUFFERSIZE>::new();
      let mut c: char;
      loop {
            let read = rx.read(); // this can return ok or err, if err, there was no data to read
            if read.is_err(){
                  continue; // continue reading -> wait for data
            }
            else{ // read ok
                  c = read.ok().unwrap() as char;
                  if c == '\n'{
                        break; // new line
                  }
                  else if c != '\r'{ // do not append '\r' symbol
                        let append_res = result.push(c);
                        if append_res.is_err(){
                              return result; // string is full (max. buffer size reached -> string is returned)
                        }
                  }
            }
      }
      result
}

#[entry]
fn main() -> ! {
      let dp = pac::Peripherals::take().unwrap();
      let cp = pac::CorePeripherals::take().unwrap();

      let mut rcc = dp.RCC.constrain();
      let mut flash = dp.FLASH.constrain();

      let clocks = rcc.cfgr.freeze(&mut flash.acr);
      let mut gpioa = dp.GPIOA.split(&mut rcc.ahb);

      // switch tx and rx pin to alternative function
      let pa2_tx = gpioa.pa2.into_af7(&mut gpioa.moder, &mut gpioa.afrl);
      let pa3_rx = gpioa.pa3.into_af7(&mut gpioa.moder, &mut gpioa.afrl);

      // create UART struct
      let uart = hal::serial::Serial::usart2(dp.USART2, (pa2_tx, pa3_rx), BAUDRATE.bps(), clocks, &mut rcc.apb1);
      // get tx and rx structs
      let (mut tx, mut rx) = uart.split();

      let mono_timer = hal::time::MonoTimer::new(cp.DWT, clocks);
      
      loop {
            // echo lines
            tx.bwrite_all("Send text!".as_bytes()).unwrap();
            tx.bwrite_all("\r\n".as_bytes()).unwrap();
            let line = read_string_until_timeout('\r', 10_000.ms(), &mut rx, &mono_timer); // delimiter may also be '\n', depends on the interacting program
            match line{
                  Ok(line) => {
                        tx.bwrite_all("You sent: ".as_bytes()).unwrap();
                        tx.bwrite_all(line.as_bytes()).unwrap();
                  }
                  Err(_error) => {
                        tx.bwrite_all("Too slow, try again...".as_bytes()).unwrap();
                  }
            }
            tx.bwrite_all("\r\n".as_bytes()).unwrap();
      }
}