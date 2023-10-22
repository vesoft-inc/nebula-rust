//
// ref https://stackoverflow.com/questions/39638363/how-can-i-use-a-hashmap-with-f64-as-key-in-rust
//
use core::cmp::Ordering;

//
#[derive(Clone, Copy, Debug, Default)]
pub struct Double(pub f64);

impl Double {
    fn canonicalize(&self) -> i64 {
        (self.0 * 1024.0 * 1024.0).round() as i64
    }
}

impl PartialEq for Double {
    fn eq(&self, other: &Double) -> bool {
        self.canonicalize() == other.canonicalize()
    }
}

impl Eq for Double {}

impl PartialOrd for Double {
    fn partial_cmp(&self, other: &Double) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

impl Ord for Double {
    fn cmp(&self, other: &Double) -> Ordering {
        self.canonicalize().cmp(&other.canonicalize())
    }
}

impl<P> ::fbthrift::Serialize<P> for Double
where
    P: ::fbthrift::ProtocolWriter,
{
    #[inline]
    fn write(&self, p: &mut P) {
        p.write_double(self.0)
    }
}

impl<P> ::fbthrift::Deserialize<P> for Double
where
    P: ::fbthrift::ProtocolReader,
{
    #[inline]
    fn read(p: &mut P) -> ::anyhow::Result<Self> {
        ::std::result::Result::Ok(Self(p.read_double()?))
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    use float_cmp::approx_eq;

    #[test]
    fn set_get() {
        assert!(approx_eq!(f64, Double(1.0_f64).0, 1.0_f64));
    }
}
