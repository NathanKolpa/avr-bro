//! The ATmega328P microcontroller.
//!
//! ## Additional resources:
//! - [Atmel-7810-Automotive-Microcontrollers-ATmega328P_Datasheet.pdf](https://ww1.microchip.com/downloads/en/DeviceDoc/Atmel-7810-Automotive-Microcontrollers-ATmega328P_Datasheet.pdf)

pub mod registers {
    //! Every documented register.

    use crate::register::{Register, RegisterAddress};

    /// The Port B Input Pins Address
    pub const PINB: Register = Register::new(0x24 as RegisterAddress);

    /// The Port B Data Direction Register
    pub const DDRB: Register = Register::new(0x24 as RegisterAddress);

    /// The Port B Data Register
    pub const PORTB: Register = Register::new(0x25 as RegisterAddress);

    /// The Port C Input Pins Address
    pub const PINC: Register = Register::new(0x26 as RegisterAddress);

    /// The Port C Data Direction Register
    pub const DDRC: Register = Register::new(0x27 as RegisterAddress);

    /// The Port C Data Register
    pub const PORTC: Register = Register::new(0x28 as RegisterAddress);

    /// The Port D Input Pins Address
    pub const PIND: Register = Register::new(0x29 as RegisterAddress);

    /// The Port D Data Direction Register
    pub const DDRD: Register = Register::new(0x2A as RegisterAddress);

    /// The Port D Data Register
    pub const PORTD: Register = Register::new(0x2B as RegisterAddress);

    /// Timer/Counter 0 Interrupt Flag Register
    pub const TIFR0: Register = Register::new(0x35 as RegisterAddress);

    /// Timer/Counter1 Interrupt Flag Register
    pub const TIFR1: Register = Register::new(0x36 as RegisterAddress);

    /// Timer/Counter2 Interrupt Flag Register
    pub const TIFR2: Register = Register::new(0x37 as RegisterAddress);

    /// Pin Change Interrupt Flag Register
    pub const PCIFR: Register = Register::new(0x3B as RegisterAddress);

    /// External Interrupt Flag Register
    pub const EIFR: Register = Register::new(0x3C as RegisterAddress);

    /// External Interrupt Mask Register
    pub const EIMSK: Register = Register::new(0x3D as RegisterAddress);

    /// General Purpose I/O Register 0
    pub const GPIOR0: Register = Register::new(0x3E as RegisterAddress);

    /// The EEPROM Control Register0
    pub const EECR: Register = Register::new(0x3F as RegisterAddress);

    /// EEPROM data register
    pub const EEDR: Register = Register::new(0x40 as RegisterAddress);

    /// EEPROM address register low byte
    pub const EEARL: Register = Register::new(0x41 as RegisterAddress);

    /// EEPROM address register high byte
    pub const EEARH: Register = Register::new(0x42 as RegisterAddress);

    /// General Timer/Counter Control Register
    pub const GTCCR: Register = Register::new(0x43 as RegisterAddress);

    /// Timer/Counter Control Register A
    pub const TCCR0A: Register = Register::new(0x44 as RegisterAddress);

    /// Timer/Counter Control Register B
    pub const TCCR0B: Register = Register::new(0x45 as RegisterAddress);

    /// Timer/Counter0 (8-bit)
    pub const TCNT0: Register = Register::new(0x46 as RegisterAddress);

    /// Timer/Counter0 output compare register A
    pub const OCR0A: Register = Register::new(0x47 as RegisterAddress);

    /// Timer/Counter0 output compare register B
    pub const OCR0B: Register = Register::new(0x48 as RegisterAddress);

    /// General Purpose I/O Register 1
    pub const GPIOR1: Register = Register::new(0x4A as RegisterAddress);

    /// SPI Control Register
    pub const SPCR: Register = Register::new(0x4C as RegisterAddress);

    /// SPI Status Register
    pub const SPSR: Register = Register::new(0x4D as RegisterAddress);

    /// SPI Data Register
    pub const SPDR: Register = Register::new(0x4E as RegisterAddress);

    /// Analog Comparator Control and Status Register
    pub const ACSR: Register = Register::new(0x50 as RegisterAddress);

    /// Sleep Mode Control Register
    pub const SMCR: Register = Register::new(0x53 as RegisterAddress);

    /// MCU Status Register
    pub const MCUSR: Register = Register::new(0x54 as RegisterAddress);

    /// MCU Control Register
    pub const MCUCR: Register = Register::new(0x55 as RegisterAddress);

    ///  Store Program Memory Control and Status Register
    pub const SPMCSR: Register = Register::new(0x57 as RegisterAddress);

    /// Stack Pointer Low Register
    pub const SPL: Register = Register::new(0x5D as RegisterAddress);

    /// Stack Pointer High Register
    pub const SPH: Register = Register::new(0x5E as RegisterAddress);

    /// Watchdog Timer Control Register
    pub const WDTCSR: Register = Register::new(0x60 as RegisterAddress);

    /// Clock Prescale Register
    pub const CLKPR: Register = Register::new(0x61 as RegisterAddress);

    /// Power Reduction Register
    pub const PRR: Register = Register::new(0x64 as RegisterAddress);

    /// Oscillator calibration register
    pub const OSCCAL: Register = Register::new(0x66 as RegisterAddress);

    /// Pin Change Interrupt Control Register
    pub const PCICR: Register = Register::new(0x68 as RegisterAddress);

    /// External Interrupt Control Register A
    pub const EICRA: Register = Register::new(0x69 as RegisterAddress);

    /// Pin Change Mask Register 0
    pub const PCMSK0: Register = Register::new(0x6B as RegisterAddress);

    /// Pin Change Mask Register 1
    pub const PCMSK1: Register = Register::new(0x6C as RegisterAddress);

    /// Pin Change Mask Register 2
    pub const PCMSK2: Register = Register::new(0x6D as RegisterAddress);

    /// Timer/Counter Interrupt Mask Register
    pub const TIMSK0: Register = Register::new(0x6E as RegisterAddress);

    /// Timer/Counter2 Interrupt Mask Register
    pub const TIMSK2: Register = Register::new(0x70 as RegisterAddress);

    /// ADC data register low byte
    pub const ADCL: Register = Register::new(0x78 as RegisterAddress);

    /// ADC data register high byte
    pub const ADCH: Register = Register::new(0x79 as RegisterAddress);

    /// ADC Control and Status Register A
    pub const ADCSRA: Register = Register::new(0x7A as RegisterAddress);

    /// ADC Control and Status Register B
    pub const ADCSRB: Register = Register::new(0x7B as RegisterAddress);

    /// ADC Multiplexer Selection Register
    pub const ADMUX: Register = Register::new(0x7C as RegisterAddress);

    /// Digital Input Disable Register 0
    pub const DIDR0: Register = Register::new(0x7E as RegisterAddress);

    /// Digital Input Disable Register 1
    pub const DIDR1: Register = Register::new(0x7F as RegisterAddress);

    /// Timer/Counter1 Control Register A
    pub const TCCR1A: Register = Register::new(0x80 as RegisterAddress);

    /// Timer/Counter1 Control Register B
    pub const TCCR1B: Register = Register::new(0x81 as RegisterAddress);

    /// Timer/Counter1 Control Register C
    pub const TCCR1C: Register = Register::new(0x82 as RegisterAddress);

    /// Timer/Counter1 - Counter register low byte
    pub const TCNT1L: Register = Register::new(0x84 as RegisterAddress);

    /// Timer/Counter1 - Counter register high byte
    pub const TCNT1H: Register = Register::new(0x85 as RegisterAddress);

    /// Timer/Counter1 - Input capture register low byte
    pub const ICR1L: Register = Register::new(0x86 as RegisterAddress);

    /// Timer/Counter1 - Input capture register high byte
    pub const ICR1H: Register = Register::new(0x87 as RegisterAddress);

    /// Timer/Counter1 - Output compare register A low byte
    pub const OCR1AL: Register = Register::new(0x88 as RegisterAddress);

    /// Timer/Counter1 - Output compare register A high byte
    pub const OCR1AH: Register = Register::new(0x89 as RegisterAddress);

    /// Timer/Counter1 - Output compare register B low byte
    pub const OCR1BL: Register = Register::new(0x8A as RegisterAddress);

    /// Timer/Counter1 - Output compare register B high byte
    pub const OCR1BH: Register = Register::new(0x8B as RegisterAddress);

    /// Timer/Counter Control Register A
    pub const TCCR2A: Register = Register::new(0xB0 as RegisterAddress);

    /// Timer/Counter Control Register B
    pub const TCCR2B: Register = Register::new(0xB1 as RegisterAddress);

    /// Timer/Counter2 (8-bit)
    pub const TCNT2: Register = Register::new(0xB2 as RegisterAddress);

    /// Timer/Counter2 output compare register A
    pub const OCR2A: Register = Register::new(0xB3 as RegisterAddress);

    /// Timer/Counter2 output compare register B
    pub const OCR2B: Register = Register::new(0xB4 as RegisterAddress);

    /// Asynchronous Status Register
    pub const ASSR: Register = Register::new(0xB6 as RegisterAddress);

    /// 2-wire serial interface bit rate register
    pub const TWBR: Register = Register::new(0xB8 as RegisterAddress);

    /// TWI Status Register
    pub const TWSR: Register = Register::new(0xB9 as RegisterAddress);

    /// TWI (Slave) Address Register
    pub const TWAR: Register = Register::new(0xBA as RegisterAddress);

    /// 2-wire serial interface data register
    pub const TWDR: Register = Register::new(0xBB as RegisterAddress);

    /// TWI Control Register
    pub const TWCR: Register = Register::new(0xBC as RegisterAddress);

    /// TWI (Slave) Address Mask Register
    pub const TWAMR: Register = Register::new(0xBD as RegisterAddress);

    /// USART Control and Status Register n A
    pub const UCSR0A: Register = Register::new(0xC0 as RegisterAddress);

    /// USART Control and Status Register n B
    pub const UCSR0B: Register = Register::new(0xC1 as RegisterAddress);

    /// USART Control and Status Register n C
    pub const UCSR0C: Register = Register::new(0xC2 as RegisterAddress);

    /// USART baud rate register low
    pub const UBRR0L: Register = Register::new(0xC4 as RegisterAddress);

    /// USART baud rate register high
    pub const UBRR0H: Register = Register::new(0xC5 as RegisterAddress);

    /// USART I/O data register
    pub const UDR0: Register = Register::new(0xC6 as RegisterAddress);
}
