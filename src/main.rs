const MULTIPLEXER_BITS: u8 = 4;
const MAJOR_COUNT_TO: u8 = 4;
const MINOR_COUNT_TO: u8  = 5;

const GPIO_LED: u8 = 23;


const MULTIPLEXER_PINS_MAJOR_IDS: [u8; 4] = [0, 1, 2, 3];
const MULTIPLEXER_PINS_MINOR_IDS: [u8; 4] = [7, 6, 5, 4];
use rppal::gpio::{Gpio, OutputPin};



fn main() -> Result<(), Box<dyn std::error::Error>> {

    const default_min_time_switch: usize = 68;
    let mut actual_min_time_switch = default_min_time_switch;
    let mut sync_pin = Gpio::new()?.get(GPIO_LED)?.into_output();

    let mut multiplexer_pins_major: Vec<OutputPin> = Vec::with_capacity(MULTIPLEXER_BITS as usize);
    let mut multiplexer_pins_minor: Vec<OutputPin> = Vec::with_capacity(MULTIPLEXER_BITS as usize);

    for i in 0..4 {
        multiplexer_pins_major[i] = Gpio::new()?.get(MULTIPLEXER_PINS_MAJOR_IDS[i])?.into_output();
        multiplexer_pins_minor[i] = Gpio::new()?.get(MULTIPLEXER_PINS_MINOR_IDS[i])?.into_output();
        multiplexer_pins_major[i].set_low();
        multiplexer_pins_minor[i].set_low();
    }


    let mut current_major_state = 0;
    let mut current_minor_state = 0;


    loop {
        current_minor_state+=1;
        if current_minor_state >= MINOR_COUNT_TO {
      
          current_major_state+=1;
      
          if current_major_state > MAJOR_COUNT_TO {
            current_major_state = 0;
            current_minor_state = 0;
            sync_pin.set_low()
      
          } else if current_major_state == MAJOR_COUNT_TO {
            sync_pin.set_high()
          } else {
            current_minor_state = 0;
          }
        }
        if current_major_state != MAJOR_COUNT_TO {
          multiplexer_write_byte(&mut multiplexer_pins_minor, current_minor_state);
          multiplexer_write_byte(&mut multiplexer_pins_major, current_major_state);
        }
    }

}

fn multiplexer_write_byte(multiplexer_pins: &mut Vec<OutputPin>, byte_to_write: u8) {
    for i in 0..MULTIPLEXER_BITS {
        if byte_to_write & (i << i) != 0 { // Bit mask the byte to produce the state of the pin. 
            multiplexer_pins[i as usize].set_high()
        } else {
            multiplexer_pins[i as usize].set_low()
        }

    //   digitalWrite(PINS[i], bitRead(byte_to_write, i));
    }
  }