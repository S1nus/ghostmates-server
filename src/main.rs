use std::env;
use std::net::{SocketAddrV4, SocketAddr};
use std::sync::{Arc, Mutex};

use async_std::prelude::*;
use async_std::{task,
    io::{BufRead, BufReader, BufWriter},
    net::{TcpListener, TcpStream},
    stream::{Stream, StreamExt},
};

use futures::stream::TryStreamExt;
use futures::SinkExt;
use futures_codec::{Bytes, BytesMut, LengthCodec, Framed, FramedWrite, Decoder, Encoder};
use std::io::{Error, ErrorKind};

use ghostmates_common::{new_codec_reader, new_codec_writer};

async fn run_server(addr: SocketAddrV4) {
    let listener = TcpListener::bind(addr)
        .await
        .expect(format!("Failed to bind to {}", addr).as_str());

    println!("Listening on {}", &addr);

    while let Ok((stream, addr)) = listener.accept().await {
        println!("Got a stream from {:?}!", addr);
        task::spawn(connection_loop(stream, addr));
    }
}

async fn connection_loop(stream: TcpStream, addr: SocketAddr) {
    let stream_reader = BufReader::new(&stream);
    let stream_writer = BufWriter::new(&stream);

    let mut codec_reader = new_codec_reader(stream_reader);
    let mut codec_writer = new_codec_writer(stream_writer);

    while let Some(message) = codec_reader.try_next().await.expect("error with codec") {
        println!("{:?}", message);
        &codec_writer.send("You suck".to_owned())
            .await
            .expect("failed to send message");
    }
}

fn main() {
    task::block_on(run_server(
        SocketAddrV4::new(
            "127.0.0.1"
            .parse()
            .unwrap(),
            4000
        )
    ));
}
