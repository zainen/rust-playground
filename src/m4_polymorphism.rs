#[cfg(test)]
mod test {
  use ethers::types::Address;
  use std::str::FromStr;

  trait EthAddress {
    fn convert_address(&self) -> Result<Address, &'static str>;
  }

  impl EthAddress for &str {
    fn convert_address(&self) -> Result<Address, &'static str> {
      match Address::from_str(self) {
        Ok(address) => Ok(address),
        Err(_) => Err("Invalid Eth Address String")
      }
    }
  }

  impl EthAddress for Address {
    fn convert_address(&self) -> Result<Address, &'static str> {
      Ok(*self)
    }
  }


  fn get_eth_data<T: EthAddress>(address: T) -> Address {
    let converted_address: Address = address.convert_address().unwrap();
    converted_address
  }

  #[test]
  fn test_polymorphism() {
    let addr = Address::from_str("0x388C818CA8B9251b393131C08a736A67ccB19297").unwrap();

    let new_addr: Address = get_eth_data(addr);
    assert_eq!(new_addr, Address::from_str("0x388C818CA8B9251b393131C08a736A67ccB19297").unwrap());

    let new_addr: Address = get_eth_data("0x388C818CA8B9251b393131C08a736A67ccB19297");
    assert_eq!(new_addr, Address::from_str("0x388C818CA8B9251b393131C08a736A67ccB19297").unwrap());
  }
}