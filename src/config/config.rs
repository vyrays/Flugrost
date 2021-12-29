use crate::ConfigHandler;
use std::error;

type Result<T> = std::result::Result<T, Box<dyn error::Error>>;

pub(crate) trait ConfigTrait {
    fn new() -> Result<ConfigHandler>;
}
