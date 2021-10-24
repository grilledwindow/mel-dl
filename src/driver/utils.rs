use async_trait::async_trait;
use futures::future::join_all;
use regex::Regex;
use std::time::Duration;
use stringmatch::StringMatch;
use thirtyfour::prelude::*;

#[derive(Debug)]
pub struct Folder {
    pub week_no: String,
    pub link: String,
}

pub struct Links {
    pub item_links: Vec<String>,
    pub file_links: Vec<String>,
    pub folder_links: Vec<String>,
    pub folder_week_links: Vec<Folder>,
}

#[derive(Debug)]
pub enum FolderType {
    FolderWeek,
    Folder,
    Item,
    File,
}

// impl ImgAlt {
//     fn to_string(&self) -> String {
//         match self {
//             ImgAlt::Folder => String::from("folder"),
//             ImgAlt::Item => String::from("item"),
//             ImgAlt::File => String::from("file"),            
//         }
//     }
// }

// use ImgAlt;

#[async_trait]
pub trait Utils {
    async fn query_wait<'a>(
        &'a self,
        elem: By<'a>,
        wait: &[u64],
    ) -> WebDriverResult<WebElement<'a>>;
    async fn query_wait_click<'a>(&'a self, elem: By<'a>, wait: &[u64]) -> WebDriverResult<()>;
    async fn alt_click(&self, element: &WebElement) -> WebDriverResult<()>;
    async fn open_learning_materials(&self) -> WebDriverResult<()>;
    async fn _download_files(&self, links: Vec<Folder>) -> WebDriverResult<()>;
    async fn get_links(&self, folder_type: &FolderType) -> WebDriverResult<Vec<Folder>>;
}

#[async_trait]
impl Utils for WebDriver {
    async fn query_wait<'a>(
        &'a self,
        elem: By<'a>,
        wait: &[u64],
    ) -> WebDriverResult<WebElement<'a>> {
        let mut timeout = 20; // in seconds
        let mut interval = 500; // in milliseconds
        if wait.len() == 2 {
            timeout = wait[0];
            interval = wait[1];
        };

        let q = self
            .query(elem)
            .wait(
                Duration::from_secs(timeout),
                Duration::from_millis(interval),
            )
            .first()
            .await?;

        Ok(q)
    }

    async fn query_wait_click<'a>(&'a self, elem: By<'a>, wait: &[u64]) -> WebDriverResult<()> {
        self.query_wait(elem, wait).await?.click().await?;
        Ok(())
    }

    async fn alt_click(&self, element: &WebElement) -> WebDriverResult<()> {
        self.action_chain()
            .key_down(Keys::Alt)
            .click_element(element)
            .perform()
            .await?;

        Ok(())
    }

    async fn open_learning_materials(&self) -> WebDriverResult<()> {
        self.execute_script(
            r#"
            document
                .querySelector("\#courseMenuPalette_contents span[title='Learning Materials']")
                .click();
            "#,
        )
        .await?;

        Ok(())
    }

    async fn _download_files(&self, links: Vec<Folder>) -> WebDriverResult<()> {
        // for link in links {
        //     println!("- Downloading: {}", link.link);
        //     self.alt_click(&link.link).await?;
        // }
        Ok(())
    }

    async fn get_links(&self, folder_type: &FolderType) -> WebDriverResult<Vec<Folder>> {
        let text = StringMatch::new(match folder_type {
            FolderType::FolderWeek => "week",
            _ => "",
        })
            .case_insensitive()
            .partial();

        let css = match folder_type {
            FolderType::Item => "#content_listContainer img[alt='Item'] + div + div div.detailsValue a:nth-child(2)",
            FolderType::File => "#content_listContainer img[alt='File'] + div div a:nth-child(2)",
            _ => "#content_listContainer img[alt='Content Folder'] + div a",
        };
        
        let links = self
            .query(By::Css(&css))
            .wait(Duration::new(20, 0), Duration::from_millis(1000))
            .with_text(text)
            .all()
            .await?;

        let links = join_all(links.into_iter().map(|a| async move {
            let link = a.get_attribute("href").await.ok().unwrap().unwrap();

            let text = if let FolderType::FolderWeek = folder_type {
                let re = Regex::new(r"[Ww]eek\s*\d+").unwrap();
                let text = a
                    .find_element(By::Tag("span"))
                    .await
                    .ok()
                    .unwrap()
                    .text()
                    .await
                    .ok()
                    .unwrap();
                let week = re.find(&text).unwrap();
    
                let re = Regex::new(r"\d+").unwrap();
                let text = &text[..week.end()];
                let week = re.find(text).unwrap();
    
                let text = &text[week.start()..week.end()];

                String::from(text)
            } else {
                String::new()
            };

            Folder {
                week_no: text,
                link,
            }
        }))
        .await;

        Ok(links)
    }
}
