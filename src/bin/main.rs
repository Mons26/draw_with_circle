extern crate draw_with_circles;
extern crate sdl2;

use draw_with_circles::dwc::*;

pub fn main()
{
    let dir: String = String::from("./target/debug/paths/");
    let mut file: String = String::from("default.txt");

    println!("use default drawing? (yes or no)");
    let mut option: String = String::new();
    std::io::stdin().read_line(&mut option).expect("??????");
    option.pop(); option.pop(); 

    assert!(option.eq("yes") || option.eq("no"));

    if option.eq("no")
    {
        file.clear();
        println!("enter your file name (in target/debug/paths and must be .txt format)");
        std::io::stdin().read_line(&mut file).expect("invalid file");
        file.pop(); file.pop();
        assert!(file.contains(".txt"));
    }

    let path: String = dir + &file;

    println!("{}", path);

    let mut app: App = App::new("Draw with circles", 1000, 800);
    app.init_renderer(0.8, 1.0);
    app.init(path, 1.0 / 900.0, 880, 60);
    app.run();
}
