use druid::widget::Label;
use druid::{AppLauncher, Widget, WindowDesc};
     
/*[0,0,0,0,0,0,0,0,0], 
  [0,0,0,0,0,0,0,0,0], 
  [0,0,0,0,0,0,0,0,0], 
  [0,0,0,0,0,0,0,0,0], 
  [0,0,0,0,0,0,0,0,0], 
  [0,0,0,0,0,0,0,0,0], 
  [0,0,0,0,0,0,0,0,0], 
  [0,0,0,0,0,0,0,0,0],
  [0,0,0,0,0,0,0,0,0]*/


static mut SODUKO:[[i8;9];9]=[
[0,0,0,2,6,0,7,0,1], 
[6,8,0,0,9,0,0,9,0], 
[1,9,0,0,0,4,5,0,0], 
[0,0,0,5,0,7,0,0,0], 
[0,0,4,0,0,0,1,0,0], 
[0,9,0,0,0,0,0,0,0], 
[5,0,0,0,0,0,0,7,3], 
[0,0,2,0,1,0,0,0,0],
[0,0,0,0,4,0,0,0,9]];

fn show_soduko () {
    println!("-----------------------------");
    for i in 0..9{
        for j in 0..9{
            unsafe{
                print!("{} ",SODUKO[i][j] );
            }
        }
        println!("");
        }
     
}
fn check_free(x:&mut i8, y:&mut i8) -> bool {
    for i in 0..9{ 
        for j in 0..9{
            unsafe{
                if SODUKO[i][j] == 0{
                    *x = i as i8;
                    *y = j as i8;
                    //show_soduko();
                    //println!(" i= {} j = {} ",i,j);
                    return true;
                }
            }
        }
    }
    return false;
}
fn check_true(x:i8, y:i8,n:i8) -> bool{
    unsafe{
        for i in 0..9{
            if n == SODUKO[x as usize][i] || n == SODUKO[i][y as usize]{
                return false;
            }
        }
        let new_x = (x/3)*3;
        let new_y = (y/3)*3;
        for i in new_x..new_x+3 { 
            for j in new_y..new_y+3{
                if n == SODUKO[i as usize][j as usize]{
                    //show_soduko();
                    //println!("n ={} i= {} j = {} ", n,i,j);
                    return false;
                }
            }
        }
    }
    return true
}
fn solve() -> bool {
    let mut x:i8 = 0;
    let mut y:i8 = 0;
    
    if !check_free(&mut x, &mut y){
        show_soduko();
        println!("ajab");

        return false;
    }

    for i in 1..10{
        if check_true(x ,y ,i ){
            unsafe {
                SODUKO[x as usize][y as usize] = i;
                    if solve(){
                        return true
                    }else{
                        SODUKO[x as usize][y as usize] = 0;
                    }  
            } 
        }
    }
    return false
}

fn game() {
   show_soduko();
    if solve() {
        show_soduko();
        println!("tamam shod !!?");
    }else{
        show_soduko();
        println!("kharabe !!?");
    }
}


fn build_ui() -> impl Widget<()> {
    Label::new("Salam").align_left()
}


fn main() {
    print!("slam !!?");
     let main_window = WindowDesc::new(build_ui())
        .window_size((600.0, 400.0))
        .title("Sodoku solver !!?");
    let initial_data = ();

    AppLauncher::with_window(main_window)
        .launch(initial_data)
        .expect("Failed to launch application");    
}

