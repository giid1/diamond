
use std::io;
use std::io::Write;
fn main() {
    diamond(5);
}

fn diamond(line: i32)
{
    let mut n = 1;
    let mut i = 0;
    let mut show = 1;
    let mut space = 0;
    let mut half = line/2+1;
    let mut tmp = line;

    if line < 0 || line % 2 == 0
    {
        println!("Not cool");

        return;
    }

    while n < line
    {
        while i < show
        {
            while space != half
            {
                if half < 0
                {
                    half = 1; 
                }
                print!(" ");
                io::stdout().flush().unwrap();
                space += 1;
            }
            print!("*");
            io::stdout().flush().unwrap();
            i += 1;

        }
        if show < line
        {
            println!();
            i = 0;
            space = 0;
            show += 2;
            half -= 1;
        }
        n += 1;
    }

    while n != 0
    {
        while i < tmp
        {
             //space-ek számolásának algoritmizálása
            while space != half
            {
                if half < 0
                {
                    half = 1; 
                }
                print!(" ");
                io::stdout().flush().unwrap();
                space += 1;
            }
            print!("*");
            io::stdout().flush().unwrap();
            i += 1;
        }

        if show > n
        {
            println!();
            i = 0;
            tmp -= 2;
            space = 0;
            half += 1;
        }

        n -= 1;
    }




}
