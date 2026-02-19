#[cfg(feature = "lm3s6965")]
pub mod lm3s6965;

#[cfg(feature = "stm32f103")]
pub mod stm32f103;

#[cfg(feature = "lm3s6965")]
pub use lm3s6965 as board;

#[cfg(feature = "stm32f103")]
pub use stm32f103 as board;
