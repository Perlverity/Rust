struct CarSpec {
    model: i32,
    cc: i32,
    color: i32,
}

struct Body {
    weight: f64,
    height: f64,
}

fn main() {
    let car1 = CarSpec {
        model: 2018,
        cc: 1500,
        color: 0xFF0000,
    };

    let car2 = CarSpec {
        model: 2019,
        cc: 2000,
        color: 0x00FF00,
    };

    println!("car1: {}, {}cc, {:06x}", car1.model, car1.cc, car1.color);
    println!("car2: {}, {}cc, {:06x}", car2.model, car2.cc, car2.color);

    let ichiro = Body { weight: 90.0, height: 170.0 };
    let jiro = Body { weight: 70.0, height: 180.0 };

    println!("Ichiro:{:.1}", calc_bmi(&ichiro));
    println!("Jiro:{:.1}", calc_bmi(&jiro));
}

fn calc_bmi(body: &Body) -> f64 {
    let h = body.height / 100.0;
    body.weight / h.powf(2.0)
}
