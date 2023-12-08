pub trait Routers {
    fn build(self) -> Vec<salvo::Router>;
}
