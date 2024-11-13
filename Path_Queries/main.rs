fn main() {
    // Scanners in Rust
    let stdin = std::io::stdin();
    let mut scanner = Scanner::new(stdin.lock());

    let n: u32 = scanner.next();
    let q: u32 = scanner.next();

    let mut values::<i32> = vec![0i32; n];

    for i in 1..n {
        values[i] = scanner.next();
    }

    println!("{}, {}", n, q);
}

//Implementatio of a scanner in Rust, the code is not mine. Credits to: 
pub struct Scanner<R: std::io::BufRead> {
    reader: R,
    buffer: Vec<String>,
}

impl<R: std::io::BufRead> Scanner<R> {
    pub fn new(reader: R) -> Self {
        Scanner {
            reader,
            buffer: Vec::new(),
        }
    }
    pub fn next<T: std::str::FromStr>(&mut self) -> T {
        loop {
            if let Some(token) = self.buffer.pop() {
                return token.parse().ok().expect("Token Parsing Failure");
            }
            let mut line = String::new();
            self.reader.read_line(&mut line).expect("Line Read Failure");
            self.buffer = line
                .split_ascii_whitespace()
                .rev()
                .map(String::from)
                .collect();
        }
    }
}