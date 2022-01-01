use anyhow::{Context, Result};
use clap::{App, AppSettings, Arg, SubCommand};
use lazy_static::lazy_static;
use std::{env, path};

pub const APP_NAME: &str = "arctis7ctl";
pub const APP_NAME_UPPER: &str = "ARCTIS7CTL";
