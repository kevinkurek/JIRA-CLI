use std::rc::Rc;

use itertools::Itertools;
use anyhow::Result;
use anyhow::anyhow;

use crate::db::JiraDatabase;
use crate::models::Action;

mod page_helpers;
use page_helpers::*;

pub trait Page {
    fn draw_page(&self) -> Result<()>;
    fn handle_input(&self, input: &str) -> Result<Option<Action>>;
}

pub struct HomePage {
    pub db: Rc<JiraDatabase>
}

impl Page for HomePage {
    fn draw_page(&self) -> Result<()> {
        println!("----------------------------- EPICS -----------------------------");
        println!("     id     |               name               |      status      ");

        // print out epics using get_column_string(). also make sure the epics are sorted by id
        let epics = self.db.read_db()?.epics;
        for id in epics.keys().sorted() {
            let epic = &epics[id];
            let id_col = get_column_string(&id.to_string(), 11);
            let name_col = get_column_string(&epic.name.to_string(), 32);
            let status_col = get_column_string(&epic.name.to_string(), 17);
            println!("{} | {} | {}", id_col, name_col, status_col);
        }

        println!();
        println!();

        println!("[q] quit | [c] create epic | [:id:] navigate to epic");

        Ok(())
    }

    fn handle_input(&self, input: &str) -> Result<Option<Action>> {
        // match against the user input and return the corresponding action. If the user input was invalid return None.

        let epics = self.db.read_db()?.epics;

        match input {
            "q" => Ok(Some(Action::Exit)),
            "c" => Ok(Some(Action::CreateEpic)),

            // TODO
            input => {
                if let Ok(epic_id) = input.parse::<u32>() {
                    if epics.contains_key(&epic_id) {
                        return Ok(Some(Action::NavigateToEpicDetail {epic_id}))
                    }
                }
                Ok(None)
            }
        }
    }
}