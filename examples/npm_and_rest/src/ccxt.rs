use stdweb::Value;

pub struct CcxtService(Option<Value>);

impl Drop for CcxtService {
    fn drop(&mut self) {
        let lib = self.0.take();
    }
}

impl CcxtService {
    pub fn new() -> Self {
        let lib = js! {
            return require("ccxt");
        };
        CcxtService(Some(lib))
    }

    pub fn exchanges(&mut self) {
        let lib = self.0.as_ref().expect("ccxt library object lost");
        let v: Value = js! {
            var ccxt = @{lib};
            console.log(ccxt.exchanges);
            return ccxt.exchanges;
        };
    }
}
