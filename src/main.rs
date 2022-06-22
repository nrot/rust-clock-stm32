#![deny(unsafe_code)]
#![no_std]
#![no_main]

use panic_halt as _;

mod glyph;
mod vk16k33;

#[rtic::app(device=stm32f1xx_hal::pac, peripherals = true, )]
mod app {

    use stm32f1xx_hal::{
        gpio::{self, Alternate, Edge, ExtiPin, Input, OpenDrain, Output, Pin, PullDown, PushPull, Analog},
        i2c::{BlockingI2c, Mode},
        pac::{self, I2C1, ADC1},
        prelude::*,
        timer::{CounterMs, Event}, adc::{Adc, Continuous, AdcPayload}, dma::RxDma,
    };
    use heapless::String;
    use core::fmt::Write;
    use core::iter::Iterator;

    use crate::glyph::{Glyph, Capitals, Symbols};
    use crate::vk16k33;

    type ODPB6 = Pin<'B', 6, false, Alternate<OpenDrain>>;
    type ODPB7 = Pin<'B', 7, false, Alternate<OpenDrain>>;
    type IAPA1 = Pin<'A', 1, false, Analog>;

    #[shared]
    struct Shared {
        i2c: BlockingI2c<I2C1, (ODPB6, ODPB7)>,
        led: gpio::PB1<Output<PushPull>>,
        to_draw: [Glyph; 4],
        led_state: bool,
    }

    #[local]
    struct Local {
        timer: CounterMs<pac::TIM1>,
        touch: gpio::PA10<Input<PullDown>>,
        adc: Adc<ADC1>,
        analog: IAPA1,
    }
    

    #[init]
    fn init(mut cx: init::Context) -> (Shared, Local, init::Monotonics) {
        let mut flash = cx.device.FLASH.constrain();
        let rcc = cx.device.RCC.constrain();

        let clocks = rcc.cfgr.sysclk(48.MHz()).adcclk(2.MHz()).freeze(&mut flash.acr);
        // let mut sys_cfg = cx.device.
        // let mut nvic = cx.core.NVIC.stir.

        cx.core.DCB.enable_trace();
        cx.core.DWT.enable_cycle_counter();

        let mut gpb = cx.device.GPIOB.split();
        let mut gpa = cx.device.GPIOA.split();

        let mut touch = gpa.pa10.into_pull_down_input(&mut gpa.crh);

        let analog = gpa.pa1.into_analog(&mut gpa.crl);
        // let dma = cx.device.DMA1.split().1;

        let mut adc = Adc::adc1(cx.device.ADC1, clocks);
        // let adc_dma = adc.with_dma(analog, dma);
        adc.set_continuous_mode(true);
        // adc.start_conversion();
        // static mut buf: [u16; 2] = [0u16; 2];
        // let b :u16 = adc.read_abs_mv(&mut analog).unwrap();

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

        let disp:[Glyph; 4] = [0u16.into(); 4];

        (
            Shared {
                led: gled,
                led_state: false,
                i2c,
                to_draw: disp,
            },
            Local {
                timer,
                touch,
                adc,
                analog
            },
            init::Monotonics(),
        )
    }

    #[idle(shared=[i2c, to_draw], local=[adc, analog])]
    fn idle(mut cx: idle::Context) -> ! {
        cx.shared.i2c.lock(|i2c| {
            vk16k33::init(i2c);
            vk16k33::clear(i2c);
        });
        let a = cx.local.adc;
        let analog = cx.local.analog;
        const BUFF_SIZE: u8= 16;
        let mut adc_buff = [0u16; BUFF_SIZE as usize];
        let mut adc_buff_p = 0u8;
        let mut sm = 0;
        loop {
            let b: u16 = a.read(analog).unwrap();
            if adc_buff_p < BUFF_SIZE{
                adc_buff[adc_buff_p as usize] = b;
                adc_buff_p += 1;
            } else {
                sm = (adc_buff.iter().cloned().map(u64::from).sum::<u64>() / (BUFF_SIZE as u64) / 16u64) as u16;
                adc_buff_p = 0;
                cx.shared.to_draw.lock(|to_draw|{
                    let mut data: String<4> = String::new();
                    let _ = write!(data, "{:>4}", sm);
                    data.as_bytes().iter().enumerate().map(|(i, v)|{
                        (i, Glyph::from(*v as char))
                    }).fold(to_draw , |a, (i, b)|{
                        a[i] = b;
                        a
                    });
                });
            }
            cortex_m::asm::wfi();
        }
    }

    // #[task(binds=[GPIOAE], priority=1)]
    // fn int_touch(mut cx: int_touch::Context) {
    //     cx.shared.touch.read();
    // }

    #[task(binds=TIM1_UP, priority=1, local=[timer, count:u8=0], shared=[led, i2c, to_draw])]
    fn tick(mut cx: tick::Context) {
        cx.shared.i2c.lock(|i2c| {
            cx.shared.to_draw.lock(|to_draw|{
                for (i, g) in to_draw.iter().enumerate(){
                    vk16k33::draw_glyph(i2c, u16::from(*g), i as u8);
                }
            })
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
