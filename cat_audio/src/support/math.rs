/// Максимальный общий делитель.
pub fn gcd(a:u32,b:u32)->u32{
    if b==0{
        a
    }
    else{
        gcd(b,a%b)
    }
}