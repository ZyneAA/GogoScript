use crux::Rei;

mod frontend;
mod backend;
mod tools;
mod crux;

#[cfg(test)]
mod tests;

fn main() -> Result<(), Box<dyn std::error::Error>> {

    Rei::Ayanami()

}

