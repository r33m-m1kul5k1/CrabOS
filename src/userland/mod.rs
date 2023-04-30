//! this module includes userland processes
mod syscalls;
use syscalls::log;

pub static LOGO: &'static str = r"
$$$$$$\                     $$\        $$$$$$\   $$$$$$\  
$$  __$$\                    $$ |      $$  __$$\ $$  __$$\ 
$$ /  \__| $$$$$$\  $$$$$$\  $$$$$$$\  $$ /  $$ |$$ /  \__|
$$ |      $$  __$$\ \____$$\ $$  __$$\ $$ |  $$ |\$$$$$$\  
$$ |      $$ |  \__|$$$$$$$ |$$ |  $$ |$$ |  $$ | \____$$\ 
$$ |  $$\ $$ |     $$  __$$ |$$ |  $$ |$$ |  $$ |$$\   $$ |
\$$$$$$  |$$ |     \$$$$$$$ |$$$$$$$  | $$$$$$  |\$$$$$$  |
\______/ \__|      \_______|\_______/  \______/  \______/ 
                   (\/) (°,,,,°) (\/)                                     
";

pub fn user_main() -> ! {
    log(LOGO).unwrap();
    loop {}
}
