// Copyright 2018 Guillaume Pinot (@TeXitoi) <texitoi@texitoi.eu>
//
// Licensed under the Apache License, Version 2.0 <LICENSE-APACHE or
// http://www.apache.org/licenses/LICENSE-2.0> or the MIT license
// <LICENSE-MIT or http://opensource.org/licenses/MIT>, at your
// option. This file may not be copied, modified, or distributed
// except according to those terms.

use clap::Clap;

#[derive(Clap)]
#[clap(name = "make-cookie")]
struct MakeCookie {
    #[clap(short)]
    s: String,

    #[clap(subcommand, flatten)]
    cmd: Command,
}

#[derive(Clap)]
enum Command {
    #[clap(name = "pound")]
    /// Pound acorns into flour for cookie dough.
    Pound { acorns: u32 },

    Sparkle {
        #[clap(short)]
        color: String,
    },
}

fn main() {}
