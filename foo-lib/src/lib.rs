pub fn useful_func() {}

mod private_mod {
    struct PrivateType;

    impl PartialEq<PrivateType> for usize {
        fn eq(&self, _other: &PrivateType) -> bool {
            unimplemented!();
        }
    }
}
