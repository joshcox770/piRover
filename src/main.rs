use std::net::{TcpListener, UdpSocket, SocketAddr};
use std::io::{Read, Write};
use std::thread;
use rppal::gpio::Gpio;
use v4l::{prelude::*, FourCC};
use v4l::Format;
use v4l::buffer::Type;
use v4l::io::traits::CaptureStream;

mod motor;
use motor::Motor;

fn main() {
    // startup
    let gpio = match Gpio::new() {
        Ok(gpio) => gpio,
        Err(error) => panic!("Problem binding gpio pins: {error:?}"),
    };

    let control_thread = thread::spawn(|| {
        control_server(9999, gpio);
    });
    let video_thread = thread::spawn(|| {
        video_server(10000);
    });

    control_thread.join().unwrap();

    // camera

    // set up listener for connection

    // loop where camera is sent back via udp

    // listen for packet coming in where 
}

fn control_server(port: u16, gpio: Gpio) {
    let mut left_motor = Motor::new(&gpio, 12, 1, 7);
    let mut right_motor = Motor::new(&gpio, 13, 6, 5);

    let listener = TcpListener::bind(format!("{}{}", "0.0.0.0:", port)).unwrap();
    println!("Listening for control input on port {}", port);

    let mut handle_message = | mut message: &str |  {
        println!("Received: {}", message);

        let mut handle_turn = |amount: i8| {
            println!("turn");
            // left will be pin 1
        };

        let mut handle_drive = |amount: i8| {
            println!("set drive {}", amount);
            left_motor.set_power(amount);
            right_motor.set_power(amount);
        };

        let message_split: Vec<&str> = message.split(":").collect();
        if message_split.len() >= 1 {
            let command = message_split[0];
            match command {
                "MOTORS" => { 
                    let left_motor_power = match message_split[1].parse::<i8>() {
                        Ok(param) => param,
                        Err(error) => {
                            println!("error parsing integer from control request {error:?}");
                            127
                        }
                    };
                    let right_motor_power = match message_split[2].parse::<i8>() {
                        Ok(param) => param,
                        Err(error) => {
                            println!("error parsing integer from control request {error:?}");
                            127
                        }
                    };
                    left_motor.set_power(left_motor_power);
                    right_motor.set_power(right_motor_power);
                },
                &_ => print!("Problem reading command"),
            };
        }

    };

    for stream in listener.incoming() {
        let mut stream = stream.unwrap();
        let mut buffer = [0; 512];
        
        let bytes_read = stream.read(&mut buffer).unwrap();
        let message = &*String::from_utf8_lossy(&buffer[..bytes_read]);

        handle_message(message); 

        let response = "Received";
        let _ = stream.write(response.as_bytes()).unwrap();
        let _ = stream.flush();
    }
}

fn video_server(port: u16) {
    const WIDTH: u32 = 1920;
    const HEIGHT: u32 = 1080;
    let device_path = "/dev/video0";

    let mut device = Device::with_path(device_path).unwrap();
    let mut videoStream = MmapStream::with_buffers(&mut device, Type::VideoCapture, 4).expect("failed to create video stream");

    let socket = UdpSocket::bind("0.0.0.0:0").unwrap();
    socket.set_broadcast(true).unwrap();
    let addr = format!("{}{}", "255.255.255.255:", port).parse::<SocketAddr>().unwrap();

    loop {
        let (buf, meta) = videoStream.next().unwrap();
        let chunked_buffer: Vec<Vec<u8>> = buf.chunks(508).map(|chunk| chunk.to_vec()).collect();

        for chunk in chunked_buffer {
            let status = socket.send_to(&chunk, addr);
            match status {
                Ok(size) => {},
                Err(e) => { println!("{}", e) }
            }
        }
    }

}