use super::utils::*;
use crate::config::{Module, Settings};
use crate::directory;
use async_trait::async_trait;
use serde_json::Value;
use std::fs::read_to_string;
use std::thread::sleep;
use std::time::Duration;
use thirtyfour::{prelude::*, ScriptArgs};

use super::utils::FolderType;

#[async_trait]
pub trait Download {
    async fn sign_in(&self) -> WebDriverResult<()>;
    async fn download_files(
        &mut self,
        module: &Module,
        mut folder: u8,
        settings: &Settings,
    ) -> WebDriverResult<()>;
}

#[async_trait]
impl Download for WebDriver {
    async fn sign_in(&self) -> WebDriverResult<()> {
        let credentials_data =
            read_to_string("credentials.json").expect("Error reading credentials.json");
        let credentials: Value =
            serde_json::from_str(&credentials_data).expect("Error deserializing credentials.json");

        let email = credentials
            .get("email")
            .expect("email field missing in credentials.json")
            .as_str()
            .unwrap();
        let password = credentials
            .get("password")
            .expect("password field missing in credentials.json")
            .as_str()
            .unwrap();

        // Microsoft Login
        // Select email input, input email and enter
        self.query_wait(By::Css("input[type='email']"), &[])
            .await?
            .send_keys(TypingData::from(email) + Keys::Return)
            .await?;
        // Select password input, input password
        self.query_wait(By::Css("input[type='password']"), &[])
            .await?
            .send_keys(TypingData::from(password))
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

        // Close popup
        if let Ok(popup_close) = self
            .query(By::Css(".ms-Dialog-main .button"))
            .nowait()
            .ignore_errors(true)
            .first()
            .await
        {
            self.alt_click(&popup_close).await?;
        }

        Ok(())
    }

    async fn download_files(
        &mut self,
        module: &Module,
        mut folder: u8,
        settings: &Settings,
    ) -> WebDriverResult<()> {
        // module.course_nth + 1: Don't know why, nth-child gets child 1 using index 2
        let module_css = format!(".default-group:nth-child({})", module.course_nth + 1);
        let module_tab = self.query_wait(By::Css(&module_css), &[40, 1000]).await?;

        let _ret = self
            .execute_script_with_args(
                r#"
                    document
                        .querySelector(arguments[0])
                        .scrollIntoViewIfNeeded(true);
                "#,
                ScriptArgs::new().push(&module_css)?,
            )
            .await?;

        sleep(Duration::from_millis(500));
        module_tab.click().await?;

        sleep(Duration::from_millis(2000));
        self.switch_to().frame_number(0).await?;

        self.query_wait_click(By::Id("menuPuller"), &[]).await?;
        self.open_learning_materials().await?;
        let all_weeks_links = self.get_links(&FolderType::FolderWeek).await?;
        let n_weeks = all_weeks_links.iter().count() as u8;

        for f in &all_weeks_links {
            println!("folder: {:?}", f.link);
        }

        folder += module.week_start;
        if folder > n_weeks {
            println!("Folder {0} is greater than the number of folders found: {1}.\nDownloading folder {1}", folder, n_weeks);
            folder = n_weeks;
        }

        folder -= 1;
        println!("{}", &all_weeks_links[folder as usize].link);
        sleep(Duration::from_millis(500));
        self.get(format!(
            "{}{}",
            "https://mel.np.edu.sg", &all_weeks_links[folder as usize].link
        ))
        .await?;
        println!("Week: {:?}", &all_weeks_links[folder as usize].week_no);

        // let file_links = self.get_links(&FolderType::File).await?;
        // for f in &file_links {
        //     println!("file: {:?}", f.link);
        // }

        // let item_links = self.get_links(&FolderType::Item).await?;
        // for f in &item_links {
        //     println!("item: {:?}", f.link);
        // }
        self._download_files(FolderType::File).await?;
        self._download_files(FolderType::Item).await?;
        /*
        let folder_links = self.get_folder_links(false).await?;
        if folder_links.iter().count() == 0 {
            directory::move_files(
                settings,
                module.name,
                &all_weeks_links[folder as usize].week_no,
            )?;
            self.get("https://mel.np.edu.sg/ultra/course").await?;
            return Ok(());
        }

        // open new tab
        self.execute_script(r#"window.open("about:blank", target="_blank");"#)
            .await?;

        let handles = self.window_handles().await?;
        self.switch_to().window(&handles[1]).await?;

        for folder in folder_links {
            self.download_i_files().await?;
            self.get(folder.link).await?;
            sleep(Duration::from_secs(1));
        }

        // close new tab
        self.close().await?;

        // switch to default tab
        self.switch_to().window(&handles[0]).await?;
        sleep(Duration::from_millis(100));

        // go back to learning materials
        self.back().await?;

        // switch to main frame
        self.switch_to().default_content().await?;
        sleep(Duration::from_millis(100));

        // Go back to module homepage
        self.back().await?;

        // Go back to all modules
        self.back().await?; */

        directory::move_files(
            settings,
            module.name,
            &all_weeks_links[folder as usize].week_no,
        )?;

        Ok(())
    }
}
