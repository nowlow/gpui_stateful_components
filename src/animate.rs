use gpui::{prelude::FluentBuilder, *};
use crate::state::{State, StateModel};
use std::time::{Instant, SystemTime};
use num_traits::{NumCast, FromPrimitive, ToPrimitive};

fn interpolate<T>(start: T, end: T, duration: f32, current_time: f32) -> T
where
    T: 'static + Copy + NumCast + ToPrimitive + FromPrimitive + core::fmt::Display + core::cmp::PartialEq,
{
    if current_time <= 0.0 {
        return start;
    }
    if current_time >= duration {
        return end;
    }

    let progress = current_time / duration;

    // Convert start and end to f64 for calculation
    let start_f64 = ToPrimitive::to_f64(&start).unwrap_or(0.0);
    let end_f64 = ToPrimitive::to_f64(&end).unwrap_or(0.0);

    // Perform the interpolation in floating point
    let interpolated = start_f64 + (end_f64 - start_f64) * progress as f64;

    // Convert back to T, rounding as necessary
    FromPrimitive::from_f64(interpolated).unwrap_or(start)
}

pub trait Animate {
    fn animate<T>(self, state: &State<T>, time_ms: u128, cx: &mut WindowContext, then: impl (FnOnce(Self, T) -> Self)) -> Self
    where
        T: 'static + Copy + NumCast + ToPrimitive + FromPrimitive + core::fmt::Display + core::cmp::PartialEq,
        Self: Sized;
}

impl Animate for Stateful<Div> {
    fn animate<T>(self, state: &State<T>, time_ms: u128, cx: &mut WindowContext, then: impl (FnOnce(Self, T) -> Self)) -> Self
    where
        T: 'static + Copy + NumCast + ToPrimitive + FromPrimitive + core::fmt::Display + core::cmp::PartialEq,
        Self: Sized
    {
        if let Some(value) = state.get() {
        //     let mut key = state.key.clone();
        //     key[1] = key[1] - 1;

        //     let now = Instant::now();

        //     let animation_state = State::new((value, now), key, cx);

            let mut start_key = state.key.clone();
            start_key[1] = start_key[1] - 1;

            let animation_start_state = State::new((value, false), start_key, cx);

        //     // cx.on_next_frame(move |cx| {
        //     if key[0] == 1 {
        //         println!("UPDATE");

        //         if let Some((animation_start_value, started)) = animation_start_state.get() {
        //             // let elpased = start.elapsed().as_millis();

        //             if value != animation_start_value && started == false {
        //                 animation_state.delete(cx);
        //                 let now = Instant::now();

        //                 println!("starts animation");
        //                 animation_state.update((animation_start_value, now), cx);
        //                 animation_start_state.update((animation_start_value, true), cx);
        //             }

        //             if let Some((current_value, start)) = animation_state.get() {
        //                 let elpased = start.elapsed().as_millis();

        //                 println!("animate [id: {}]\n\telpased:\t{}\n\tvalue:\t\t{}\n\tc_value:\t{}", key[0], elpased, value, current_value);

        //                 if elpased <= time_ms { // doesn't work for some reason?

        //                     // let new_value = interpolate(value, value)

        //                     println!("animation calls update");
        //                     animation_state.update((value, start), cx); // don't recall the animate function?
        //                     // cx.update_view::<StateModel, _>();
        //                 } else {
        //                     animation_state.update((value, now), cx);
        //                     animation_start_state.update((value, false), cx);
        //                 }
        //             }
        //         }
        //     }
        //     // });
    
            return self.map(move |this| {
                if let Some((value, _)) = animation_start_state.get() {

                    let frame = move |cx: &mut WindowContext| {
                        then(this, value);
                    };

                    cx.on_next_frame(frame);

                    this
                } else {
                    this
                }
            });
        }

        self
    }
}