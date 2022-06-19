#![deny(unsafe_code)]
#![no_std]
#![no_main]

use panic_halt as _;

mod glyph;
mod vk16k33;

#[rtic::app(device=stm32f1xx_hal::pac, peripherals = true, )]
mod app {

    use stm32f1xx_hal::{
        gpio::{self, Alternate, Edge, ExtiPin, Input, OpenDrain, Output, Pin, PullDown, PushPull},
        i2c::{BlockingI2c, Mode},
        pac::{self, I2C1},
        prelude::*,
        timer::{CounterMs, Event},
    };

    use crate::glyph::{Glyph, Capitals};
    use crate::vk16k33;

    type ODPB6 = Pin<'B', 6, false, Alternate<OpenDrain>>;
    type ODPB7 = Pin<'B', 7, false, Alternate<OpenDrain>>;

    #[shared]
    struct Shared {
        disp: vk16k33::DisplayBuff,
        i2c: BlockingI2c<I2C1, (ODPB6, ODPB7)>,
        led: gpio::PB1<Output<PushPull>>,
        led_state: bool,
    }

    #[local]
    struct Local {
        timer: CounterMs<pac::TIM1>,
        touch: gpio::PA10<Input<PullDown>>,
    }

    #[init]
    fn init(mut cx: init::Context) -> (Shared, Local, init::Monotonics) {
        let mut flash = cx.device.FLASH.constrain();
        let rcc = cx.device.RCC.constrain();

        let clocks = rcc.cfgr.sysclk(48.MHz()).freeze(&mut flash.acr);
        // let mut sys_cfg = cx.device.
        // let mut nvic = cx.core.NVIC.stir.

        cx.core.DCB.enable_trace();
        cx.core.DWT.enable_cycle_counter();

        let mut gpb = cx.device.GPIOB.split();
        let mut gpa = cx.device.GPIOA.split();

        let mut touch = gpa.pa10.into_pull_down_input(&mut gpa.crh);

        let mut afio = cx.device.AFIO.constrain();
        let scl1 = gpb.pb6.into_alternate_open_drain(&mut gpb.crl);
        let sda1 = gpb.pb7.into_alternate_open_drain(&mut gpb.crl);

        touch.make_interrupt_source(&mut afio);
        touch.trigger_on_edge(&cx.device.EXTI, Edge::Rising);
        touch.enable_interrupt(&cx.device.EXTI);

        let i2c = BlockingI2c::i2c1(
            cx.device.I2C1,
            (scl1, sda1),
            &mut afio.mapr,
            Mode::Standard {
                frequency: 100.kHz(),
            },
            clocks,
            1000,
            10,
            1000,
            1000,
        );

        let gled = gpb
            .pb1
            .into_push_pull_output_with_state(&mut gpb.crl, gpio::PinState::High);
        let mut timer = cx.device.TIM1.counter_ms(&clocks);
        timer.start(1.secs()).unwrap();
        timer.listen(Event::Update);

        let mut disp = [0; (vk16k33::DISP_SIZE + 1) as usize];
        disp[0] = vk16k33::DISP_SIZE;

        (
            Shared {
                led: gled,
                led_state: false,
                disp,
                i2c,
            },
            Local {
                timer,
                touch,
            },
            init::Monotonics(),
        )
    }

    #[idle(shared=[i2c, disp])]
    fn idle(mut cx: idle::Context) -> ! {
        cx.shared.i2c.lock(|i2c| {
            cx.shared.disp.lock(|disp| {
                vk16k33::init(i2c);
                vk16k33::clear(i2c);
            });
        });
        loop {
            cortex_m::asm::wfi();
        }
    }

    // #[task(binds=[GPIOAE], priority=1)]
    // fn int_touch(mut cx: int_touch::Context) {
    //     cx.shared.touch.read();
    // }

    #[task(binds=TIM1_UP, priority=1, local=[timer, count:u8=0], shared=[led, i2c])]
    fn tick(mut cx: tick::Context) {
        cx.shared.i2c.lock(|i2c| {

            vk16k33::draw_glyph(i2c, 0b0000000000111111, *cx.local.count);
            vk16k33::draw_glyph(i2c, Capitals::L as u16, 0);
            vk16k33::draw_glyph(i2c, Capitals::O as u16, 1);
            vk16k33::draw_glyph(i2c, Capitals::V as u16, 2);
            vk16k33::draw_glyph(i2c, Capitals::E as u16, 3);
            
            vk16k33::clear(i2c);
            vk16k33::draw_glyph(i2c, Capitals::L as u16, 0);
            vk16k33::draw_glyph(i2c, Capitals::I as u16, 1);
            vk16k33::draw_glyph(i2c, Capitals::L as u16, 2);
            vk16k33::draw_glyph(i2c, Capitals::I as u16, 3);
        });

        // Count used to change the timer update frequency
        let (v, _) = cx.local.count.overflowing_add(1);
        *cx.local.count = v;

        if *cx.local.count > 4 {
            *cx.local.count = 0;
        }

        // Clears the update flag
        cx.local.timer.clear_interrupt(Event::Update);
    }
}
