use anyhow::Result;
use presentation_graphql::GraphQL;
use std::io::Write;

fn main() -> Result<()> {
    let schema_output_path = "./schema.graphql";
    let sdl = GraphQL::sdl();

    let mut sdl_file = if std::path::Path::new(schema_output_path).exists() {
        std::fs::OpenOptions::new()
            .write(true)
            .open(schema_output_path)
    } else {
        std::fs::File::create(schema_output_path)
    }?;

    write!(&mut sdl_file, "{}", &sdl).expect("write failed");

    println!("write schema in {}", schema_output_path);
    Ok(())
}
