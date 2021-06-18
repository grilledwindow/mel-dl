use async_trait::async_trait;
use futures::future::{join_all};
use std::thread::sleep;
use std::time::Duration;
use std::{process, env};
use thirtyfour::{prelude::*, ScriptArgs};
use thirtyfour_query::{ElementQueryable, ElementWaitable};

use crate::mel::{Module, Modules::*};
use crate::login::*;

#[async_trait]
pub trait Sss {
    async fn query_wait<'a>(
        &'a self,
        elem: By<'a>,
        wait: &[u64],
    ) -> WebDriverResult<WebElement<'a>>;
    async fn query_wait_click<'a>(&'a self, elem: By<'a>, wait: &[u64]) -> WebDriverResult<()>;
    async fn alt_click(&self, element: &WebElement) -> WebDriverResult<()>;
    async fn sign_in(&self) -> WebDriverResult<()>;
    async fn download_files(&self, module: Module, week: u8) -> WebDriverResult<()>;
}

#[async_trait]
impl Sss for WebDriver {
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
    async fn sign_in(&self) -> WebDriverResult<()> {
        // Microsoft Login
        // Select email input, input email and enter
        self.query_wait(By::Css("input[type='email']"), &[])
            .await?
            .send_keys(TypingData::from(login().email) + Keys::Return)
            .await?;
        // Select password input, input password
        self.query_wait(By::Css("input[type='password']"), &[])
            .await?
            .send_keys(TypingData::from(login().password))
            .await?;

        // Select submit input, wait until stale
        self.query_wait(By::Css("input[type='submit']"), &[0, 0])
            .await?
            .wait_until()
            .stale()
            .await?;

        // Select submit input, click
        while let Err(_) = self
            .query_wait_click(By::Css("input[type='submit']"), &[])
            .await
        {
            println!("error3");
            continue;
        }

        // Prompt to save login details
        // Select submit input, click
        self.query_wait_click(By::Css("input[type='submit']"), &[])
            .await?;

        Ok(())
    }
    async fn download_files(&self, module: Module, week: u8) -> WebDriverResult<()> {
        // module.course_nth + 1: Don't know why, nth-child gets child 1 using index 2
        let module_css = format!(".term-_83_1:nth-child({})", module.course_nth + 1);
        let module_tab = self.query_wait(By::Css(&module_css), &[40, 1000]).await?;

        let _ret = self
            .execute_script_with_args(
                r#"
            let mod = document.querySelector(arguments[0])
            mod.scrollIntoViewIfNeeded(true);
            "#,
                ScriptArgs::new().push(&module_css)?, // ScriptArgs::new().push(&module_tab)?
            )
            .await?;

        sleep(Duration::from_millis(500));
        module_tab.click().await?;

        sleep(Duration::from_millis(1000));
        self.switch_to().frame_number(0).await?;

        while let Err(_) = self.query_wait_click(By::Id("menuPuller"), &[]).await {
            println!("error");
            continue;
        }

        self.query_wait_click(By::Id(&module.learning_materials_tab_id), &[])
            .await?;

        let week = week - 1;

        match module.module {
            AppliedAnalytics => {
                /* Structure
                - Learning Materials
                  - Week
                    --File
                */
                let c = format!(
                    "#content_listContainer li:nth-child({}) a",
                    module.learning_materials_nth + week
                );

                self.query_wait_click(By::Css(&c), &[]).await?;

                let download_links = self
                    .query(By::Css(
                        "#content_listContainer .detailsValue a:nth-child(2)",
                    ))
                    .wait(Duration::new(20, 0), Duration::from_millis(1000))
                    .all()
                    .await?;

                for link in download_links {
                    println!("- Downloading: {}...", link.text().await?);

                    // link.wait_until().clickable().await?;
                    self.alt_click(&link).await?;
                }

                // Go back to module
                self.back().await?;
            }
            WebAppDev => {
                /* Structure
                - Learning Materials
                  - Week
                    --File
                    - Content Folder
                      --File
                */

                let c = format!(
                    "#content_listContainer li:nth-child({}) a",
                    module.learning_materials_nth + week
                ); // lm > wk N

                self.query_wait_click(By::Css(&c), &[]).await?;

                // Getting all the file links
                /* let file_links = self
                    .query(By::Css("#content_listContainer .detailsValue ul li a"))
                    .wait(Duration::new(20, 0), Duration::from_millis(1000))
                    .all()
                    .await?;

                let file_links = join_all(file_links.into_iter().map(|a| async move {
                    if let Some(Some(href)) = a.get_attribute("href").await.ok() {
                        href
                    } else {
                        String::from("")
                    }
                }))
                .await; */

                let folder_links = self
                    .query(By::Css(
                        "#content_listContainer img[alt='Content Folder'] + div a",
                    ))
                    .wait(Duration::new(20, 0), Duration::from_millis(1000))
                    .all()
                    .await?;
                let n_max = folder_links.iter().count();
                let folder_links = join_all(folder_links.into_iter().map(|a| async move {
                    if let Some(Some(href)) = a.get_attribute("href").await.ok() {
                        href
                    } else {
                        String::from("")
                    }
                }))
                .await;

                for link in folder_links {
                    self.get(link).await?;
                    break;
                }

                // for link in file_links {
                //     println!("- Downloading: {:?}...", link);
                //     // self.alt_click(&link).await?;
                // }
                let mut li_nth: u8 = 0;
                let mut li_css: String;

                // while let Ok(li) = {
                //     li_nth += 1;
                //     li_css = format!("#content_listContainer li:nth-child({})", li_nth);
                //     self.query_wait(By::Css(&li_css), &[]).await
                // }
                for n in 1..n_max + 1 {
                    let li_css = format!("#content_listContainer > li:nth-child({})", n);
                    self.query(By::Css(&li_css))
                        .first()
                        .await?
                        .wait_until()
                        .clickable()
                        .await?;
                    let li = self.query_wait(By::Css(&li_css), &[]).await?;
                    println!("{}", li.text().await?);

                    let ul = self
                        .query(By::Css("#content_listContainer > li"))
                        .all()
                        .await?;
                    for li in ul {
                        println!("li: {}", li.text().await?);
                    }
                    // first if let causes slowd down
                    // if let Ok(download_link) = li.query(By::Css(".detailsValue a")).first().await {
                    //     println!("- Downloading: {}...", download_link.text().await?);
                    //     // download_link.click().await?;
                    // } else
                    sleep(Duration::from_millis(500));
                    if let Ok(content_folder) = li
                        .query(By::Css("img[alt='Content Folder'] + div a"))
                        .first()
                        .await
                    {
                        println!("- {}...", content_folder.text().await?);
                        content_folder.click().await?;

                        let a = self
                            .query(By::Css(".detailsValue a:nth-child(2)"))
                            .wait(Duration::new(20, 0), Duration::from_millis(1000))
                            .all()
                            .await?;
                        for aa in a {
                            println!("  - Downloading: {}", aa.text().await?);
                        }
                        sleep(Duration::from_millis(500));
                        self.back().await?;
                    }
                }

                let li_css = format!("#content_listContainer > li:nth-child(2) a");
                let li = self.query_wait(By::Css(&li_css), &[]).await?;
                li.click().await?;
                println!(" 1");
                sleep(Duration::from_millis(3000));

                self.back().await?;
                sleep(Duration::from_millis(3000));

                let li_css = format!("#content_listContainer > li:nth-child(3) a");
                let li = self.query_wait(By::Css(&li_css), &[]).await?;

                li.click().await?;
                println!(" 2");
                self.back().await?;
                // wk N > more

                // Go back to module, all weeks
                // self.back().await?;

                // // Switch to main frame
                // self.switch_to().default_content().await?;
                // sleep(Duration::from_millis(100));

                // // Go back to module homepage
                // self.back().await?;
                // // Go back to all modules
                // self.back().await?;
            }
            MobileAppDev => {}
            Calculus => {}
        }

        Ok(())
    }
}
