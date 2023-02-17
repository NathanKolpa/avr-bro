//! The ATmega328P microcontroller.
//!
//! ## Additional resources:
//! - [Atmel-7810-Automotive-Microcontrollers-ATmega328P_Datasheet.pdf](https://ww1.microchip.com/downloads/en/DeviceDoc/Atmel-7810-Automotive-Microcontrollers-ATmega328P_Datasheet.pdf)

pub mod registers {
    //! Every documented register.

    /// The Port B Input Pins Address
    pub const PINB: *mut u8 = 0x24 as *mut u8;

    /// The Port B Data Direction Register
    pub const DDRB: *mut u8 = 0x24 as *mut u8;

    /// The Port B Data Register
    pub const PORTB: *mut u8 = 0x25 as *mut u8;

    /// The Port C Input Pins Address
    pub const PINC: *mut u8 = 0x26 as *mut u8;

    /// The Port C Data Direction Register
    pub const DDRC: *mut u8 = 0x27 as *mut u8;

    /// The Port C Data Register
    pub const PORTC: *mut u8 = 0x28 as *mut u8;

    /// The Port D Input Pins Address
    pub const PIND: *mut u8 = 0x29 as *mut u8;

    /// The Port D Data Direction Register
    pub const DDRD: *mut u8 = 0x2A as *mut u8;

    /// The Port D Data Register
    pub const PORTD: *mut u8 = 0x2B as *mut u8;

    /// Timer/Counter 0 Interrupt Flag Register
    pub const TIFR0: *mut u8 = 0x35 as *mut u8;

    /// Timer/Counter1 Interrupt Flag Register
    pub const TIFR1: *mut u8 = 0x36 as *mut u8;

    /// Timer/Counter2 Interrupt Flag Register
    pub const TIFR2: *mut u8 = 0x37 as *mut u8;

    /// Pin Change Interrupt Flag Register
    pub const PCIFR: *mut u8 = 0x3B as *mut u8;

    /// External Interrupt Flag Register
    pub const EIFR: *mut u8 = 0x3C as *mut u8;

    /// External Interrupt Mask Register
    pub const EIMSK: *mut u8 = 0x3D as *mut u8;

    /// General Purpose I/O Register 0
    pub const GPIOR0: *mut u8 = 0x3E as *mut u8;

    /// The EEPROM Control Register0
    pub const EECR: *mut u8 = 0x3F as *mut u8;

    /// EEPROM data register
    pub const EEDR: *mut u8 = 0x40 as *mut u8;

    /// EEPROM address register low byte
    pub const EEARL: *mut u8 = 0x41 as *mut u8;

    /// EEPROM address register high byte
    pub const EEARH: *mut u8 = 0x42 as *mut u8;

    /// General Timer/Counter Control Register
    pub const GTCCR: *mut u8 = 0x43 as *mut u8;

    /// Timer/Counter Control Register A
    pub const TCCR0A: *mut u8 = 0x44 as *mut u8;

    /// Timer/Counter Control Register B
    pub const TCCR0B: *mut u8 = 0x45 as *mut u8;

    /// Timer/Counter0 (8-bit)
    pub const TCNT0: *mut u8 = 0x46 as *mut u8;

    /// Timer/Counter0 output compare register A
    pub const OCR0A: *mut u8 = 0x47 as *mut u8;

    /// Timer/Counter0 output compare register B
    pub const OCR0B: *mut u8 = 0x48 as *mut u8;

    /// General Purpose I/O Register 1
    pub const GPIOR1: *mut u8 = 0x4A as *mut u8;

    /// SPI Control Register
    pub const SPCR: *mut u8 = 0x4C as *mut u8;

    /// SPI Status Register
    pub const SPSR: *mut u8 = 0x4D as *mut u8;

    /// SPI Data Register
    pub const SPDR: *mut u8 = 0x4E as *mut u8;

    /// Analog Comparator Control and Status Register
    pub const ACSR: *mut u8 = 0x50 as *mut u8;

    /// Sleep Mode Control Register
    pub const SMCR: *mut u8 = 0x53 as *mut u8;

    /// MCU Status Register
    pub const MCUSR: *mut u8 = 0x54 as *mut u8;

    /// MCU Control Register
    pub const MCUCR: *mut u8 = 0x55 as *mut u8;

    ///  Store Program Memory Control and Status Register
    pub const SPMCSR: *mut u8 = 0x57 as *mut u8;

    /// Stack Pointer Low Register
    pub const SPL: *mut u8 = 0x5D as *mut u8;

    /// Stack Pointer High Register
    pub const SPH: *mut u8 = 0x5E as *mut u8;

    /// Watchdog Timer Control Register
    pub const WDTCSR: *mut u8 = 0x60 as *mut u8;

    /// Clock Prescale Register
    pub const CLKPR: *mut u8 = 0x61 as *mut u8;

    /// Power Reduction Register
    pub const PRR: *mut u8 = 0x64 as *mut u8;

    /// Oscillator calibration register
    pub const OSCCAL: *mut u8 = 0x66 as *mut u8;

    /// Pin Change Interrupt Control Register
    pub const PCICR: *mut u8 = 0x68 as *mut u8;

    /// External Interrupt Control Register A
    pub const EICRA: *mut u8 = 0x69 as *mut u8;

    /// Pin Change Mask Register 0
    pub const PCMSK0: *mut u8 = 0x6B as *mut u8;

    /// Pin Change Mask Register 1
    pub const PCMSK1: *mut u8 = 0x6C as *mut u8;

    /// Pin Change Mask Register 2
    pub const PCMSK2: *mut u8 = 0x6D as *mut u8;

    /// Timer/Counter Interrupt Mask Register
    pub const TIMSK0: *mut u8 = 0x6E as *mut u8;

    /// Timer/Counter2 Interrupt Mask Register
    pub const TIMSK2: *mut u8 = 0x70 as *mut u8;

    /// ADC data register low byte
    pub const ADCL: *mut u8 = 0x78 as *mut u8;

    /// ADC data register high byte
    pub const ADCH: *mut u8 = 0x79 as *mut u8;

    /// ADC Control and Status Register A
    pub const ADCSRA: *mut u8 = 0x7A as *mut u8;

    /// ADC Control and Status Register B
    pub const ADCSRB: *mut u8 = 0x7B as *mut u8;

    /// ADC Multiplexer Selection Register
    pub const ADMUX: *mut u8 = 0x7C as *mut u8;

    /// Digital Input Disable Register 0
    pub const DIDR0: *mut u8 = 0x7E as *mut u8;

    /// Digital Input Disable Register 1
    pub const DIDR1: *mut u8 = 0x7F as *mut u8;

    /// Timer/Counter1 Control Register A
    pub const TCCR1A: *mut u8 = 0x80 as *mut u8;

    /// Timer/Counter1 Control Register B
    pub const TCCR1B: *mut u8 = 0x81 as *mut u8;

    /// Timer/Counter1 Control Register C
    pub const TCCR1C: *mut u8 = 0x82 as *mut u8;

    /// Timer/Counter1 - Counter register low byte
    pub const TCNT1L: *mut u8 = 0x84 as *mut u8;

    /// Timer/Counter1 - Counter register high byte
    pub const TCNT1H: *mut u8 = 0x85 as *mut u8;

    /// Timer/Counter1 - Input capture register low byte
    pub const ICR1L: *mut u8 = 0x86 as *mut u8;

    /// Timer/Counter1 - Input capture register high byte
    pub const ICR1H: *mut u8 = 0x87 as *mut u8;

    /// Timer/Counter1 - Output compare register A low byte
    pub const OCR1AL: *mut u8 = 0x88 as *mut u8;

    /// Timer/Counter1 - Output compare register A high byte
    pub const OCR1AH: *mut u8 = 0x89 as *mut u8;

    /// Timer/Counter1 - Output compare register B low byte
    pub const OCR1BL: *mut u8 = 0x8A as *mut u8;

    /// Timer/Counter1 - Output compare register B high byte
    pub const OCR1BH: *mut u8 = 0x8B as *mut u8;

    /// Timer/Counter Control Register A
    pub const TCCR2A: *mut u8 = 0xB0 as *mut u8;

    /// Timer/Counter Control Register B
    pub const TCCR2B: *mut u8 = 0xB1 as *mut u8;

    /// Timer/Counter2 (8-bit)
    pub const TCNT2: *mut u8 = 0xB2 as *mut u8;

    /// Timer/Counter2 output compare register A
    pub const OCR2A: *mut u8 = 0xB3 as *mut u8;

    /// Timer/Counter2 output compare register B
    pub const OCR2B: *mut u8 = 0xB4 as *mut u8;

    /// Asynchronous Status Register
    pub const ASSR: *mut u8 = 0xB6 as *mut u8;

    /// 2-wire serial interface bit rate register
    pub const TWBR: *mut u8 = 0xB8 as *mut u8;

    /// TWI Status Register
    pub const TWSR: *mut u8 = 0xB9 as *mut u8;

    /// TWI (Slave) Address Register
    pub const TWAR: *mut u8 = 0xBA as *mut u8;

    /// 2-wire serial interface data register
    pub const TWDR: *mut u8 = 0xBB as *mut u8;

    /// TWI Control Register
    pub const TWCR: *mut u8 = 0xBC as *mut u8;

    /// TWI (Slave) Address Mask Register
    pub const TWAMR: *mut u8 = 0xBD as *mut u8;

    /// USART Control and Status Register n A
    pub const UCSR0A: *mut u8 = 0xC0 as *mut u8;

    /// USART Control and Status Register n B
    pub const UCSR0B: *mut u8 = 0xC1 as *mut u8;

    /// USART Control and Status Register n C
    pub const UCSR0C: *mut u8 = 0xC2 as *mut u8;

    /// USART baud rate register low
    pub const UBRR0L: *mut u8 = 0xC4 as *mut u8;

    /// USART baud rate register high
    pub const UBRR0H: *mut u8 = 0xC5 as *mut u8;

    /// USART I/O data register
    pub const UDR0: *mut u8 = 0xC6 as *mut u8;
}
