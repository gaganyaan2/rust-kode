use serde::{Serialize, Deserialize};
use serde_yaml::{Mapping, Value};

#[derive(Serialize, Deserialize, PartialEq, Debug)]

enum Enum {
    Unit,
    Newtype(usize),
    Tuple(usize, usize, usize),
    Struct { x: f64, y: f64 },
}

fn main(){
    let mut m = Mapping::new();
    m.insert("namespace".into(), "test".into());
    let x: Value = m.into();
    println!("{:?}",x)
    // let yaml = "
    //     - !Newtype 1
    //     - !Tuple [0, 0, 0]
    //     - !Struct {x: 1.0, y: 2.0}
    // ";
    // let values: Vec<Enum> = serde_yaml::from_str(yaml).unwrap();
    // assert_eq!(values[0], Enum::Newtype(1));
    // assert_eq!(values[1], Enum::Tuple(0, 0, 0));
    // assert_eq!(values[2], Enum::Struct { x: 1.0, y: 2.0 });

    // // The last two in YAML's block style instead:
    // let yaml = "
    //     - !Tuple
    //       - 0
    //       - 0
    //       - 0
    //     - !Struct
    //       x: 1.0
    //       y: 2.0
    // ";
    // let values: Vec<Enum> = serde_yaml::from_str(yaml).unwrap();
    // assert_eq!(values[0], Enum::Tuple(0, 0, 0));
    // assert_eq!(values[1], Enum::Struct { x: 1.0, y: 2.0 });

    // // Variants with no data can be written using !Tag or just the string name.
    // let yaml = "
    //     - Unit  # serialization produces this one
    //     - !Unit
    // ";
    // let values: Vec<Enum> = serde_yaml::from_str(yaml).unwrap();
    // assert_eq!(values[0], Enum::Unit);
    // assert_eq!(values[1], Enum::Unit);

    // Ok(())
}