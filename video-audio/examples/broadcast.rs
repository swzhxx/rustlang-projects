use tokio::sync::broadcast;

#[tokio::main]
async fn main() {
    let (tx, mut rx1) = broadcast::channel(16);
    let mut rx2 = tx.subscribe();

    let handle = tokio::spawn(async move {
        // assert_eq!(rx1.recv().await.unwrap(), 10);
        // assert_eq!(rx1.recv().await.unwrap(), 20);
        println!("recv {:?}", rx1.recv().await.unwrap());
        println!("recv {:?}", rx1.recv().await.unwrap());
    });

    let handle2 = tokio::spawn(async move {
        assert_eq!(rx2.recv().await.unwrap(), 10);
        assert_eq!(rx2.recv().await.unwrap(), 20);
    });

    tx.send(10).unwrap();
    tx.send(20).unwrap();
    handle.await;
    handle2.await;
}
