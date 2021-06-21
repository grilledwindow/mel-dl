use async_trait::async_trait;
use std::thread::sleep;
use std::time::Duration;
use thirtyfour::{prelude::*, ScriptArgs};

use super::utils::*;
use crate::login::login;
use crate::mel::Module;

#[async_trait]
pub trait Download {
    async fn sign_in(&self) -> WebDriverResult<()>;
    async fn download_files(&self, module: &Module, week: u8) -> WebDriverResult<()>;
}
#[async_trait]
impl Download for WebDriver {
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

    async fn download_files(&self, module: &Module, week: u8) -> WebDriverResult<()> {
        // module.course_nth + 1: Don't know why, nth-child gets child 1 using index 2
        let module_css = format!(".term-_83_1:nth-child({})", module.course_nth + 1);
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

        self.query_wait_click(By::Id(&module.learning_materials_tab_id), &[])
            .await?;

        let c = format!(
            "#content_listContainer li:nth-child({}) a",
            module.learning_materials_nth + week - 1
        );

        sleep(Duration::from_millis(500));
        self.query_wait_click(By::Css(&c), &[]).await?;

        self.download_i_files().await?;

        // open new tab
        self.execute_script(r#"window.open("about:blank", target="_blank");"#)
            .await?;

        let handles = self.window_handles().await?;
        self.switch_to().window(&handles[1]).await?;

        for link in self.get_folder_links().await? {
            // switch tabs
            println!("{}", link);
            self.download_i_files().await?;
            self.get(link).await?;
            sleep(Duration::from_secs(3));
        }

        println!("Closing");
        // close new tab
        self.close().await?;

        // switch to default tab
        self.switch_to().window(&handles[0]).await?;
        sleep(Duration::from_millis(100));

        // go back to module, all weeks
        self.back().await?;

        // switch to main frame
        self.switch_to().default_content().await?;
        sleep(Duration::from_millis(100));

        // Go back to module homepage
        self.back().await?;
        
        // Go back to all modules
        self.back().await?;

        Ok(())
    }
}
