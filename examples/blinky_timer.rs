#![no_main]
#![no_std]

extern crate panic_itm;

use stm32f407g_disc as board;

use crate::board::{
    hal::stm32,
    hal::{
        delay::Delay,
        interrupt,
        prelude::*,
        rcc::{Clocks, Rcc},
        timer::{Event, Timer},
    },
    led::{LedColor, Leds},
};

use core::cell::{Cell, RefCell};
use core::ops::DerefMut;
use core::sync::atomic::{AtomicBool, AtomicUsize, Ordering};
use cortex_m::interrupt::{free, CriticalSection, Mutex};
use cortex_m::peripheral::Peripherals;

use cortex_m_rt::entry;

static TIMER_TIM2: Mutex<RefCell<Option<Timer<stm32::TIM2>>>> = Mutex::new(RefCell::new(None));
static LAST_LIT_INDEX: AtomicUsize = AtomicUsize::new(0);
static INDEX_CHANGED: AtomicBool = AtomicBool::new(true);

#[entry]
fn main() -> ! {
    if let (Some(p), Some(cp)) = (stm32::Peripherals::take(), Peripherals::take()) {
        p.RCC.apb2enr.write(|w| w.syscfgen().enabled());
        let gpiod = p.GPIOD.split();

        // Initialize on-board LEDs
        let mut leds = Leds::new(gpiod);

        // Constrain clock registers
        let rcc = p.RCC.constrain();

        // Configure clock to 48 MHz and freeze it
        let clocks = rcc
            .cfgr
            .sysclk(48.mhz())
            // .hclk(48.mhz())
            // .pclk1(24.mhz())
            // .pclk2(24.mhz())
            .freeze();

        // Get delay provider
        // let mut delay = Delay::new(cp.SYST, clocks);

        let mut timer = Timer::tim2(p.TIM2, 10.hz(), clocks);
        timer.listen(Event::TimeOut);

        free(|cs| {
            TIMER_TIM2.borrow(cs).replace(Some(timer));
        });

        // Enable interrupt
        stm32::NVIC::unpend(stm32::Interrupt::TIM2);
        unsafe {
            stm32::NVIC::unmask(stm32::Interrupt::TIM2);
        }

        // let period_ms = 50_u16;

        loop {
            // for curr in 0..4 {
            //     let next = (curr + 1) % 4;
            //     // Turn LEDs on one after the other with 500ms delay between them
            //     leds[next].on();
            //     delay(&timer, period_ms);
            //     // delay.delay_ms(period_ms);
            //     leds[curr].off();
            //     delay(&timer, period_ms);
            //     // delay.delay_ms(period_ms);
            // }
            if INDEX_CHANGED.load(Ordering::Relaxed) {
                INDEX_CHANGED.store(false, Ordering::Relaxed);
                let last_index = LAST_LIT_INDEX.load(Ordering::Relaxed);
                let next_index = (last_index + 1) % 4;
                leds[last_index].off();
                leds[next_index].on();
            }
        }
    }

    loop {
        continue;
    }
}

#[interrupt]
fn TIM2() {
    free(|cs| {
        if let Some(ref mut tim2) = TIMER_TIM2.borrow(cs).borrow_mut().deref_mut() {
            tim2.clear_interrupt(Event::TimeOut);
        }

        LAST_LIT_INDEX.store(
            (LAST_LIT_INDEX.load(Ordering::Relaxed) + 1) % 4,
            Ordering::Relaxed,
        );
        INDEX_CHANGED.store(true, Ordering::Relaxed);
        // let cell = ELAPSED_MS.borrow(cs);
        // let val = cell.get();
        // cell.replace(val + 1);
    });
}
