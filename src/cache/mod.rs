/// 계산에 대한 동작을 설정하고 반환 값을 캐시하는 구조체입니다.
///
///  # Example
/// ``` rust
/// use tmrustplayground::cache::Cache;
///
/// let mut cache = Cache::new(|value| value + 10);
///
/// assert_eq!(15, cache.value(5)); // 값이 일치합니다.
/// assert_ne!(20, cache.value(10)); // 값이 일치하지 않습니다. 이미 초기 계산 결과가 있으므로 캐시된 값을 반환합니다.
/// ```
///

pub struct Cache<T>
    where T: Fn(u32) -> u32 {
    calculate: T,
    value: Option<u32>,
}

impl<T> Cache<T>
    where T: Fn(u32) -> u32 {
    pub fn new(calculate: T) -> Cache<T> {
        Cache {
            calculate,
            value: None
        }
    }

    pub fn value(&mut self, arg: u32) -> u32 {
        match self.value {
            Some(value) => value,
            None => {
                let value = (self.calculate)(arg);
                self.value = Some(value);

                value
            }
        }
    }
}