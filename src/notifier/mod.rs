pub trait Notifier {
    type AccessInput;

    fn acquire_notified_access(&self, input: Self::AccessInput);
}