use notion_client::{NotionClientError, endpoints::{Client, search::title::{request::{Filter, SearchByTitleRequest, Sort, SortDirection, Timestamp}, response::PageOrDatabase}}, objects::page::Page};

use crate::environment::Environment;

pub struct Collector {
    api: Client,
    env: Environment
}

impl Collector {
    pub fn new(env: Environment) -> Result<Self, NotionClientError> {
        let api = Client::new(env.notion_api_token.clone(), None)?;

        Ok(Self { api: api, env })
    }

    pub async fn fetch_root_page(&self) -> Result<Page, NotionClientError> {
        let page = self.api.pages.retrieve_a_page(&self.env.notion_root_page, None).await?;
        Ok(page)
    }

    pub async fn fetch(&self) -> Result<(), NotionClientError> {
        let request = SearchByTitleRequest {
            query: None,
            filter: None,
            sort: Some(Sort {
                direction: SortDirection::Descending,
                timestamp: Timestamp::LastEditedTime
            }),
            start_cursor: None,
            page_size: None
        };

        let response = self.api.search.search_by_title(request).await?;

        for result in response.results {
            match result {
                PageOrDatabase::Page(page) => {
                    println!("page: {:?}", page.public_url);
                }
                _ => {}
            }
        }

        Ok(())
    }
}