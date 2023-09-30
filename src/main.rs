extern crate tendermint_abci;

use tendermint_abci::{Application, RequestCheckTx, ResponseCheckTx, RequestDeliverTx, ResponseDeliverTx, RequestCommit, ResponseCommit};

struct CounterApp {
    count: i64,
}

impl Application for CounterApp {
    fn check_tx(&self, req: RequestCheckTx) -> ResponseCheckTx {
        let transaction = String::from_utf8(req.tx.clone()).unwrap_or_default();
        let parts: Vec<&str> = transaction.split('=').collect();

        if parts.len() != 2 {
            return ResponseCheckTx { code: 1, log: "Expected format: key=value".into(), ..Default::default() };
        }

        ResponseCheckTx::default()
    }

    fn deliver_tx(&mut self, req: RequestDeliverTx) -> ResponseDeliverTx {
        let transaction = String::from_utf8(req.tx).unwrap_or_default();
        let parts: Vec<&str> = transaction.split('=').collect();

        if parts[0] == "increment" {
            self.count += parts[1].parse::<i64>().unwrap_or(0);
        }

        ResponseDeliverTx::default()
    }

    fn commit(&self, _req: RequestCommit) -> ResponseCommit {
        let hash = format!("{:x}", self.count);

        ResponseCommit { data: hash.into_bytes(), ..Default::default() }
    }

}

fn main() {
    let app = CounterApp { count: 0 };
    let addr = "127.0.0.1:26658".parse().unwrap();
    println!("Starting ABCI server at: {}", addr);
    tendermint_abci::run(addr, app);
}
