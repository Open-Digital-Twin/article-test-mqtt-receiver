use rumqttc::{EventLoop, MqttOptions, QoS, Request, Subscribe};
use std::time::Instant;
use std::env;

#[tokio::main(core_threads = 1)]
async fn main() {
    let args :Vec<String> = env::args().collect();
    
    let expected = &args[1];
    let max = expected.parse::<u16>().unwrap() + 3;  

    let mut mqttoptions = MqttOptions::new("Reciever", "localhost", 1883);
        mqttoptions.set_keep_alive(10000);
    let mut eventloop = EventLoop::new(mqttoptions, 10).await;
    let requests_tx = eventloop.handle();

    let subscription = Subscribe::new("hello", QoS::AtMostOnce);
    let _ = requests_tx.send(Request::Subscribe(subscription)).await;
    let mut counter : u16 = 0;


    while counter < max{

        if counter < 4 {
        loop {
            let incoming = eventloop.poll().await;
            println!("Incoming = {:?}", incoming);
            counter += 1;
            break    
        }}

        else {
        let now = Instant::now();    
        loop {
            let incoming = eventloop.poll().await;
            println!("Incoming = {:?}", incoming);
            counter += 1;
            if counter >= max {break}
        }
        println!("{}",now.elapsed().as_millis());
        }
     }    
}