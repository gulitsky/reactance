#[cfg(feature = "f32")]
pub type Abc = (f32, f32, f32);
#[cfg(feature = "f64")]
pub type Abc = (f64, f64, f64);

#[cfg(feature = "f32")]
pub type AlphaBetaGamma = (f32, f32, f32);
#[cfg(feature = "f64")]
pub type AlphaBetaGamma = (f64, f64, f64);
