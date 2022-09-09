use serde::{Serialize, Serializer};

#[derive(Debug, Serialize)]
pub struct TransactionOperation {
  pub client: u16,
  #[serde(serialize_with = "round_serialize")]
  pub available: f64,
  #[serde(serialize_with = "round_serialize")]
  pub held: f64,
  #[serde(serialize_with = "round_serialize")]
  pub total: f64,
  pub locked: bool,
}

impl TransactionOperation {
  pub fn new(client: u16) -> Self {
    Self {
        client,
        available: 0.0,
        held: 0.0,
        total: 0.0,
        locked: false,
    }
  }

  pub fn deposit(&mut self, amount: f64) {
    let new_total = self.total + amount;

    if new_total > self.total {
        self.total = new_total;
        self.set_available();
    } else {
        eprintln!("Wrong amount detected, ignoring deposit");
    }
  }
  pub fn withdraw(&mut self, amount: f64) {
    if self.available >= amount {
        self.total -= amount;
        self.set_available();
    } else {
        eprintln!("not enough available balance, ignoring withdrawal");
    }
  }
  
  pub fn hold(&mut self, amount: f64) {
      self.held += amount;
      self.set_available();
  }

  pub fn release(&mut self, amount: f64) {
      self.held -= amount;
      self.set_available();
  }

  pub fn chargeback(&mut self, amount: f64) {
      self.held -= amount;
      self.total -= amount;
      self.locked = true;
      self.set_available();
  }

  fn set_available(&mut self) {
      self.available = self.total - self.held
  }
}

fn round_serialize<S: Serializer>(value: &f64, s: S) -> Result<S::Ok, S::Error> {
  // Round to a maximum of 4 decimal places
  s.serialize_f64((value * 10000.0).round() / 10000.0)
}
