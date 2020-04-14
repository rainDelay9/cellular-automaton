pub trait Rules {
    type Neighborhood;

    fn new(config: &PathBuf) -> Self;

    fn perform_round(neighborhood: Self::Neighborhood) -> u8;
}
