const VGA: *mut u16 = 0xB8000 as *mut u16;
const W: usize = 80; const H: usize = 25;
static mut C: usize = 0; static mut R: usize = 0;
unsafe fn scroll() {
    for r in 1..H { for c in 0..W { *VGA.add((r-1)*W+c) = *VGA.add(r*W+c); } }
    for c in 0..W { *VGA.add((H-1)*W+c) = b' ' as u16 | 0x0700; }
    R = H-1;
}
pub unsafe fn put(c: u8) {
    match c {
        b'\n' => { C=0; R+=1; if R>=H { scroll(); } }
        _ => { *VGA.add(R*W+C) = c as u16 | 0x0700; C+=1; if C>=W { C=0; R+=1; if R>=H { scroll(); } } }
    }
}
pub unsafe fn print(s: &str) { for b in s.bytes() { put(b); } }
pub unsafe fn banner(s: &str) {
    for b in s.bytes() { match b {
        b'\n' => { C=0; R+=1; if R>=H { scroll(); } }
        _ => { *VGA.add(R*W+C) = b as u16 | 0x0300; C+=1; if C>=W { C=0; R+=1; if R>=H { scroll(); } } }
    }}
}
pub unsafe fn hex(v: u64) { print("0x"); let mut s=60i32; while s>=0 { let n=((v>>s)&0xF) as u8; put(if n<10{b'0'+n}else{b'a'+n-10}); s-=4; } }
pub unsafe fn clear() { for i in 0..(W*H) { *VGA.add(i)=0x0720; } C=0; R=0; }
