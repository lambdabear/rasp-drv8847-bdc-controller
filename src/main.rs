use cursive::{
    traits::Boxable,
    views::{Button, CircularFocus, Dialog, DummyView, LinearLayout, Panel, ProgressBar, TextView},
    Cursive, CursiveExt, With,
};
use rppal::pwm::{Channel, Polarity, Pwm};
use std::sync::{Arc, Mutex};

const PWM_FREQUENCY: f64 = 30_000.0;

enum Direction {
    Left,
    Right,
    Stop,
}

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let pwm0 = Pwm::with_frequency(Channel::Pwm0, PWM_FREQUENCY, 1.0, Polarity::Normal, true)?;
    let pwm1 = Pwm::with_frequency(Channel::Pwm1, PWM_FREQUENCY, 1.0, Polarity::Normal, true)?;

    let pwm0 = Arc::new(Mutex::new(pwm0));
    let pwm0_clone1 = pwm0.clone();
    let pwm0_clone2 = pwm0.clone();
    let pwm0_clone3 = pwm0.clone();
    let pwm0_clone4 = pwm0.clone();

    let pwm1 = Arc::new(Mutex::new(pwm1));
    let pwm1_clone1 = pwm1.clone();
    let pwm1_clone2 = pwm1.clone();
    let pwm1_clone3 = pwm1.clone();
    let pwm1_clone4 = pwm1.clone();

    let mut siv = Cursive::new();
    let speed = Arc::new(Mutex::new(0));
    let speed_clone1 = speed.clone();
    let speed_clone2 = speed.clone();
    let speed_clone3 = speed.clone();
    let speed_clone4 = speed.clone();

    let direction = Arc::new(Mutex::new(Direction::Stop));
    let direction_clone1 = direction.clone();
    let direction_clone2 = direction.clone();
    let direction_clone3 = direction.clone();
    let direction_clone4 = direction.clone();

    // Creates a dialog with a single "Quit" button
    siv.add_layer(
        Panel::new(
            LinearLayout::vertical()
                .child(
                    LinearLayout::horizontal()
                        .child(
                            Dialog::around(TextView::new(""))
                                .title("Direction")
                                .button(" <- ", move |_| {
                                    let mut direction =
                                        direction.lock().expect("direction lock failed");
                                    *direction = Direction::Left;

                                    let speed =
                                        speed_clone3.lock().expect("speed_clone3 lock failed");

                                    let pwm1 = pwm1.lock().expect("pwm1 lock failed");
                                    pwm1.set_duty_cycle(0.0).ok();

                                    let pwm0 = pwm0_clone4.lock().expect("pwm0_clone4 lock failed");
                                    pwm0.set_duty_cycle(*speed as f64 * 0.01).ok();
                                })
                                .button(" -> ", move |_| {
                                    let mut direction = direction_clone1
                                        .lock()
                                        .expect("direction_clone1 lock failed");
                                    *direction = Direction::Right;

                                    let speed =
                                        speed_clone4.lock().expect("speed_clone4 lock failed");

                                    let pwm0 = pwm0.lock().expect("pwm0 lock failed");
                                    pwm0.set_duty_cycle(0.0).ok();

                                    let pwm1 = pwm1_clone4.lock().expect("pwm1_clone4 lock failed");
                                    pwm1.set_duty_cycle(*speed as f64 * 0.01).ok();
                                })
                                .wrap_with(|v| CircularFocus::new(v, true, true)), // .fixed_width(30),
                        )
                        .child(DummyView.fixed_width(1))
                        .child(
                            Dialog::around(TextView::new(""))
                                .title("Speed")
                                .button(" + ", move |_| {
                                    let direction =
                                        direction_clone2.lock().expect("direction lock failed");

                                    let mut speed = speed.lock().unwrap();

                                    if *speed < 100 {
                                        *speed += 1;
                                    }

                                    match *direction {
                                        Direction::Left => {
                                            let pwm0 =
                                                pwm0_clone1.lock().expect("pwm0_clone lock failed");

                                            pwm0.set_duty_cycle(*speed as f64 * 0.01).ok();
                                        }
                                        Direction::Right => {
                                            let pwm1 =
                                                pwm1_clone1.lock().expect("pwm1_clone lock failed");

                                            pwm1.set_duty_cycle(*speed as f64 * 0.01).ok();
                                        }
                                        Direction::Stop => (),
                                    }
                                })
                                .button(" - ", move |_| {
                                    let direction =
                                        direction_clone3.lock().expect("direction lock failed");

                                    let mut speed = speed_clone1.lock().unwrap();

                                    if *speed < 100 {
                                        *speed -= 1;
                                    }

                                    match *direction {
                                        Direction::Left => {
                                            let pwm0 =
                                                pwm0_clone2.lock().expect("pwm0_clone lock failed");

                                            pwm0.set_duty_cycle(*speed as f64 * 0.01).ok();
                                        }
                                        Direction::Right => {
                                            let pwm1 =
                                                pwm1_clone2.lock().expect("pwm1_clone lock failed");

                                            pwm1.set_duty_cycle(*speed as f64 * 0.01).ok();
                                        }
                                        Direction::Stop => (),
                                    }
                                })
                                .wrap_with(|v| CircularFocus::new(v, true, true)), // .fixed_width(30),
                        ),
                )
                .child(DummyView.fixed_height(1))
                .child(Button::new("Stop", move |_| {
                    let pwm0 = pwm0_clone3.lock().expect("pwm0 lock failed");
                    let pwm1 = pwm1_clone3.lock().expect("pwm1 lock failed");
                    pwm1.set_duty_cycle(1.0).ok();
                    pwm0.set_duty_cycle(1.0).ok();

                    let mut direction = direction_clone4.lock().expect("direction lock failed");
                    *direction = Direction::Stop;
                }))
                .child(DummyView.fixed_height(1))
                .child(ProgressBar::default().with_task(move |counter| loop {
                    let i = speed_clone2.lock().expect("Mutex speed lock failed");

                    counter.set(*i);
                })),
        )
        .title("DC Motor Controller"),
    );

    siv.add_global_callback('q', |s| s.quit());

    siv.run();

    Ok(())
}
