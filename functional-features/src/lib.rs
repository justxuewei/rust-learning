pub mod closures;
pub mod iteration;

mod test {
    #[test]
    fn calling_next_directly() {
        let mut counter = super::iteration::Counter::new();

        assert_eq!(counter.next(), Some(1));
        assert_eq!(counter.next(), Some(2));
        assert_eq!(counter.next(), Some(3));
        assert_eq!(counter.next(), Some(4));
        assert_eq!(counter.next(), Some(5));
        assert_eq!(counter.next(), None);
    }
}
