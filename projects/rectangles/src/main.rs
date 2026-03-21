#[derive(Debug)]
struct Rectangle{
    width:u32,
    height:u32,
}

impl Rectangle{
    fn area (&self)->u32{
        self.width*self.height
    }
}

impl Rectangle{
    fn can_hold(&self, other: &Rectangle)->bool{
        self.width > other.width && self.height > other.height
    }

    fn square(size: u32) -> Self {
        Self {
            width: size,
            height: size,
        }
    }
}

fn main() {
    let width1 = 30;
    let height1 = 50;
    let rect1 = (30,50);
    let rect2 = Rectangle{width:30,height:50};
    let rect3 = Rectangle{width:20,height:40};
    let sq = Rectangle::square(width1);
    println!("The area (v0) of the rectangle is {} square pixels.",area_v0(width1,height1));
    println!("The area (v1) of the rectangle is {} square pixels.",area_v1(rect1));
    println!("The area (v2) of the rectangle is {} square pixels.",area_v2(&rect2));
    println!("The area (v3) of the rectangle is {} square pixels.",rect2.area());
    println!("Can rect2 hold rect3? {}",rect2.can_hold(&rect3));
    println!("Can rect3 hold rect2? {}",rect3.can_hold(&rect2));

    println!("Rectangle 1 is {:?}",rect1);
    println!("Rectangle 2 is {:?}",rect2);
    dbg!(&rect3);
    println!("Rectangle 3 is {:#?}",rect3);
    println!("Square is {:#?}",sq);

}

fn area_v0 (width: u32, height: u32)->u32{
    width*height
}
 
fn area_v1 (dimensions: (u32,u32))->u32{
    dimensions.0*dimensions.1 
}

fn area_v2 (rectangle: &Rectangle)->u32{
    rectangle.width*rectangle.height
}
