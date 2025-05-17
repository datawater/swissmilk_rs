#![allow(
    clippy::cast_possible_truncation,
    clippy::cast_possible_wrap,
    clippy::cast_sign_loss,
    reason = "WHO CARES IT'S JUST AN RNG"
)]

use std::num::Wrapping;
use std::ops::{Add, BitAnd, Mul, Shr};
use std::sync::atomic::Ordering;
use std::sync::{LazyLock, atomic::AtomicUsize};
use std::time::{SystemTime, UNIX_EPOCH};

pub trait NewRandom {
    fn new_random() -> Self;
}

pub trait RandInt: Copy + Shr<Output = Self> + BitAnd<Output = Self> {
    const RAND_MULTIPLIER: Wrapping<Self>;
    const RAND_INCREMENT: Wrapping<Self>;
    const RAND_SHIFT: Self;
    const RAND_MASK: Self;

    fn from_usize(x: usize) -> Self;
    fn to_usize(self) -> usize;
}

impl RandInt for i32 {
    const RAND_MULTIPLIER: Wrapping<Self> = Wrapping(214_013);
    const RAND_INCREMENT: Wrapping<Self> = Wrapping(2_531_011);
    const RAND_SHIFT: Self = 16;
    const RAND_MASK: Self = 0x7fff;

    #[inline]
    fn from_usize(x: usize) -> Self {
        x as Self
    }

    #[inline]
    fn to_usize(self) -> usize {
        self as usize
    }
}

impl RandInt for u32 {
    const RAND_MULTIPLIER: Wrapping<Self> = Wrapping(214_013);
    const RAND_INCREMENT: Wrapping<Self> = Wrapping(2_531_011);
    const RAND_SHIFT: Self = 16;
    const RAND_MASK: Self = 0x7fff;

    #[inline]
    fn from_usize(x: usize) -> Self {
        x as Self
    }

    #[inline]
    fn to_usize(self) -> usize {
        self as usize
    }
}

impl RandInt for i64 {
    const RAND_MULTIPLIER: Wrapping<Self> = Wrapping(6_364_136_223_846_793_005);
    const RAND_INCREMENT: Wrapping<Self> = Wrapping(1_442_695_040_888_963_407);
    const RAND_SHIFT: Self = 32;
    const RAND_MASK: Self = 0x7fff_ffff;

    #[inline]
    fn from_usize(x: usize) -> Self {
        x as Self
    }

    #[inline]
    fn to_usize(self) -> usize {
        self as usize
    }
}

impl RandInt for u64 {
    const RAND_MULTIPLIER: Wrapping<Self> = Wrapping(6_364_136_223_846_793_005);
    const RAND_INCREMENT: Wrapping<Self> = Wrapping(1_442_695_040_888_963_407);
    const RAND_SHIFT: Self = 32;
    const RAND_MASK: Self = 0x7fff_ffff;

    #[inline]
    fn from_usize(x: usize) -> Self {
        x as Self
    }

    #[inline]
    fn to_usize(self) -> usize {
        self as usize
    }
}

static RAND_NEXT: LazyLock<AtomicUsize> = LazyLock::new(|| {
    let now = SystemTime::now();
    AtomicUsize::new(
        now.duration_since(UNIX_EPOCH)
            .expect("We're in the past?")
            .as_millis() as usize,
    )
});

#[inline]
pub fn rand<T: RandInt>() -> T
where
    Wrapping<T>: Mul<Output = Wrapping<T>> + Add<Output = Wrapping<T>>,
{
    let mut cur = Wrapping(T::from_usize(RAND_NEXT.load(Ordering::Relaxed)));
    cur = cur * (T::RAND_MULTIPLIER) + (T::RAND_INCREMENT);

    RAND_NEXT.store(cur.0.to_usize(), Ordering::Relaxed);

    (cur.0 >> (T::RAND_SHIFT)) & T::RAND_MASK
}
