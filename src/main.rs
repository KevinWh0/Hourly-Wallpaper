use std::{error::Error, thread, time::Duration};

use crate::{
    api::wallpaper_api_config::{Resolution, WallpaperAPIConfBuilder},
    config_manager::get_random_query,
};
mod api;
mod config_manager;

#[tokio::main(flavor = "current_thread")]
async fn main() -> Result<(), Box<dyn Error>> {
    println!("Running");

    let config = config_manager::load_config().await?;

    loop {
        //For choosing a random query and tags
        let query = get_random_query(&config);
        let wallpaper_url = api::get_wallpaper_url(
            WallpaperAPIConfBuilder::new()
                .query(query.query)
                .categories(query.categories)
                .min_resolution(Resolution {
                    w: config.min_resolution.w,
                    h: config.min_resolution.h,
                })
                .api_key(config.api_key.clone())
                .purity(config.purity)
                .build(),
        );

        //Get the wallpaper json and get just the wallpaper element
        let wallpaper = api::get_wallpaper_url_from_request_url(&wallpaper_url?).await;

        //Make sure it found a wallpaper, download it if it did, then set it
        if let Some(wallpaper) = wallpaper? {
            if let Err(err) = wallpaper.download_file().await {
                //This can occur if the search query you have returns nothing. Or perhaps if you are unlucky
                println!("{}", err)
            } else {
                wallpaper.set_wallpaper()?;
            }
        }

        thread::sleep(Duration::new(
            (config.new_picture_delay as i32).try_into().unwrap(),
            0,
        ));
    }
}
