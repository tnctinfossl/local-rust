extern crate protoc_rust;

use protoc_rust::Customize;
use std::error::Error;

fn main ()->Result<(),Box<dyn Error>>{

    let proto_files=vec!["proto/grSim_Commands.proto",
    "proto/grSim_Packet.proto",
    "proto/grSim_Replacement.proto"
    ];
    let includes =vec!["proto"];
    protoc_rust::run(protoc_rust::Args{
        input:&proto_files[..],
        out_dir:"src/",
        includes:&includes[..],
        customize: Customize {
            ..Default::default()
        },
    })?;
    Ok(())
}

