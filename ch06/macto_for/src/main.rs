macro_rules! easy_for {
    (
        for $i:ident = $from:tt to $to:tt $block:block
    ) => {{
        for $i in $from..=$to {
            $block
        }
    }};

    (
        for $i:ident = $from:tt to $to:tt step $step:tt $block:block
    ) => {{
        let mut $i = $from;
        loop {
            if $i > $to { break; }
            $block
            $i += $step;
        }
    }};
}

macro_rules! map_init {
    ( $($key:expr => $val:expr), * )=> {{
        let mut tmp = std::collections::HashMap::new();
        $ (
            tmp.insert($key, $val);
        ) *
        tmp
    }}
}

macro_rules! bmi_select {
    ( $bmi:expr; $( $label:expr => $range:expr);+) => {{
        let mut result = "error";
        $ (
            if $range.start <= $bmi && $bmi < $range.end {
                result = $label;
            }
        ) +
        result
    }};
}

fn main() {
    let mut total = 0;
    easy_for! {
        for i = 1 to 10 {
            total += i;
        }
    }
    println!("total: {}", total);

    easy_for! {
        for i = 0 to 10 step 3 {
            println!("{}", i);
        }
    }

    let week = map_init![
        "monday" => 1,
        "tuesday" => 2,
        "wednesday" => 3,
        "thursday" => 4,
        "friday" => 5,
        "saturday" => 6,
        "sunday" => 7
    ];
    println!("week: {:?}", week);
    println!("wed={}", week["wednesday"]);

    let h: f32 = 158.0;
    let w: f32 = 63.0;
    let bmi = w / (h / 100.0).powf(2.0);

    let label = bmi_select! [
        bmi;
        "thin" => 0.0..18.5;
        "normal" => 18.5..25.0;
        "fat" => 25.0..30.0;
        "obese" => 30.0..99.9];
    println!("bmi: {}, label: {}", bmi, label);
}
