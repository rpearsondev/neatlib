use bevy::prelude::In;

pub type Shape = Vec<u32>;

pub struct NeuralModel;

impl NeuralModel{
    pub fn new() -> Self{
        todo!()
    }

    fn add_layer(&self, layer: &dyn Layer) {
        todo!()
    }

    fn shape(&self) -> Shape{
        vec![0]
    }
}

trait Layer {
    
}

pub struct InputLayer;

impl InputLayer{
    pub fn new(shape: Shape) -> Self{
         Self
    }
}

impl Layer for InputLayer{

}

#[cfg(test)]
mod number_line_test{
    use super::{NeuralModel, InputLayer};

    #[test]
    fn print_check(){
        let model = NeuralModel::new();
        model.add_layer(&InputLayer::new(vec![28, 28, 1]));
        println!("{:?}", model.shape()) ;
        // pipeline.add_layer(Conv::new(ConvOptions{}));
        // pipeline.add_layer(MapPool::new(MaxPoolOptions{}));
        // pipeline.add_layer(Dense::new(vec![4], DenseOptions{}));
        // pipeline.add_layer(OutputLayer::new(vec![9]));
    }

}