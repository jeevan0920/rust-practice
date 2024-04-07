use std::marker::PhantomData;

struct Green;
struct Yellow;
struct Red;

struct TrafficLight<State> {
    _state: State,
}

trait ToYellow {
    fn to_yellow(self) -> TrafficLight<Yellow>;
}

trait ToRed {
    fn to_red(self) -> TrafficLight<Red>;
}

trait ToGreen {
    fn to_green(self) -> TrafficLight<Green>;
}

impl ToYellow for TrafficLight<Green> {
    fn to_yellow(self) -> TrafficLight<Yellow> {
        TrafficLight { _state: Yellow }
    }
}

impl ToRed for TrafficLight<Yellow> {
    fn to_red(self) -> TrafficLight<Red> {
        TrafficLight { _state: Red }
    }
}

impl ToGreen for TrafficLight<Red> {
    fn to_green(self) -> TrafficLight<Green> {
        TrafficLight { _state: Green }
    }
}

struct DataWrapper<T> {
    data_ptr: *const T,
    // Notice there's no direct storage of T or its lifetime here
    // phantom: PhantomData<&'a T>,
}

fn create_wrapper<'a>(data: &'a i32) -> DataWrapper<i32> {
    DataWrapper {
        data_ptr: data as *const i32,
        // phantom: PhantomData,
    }
}

fn main() {
    // Initialize the traffic light in the Green state
    let green_light = TrafficLight::<Green> { _state: Green };

    // Transition from Green -> Yellow -> Red -> Green
    let yellow_light = green_light.to_yellow();
    let red_light = yellow_light.to_red();
    let green_light_again = red_light.to_green();

    // The following line would not compile, as it's not a valid transition:
    // let invalid_transition = green_light_again.to_red();
}
