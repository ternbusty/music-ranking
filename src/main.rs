mod api;
mod data;
use data::parse;
use sqlx::MySqlPool;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let database_url = "mysql://root:password@localhost:13306/track_db";
    println!("Connecting to the database at {}", database_url);
    let pool = MySqlPool::connect(database_url).await?;

    println!("Parsing XML file...");

    match parse::read_xml_file("library.xml") {
        Ok(plist) => {
            // トラック情報を抽出
            let tracks = parse::extract_tracks(&plist);

            // トラック情報を表示
            if tracks.is_empty() {
                println!("No tracks found.");
            } else {
                println!("Tracks found:");
                parse::print_tracks(&tracks);
            }

            println!("Inserting tracks into database...");
            data::database::insert_tracks(&pool, tracks).await?;
        }
        Err(e) => {
            eprintln!("Failed to parse XML: {}", e);
        }
    }

    println!("Starting API server...");

    let routes = api::rankings::create_routes(pool);
    warp::serve(routes).run(([127, 0, 0, 1], 8000)).await;

    Ok(())
}
