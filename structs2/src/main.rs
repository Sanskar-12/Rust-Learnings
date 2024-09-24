// if both have same tuple structure but if i pass red inside setpoint then this will cause bug 
// tuple structs was made to differentiate same types of tuples
struct Color(u8,u8,u8);
struct Point(u8,u8,u8);
fn main() {
    let red = Color(100,0,0);
    set_bg_color(red);

    let point = Point(39,49,90);
    // set_point(red);   // this will give error because its expecting the Point Type but found Color Type                                     
}

fn set_bg_color(color:Color) {
    println!("Color is Red={}, Green={}, Blue={}",color.0, color.1, color.2);
}

fn set_point(point:Point) {
    println!("Point is X={}, Y={}, Z={}",point.0, point.1, point.2);
}

