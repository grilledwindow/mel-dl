use async_trait::async_trait;
use futures::future::join_all;
use std::thread::sleep;
use std::time::Duration;
use stringmatch::StringMatch;
use thirtyfour::prelude::*;

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
    async fn download_i_files(&self) -> WebDriverResult<()>;
    async fn get_folder_links(&self, is_week: bool) -> WebDriverResult<Vec<String>>;
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

    async fn download_i_files(&self) -> WebDriverResult<()> {
        let file_links = self
            .query(By::Css(
                "#content_listContainer .detailsValue :nth-child(2)",
            ))
            .all()
            .await?;
        for link in file_links {
            println!("- Downloading: {}", link.text().await?);
            self.alt_click(&link).await?;
        }
        Ok(())
    }

    async fn get_folder_links(&self, is_week: bool) -> WebDriverResult<Vec<String>> {
        let text = StringMatch::new(if is_week { "week" } else { "" })
            .case_insensitive()
            .partial();

        let folder_links = self
            .query(By::Css(
                "#content_listContainer img[alt='Content Folder'] + div a",
            ))
            .wait(Duration::new(20, 0), Duration::from_millis(1000))
            .with_text(text)
            .all()
            .await?;

        let folder_links = join_all(
            folder_links
                .into_iter()
                .map(|a| async move { a.get_attribute("href").await.ok().unwrap().unwrap() }),
        )
        .await;

        Ok(folder_links)
    }
}
