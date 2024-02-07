use candle_core::{Device, Result, Tensor};

struct Model {
    first: Tensor,
    second: Tensor,
}

impl Model {
    fn forward(&self, image: &Tensor) -> Result<Tensor> {
        let x = image.matmul(&self.first)?;
        let x = x.relu()?;
        x.matmul(&self.second)
    }
}

fn main() -> Result<()> {
    println!("candle!");
    let device = Device::Cpu;
    let first = Tensor::randn(0f32, 1., (784, 100), &device)?;
    let second = Tensor::randn(0f32, 1., (100, 10), &device)?;
    let model = Model { first, second };

    let dummy_image = Tensor::randn(0f32, 1., (1, 784), &device)?;
    let digit = model.forward(&dummy_image)?;
    println!("Digit {digit:?} digit");
    Ok(())
}
