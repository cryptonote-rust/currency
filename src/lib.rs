use dirs;

use cryptonote_config::Config;

pub struct Currency {
  pub config: Config,
  pub is_home_dir: bool,
}

impl Currency {
  // Get Files
  fn get_files(&self, name: &String) -> String {
    let mut path;
    if cfg!(unix) && self.is_home_dir {
      path = dirs::home_dir().unwrap();
      path.push(format!(".{}", self.config.coinName));
    } else {
      path = dirs::data_dir().unwrap();
      path.push(format!("{}", self.config.coinName));
    }

    path.push(name);

    let filename = String::from(path.to_str().unwrap());
    return filename;
  }

  pub fn get_block_main_file(&self) -> String {
    return self.get_files(&self.config.files.main);
  }

  pub fn get_block_cache_file(&self) -> String {
    return self.get_files(&self.config.files.cache);
  }

  pub fn get_block_index_file(&self) -> String {
    return self.get_files(&self.config.files.index);
  }

  pub fn get_block_chain_index_file(&self) -> String {
    return self.get_files(&self.config.files.chainIndex);
  }

  pub fn new(config: Config) -> Currency {
    Currency {
      config: config,
      is_home_dir: false,
    }
  }

  pub fn old(config: Config) -> Currency {
    Currency {
      config: config,
      is_home_dir: true,
    }
  }
}

#[cfg(test)]

mod tests {
  use super::*;
  use cryptonote_basic::Version;
  use cryptonote_config::{BlockFiles, Config, NetType};

  #[test]
  fn should_get_block_main_file() {
    let files = BlockFiles::new([
      String::from("blocks.dat"),
      String::from("blockindexes.dat"),
      String::from("blockscache.dat"),
      String::from("blockchainindices.dat"),
    ]);
    let config = Config::new(
      0x3d,
      files,
      String::from("vigcoin"),
      String::from("aaa"),
      Version {
        major: 1,
        minor: 0,
        patch: 0,
      },
      NetType::Main
    );

    let currency = Currency::old(config);
    let mut path = dirs::data_dir().unwrap();
    path.push(currency.config.coinName.clone());
    assert!(currency.get_files(&String::from("blocks.dat")) == currency.get_block_main_file());
    assert!(currency.get_files(&String::from("blockindexes.dat")) == currency.get_block_index_file());
    assert!(currency.get_files(&String::from("blockscache.dat")) == currency.get_block_cache_file());
    assert!(currency.get_files(&String::from("blockchainindices.dat")) == currency.get_block_chain_index_file());
    println!("{}", currency.get_block_main_file());
    println!("{}", currency.get_block_index_file());
    println!("{}", currency.get_block_cache_file());
    println!("{}", currency.get_block_chain_index_file());
  }

  #[test]
  fn should_get_v2_block_file() {
    let files = BlockFiles::new([
      String::from("blocks.dat"),
      String::from("blockindexes.dat"),
      String::from("blockscache.dat"),
      String::from("blockchainindices.dat"),
    ]);
    let config = Config::new(
      0x3d,
      files,
      String::from("vigcoin"),
      String::from("aaa"),
      Version {
        major: 2,
        minor: 0,
        patch: 0,
      },
      NetType::Test
    );

    let currency = Currency::new(config);
    let mut path = dirs::data_dir().unwrap();
    path.push(currency.config.coinName.clone());
    assert!(currency.get_files(&String::from("blocks.dat")) == currency.get_block_main_file());
    assert!(currency.get_files(&String::from("blockindexes.dat")) == currency.get_block_index_file());
    assert!(currency.get_files(&String::from("blockscache.dat")) == currency.get_block_cache_file());
    assert!(currency.get_files(&String::from("blockchainindices.dat")) == currency.get_block_chain_index_file());
    println!("{}", currency.get_block_main_file());
    println!("{}", currency.get_block_index_file());
    println!("{}", currency.get_block_cache_file());
    println!("{}", currency.get_block_chain_index_file());
  }
}
