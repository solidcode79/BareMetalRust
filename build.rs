use std::env;

const BOARDS: &[(&str, &str)] = &[
    ("CARGO_FEATURE_LM3S6965", "linker/lm3s6965.ld"),
    ("CARGO_FEATURE_STM32F103", "linker/stm32f103.ld"),
];


fn main() {
    
    let selected: Vec<&(&str, &str)> = BOARDS
        .iter()
        .filter(|(feature, _)| env::var(feature).is_ok())
        .collect();

    match selected.len() {
        1 => {
            let (_, linker_script) = selected[0];
            println!("cargo:rustc-link-arg=-T{}", linker_script);
        }
        0 => panic!("No board feature selected. Use: cargo build --features <board>"),
        _ => panic!("Multiple board features selected. Select exactly one."),
    }    

}
