use classifier::{predict_mlp, INPUT_HEIGHT as H, INPUT_WIDTH as W};
use classifier_io::prepare_input_image;

/// Reference: https://github.com/zalandoresearch/fashion-mnist?tab=readme-ov-file#labels
#[repr(u8)]
pub enum FashionMNISTLabel {
    TShirt = 0,
    Trouser = 1,
    Pullover = 2,
    Dress = 3,
    Coat = 4,
    Sandal = 5,
    Shirt = 6,
    Sneaker = 7,
    Bag = 8,
    AnkleBoot = 9,
}

#[test]
fn mlp_predict_trouser() {
    assert_eq!(
        predict_mlp(
            prepare_input_image(include_bytes!("../fixtures/trouser_0.jpg"), W, H).unwrap()
        ),
        FashionMNISTLabel::Trouser as u8
    );
}

#[test]
fn mlp_predict_sneaker() {
    assert_eq!(
        predict_mlp(
            prepare_input_image(include_bytes!("../fixtures/sneaker_0.jpg"), W, H).unwrap()
        ),
        FashionMNISTLabel::Sneaker as u8
    );
}

#[test]
fn mlp_predict_tshirt() {
    assert_eq!(
        predict_mlp(prepare_input_image(include_bytes!("../fixtures/tshirt_0.jpg"), W, H).unwrap()),
        FashionMNISTLabel::TShirt as u8
    );
}
