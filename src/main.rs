mod trello;
mod lead;
mod writer;
mod user;
use text_io::{self, scan};
use std::str::FromStr;

#[tokio::main]
#[allow(non_snake_case)]
async fn main() {
    let mut mustContinue: bool = true;

    while mustContinue {
        let trelloService = trello::Trello::new();
        let mut leads: Vec<lead::Lead> = Vec::new();
        let mut user = user::User::new();
        let mut all: bool = false;

        match trelloService.getBoards().await {
            Ok(boards) => {
                for (i, board) in boards.iter().enumerate() {
                    println!("{}: {}", i+1, board.name);
                }

                println!("Type a number to choose a board: ");
                let boardNumber: usize;
                scan!("{}", boardNumber);

                if boardNumber > 0 && boardNumber <= boards.len() {
                    let selBoard = &boards[boardNumber - 1];
                    println!("Selected board: {}\n", selBoard.name);
                    user.setBoard(selBoard.clone());
                }
            }
            Err(e) => println!("Error while getting boards: {}", e),
        }

        match trelloService.getLists(user.getBoard().id).await {
            Ok(lists) => {
                for (i, list) in lists.iter().enumerate() {
                    println!("{}: {}", i+1, list.name);
                }

                println!("Type a list of numbers to choose lists (separated by commas): ");
                let listNumbers: String;
                scan!("{}", listNumbers);

                let selLists: Vec<usize> = listNumbers
                    .split(",")
                    .filter_map(|s| usize::from_str(s.trim()).ok())
                    .filter(|&num| num > 0 && num <= listNumbers.len())
                    .collect();

                all = selLists.len() == lists.len();

                for listNumber in selLists.iter() {
                    println!("You've selected: {}", lists[listNumber - 1].name);
                    let auxList = &lists[listNumber - 1];
                    user.setList(auxList.clone());
                }

            }
            Err(e) => println!("Error while getting lists: {}", e),
        }

        for list in user.getList() {
            match trelloService.getCards(list.id).await {
                Ok(cards) => {
                    if cards.is_empty() {
                        println!("No cards!");
                    } else {
                        for card in cards.iter() {
                            let lead = lead::Lead::extLead(&card.desc, &list.name);
                            leads.push(lead);
                        }
                    }
                }
                Err(e) => println!("Error while getting cards: {}", e),
            }
        }
        
        {
            let filename: String;
            if all {
                filename = format!("{}_ALL.csv", user.getBoard().name);
            } else {
                let names: Vec<String> = user.getList().iter().map(|list| list.name.clone()).collect();
                filename = format!("{}_{}.csv", user.getBoard().name, names.join("_"));
            }
            match writer::Writer::saveLeads(&leads, &filename) {
                Ok(()) => println!("\nCSV file successfully created!"),
                Err(e) => println!("Error while saving CSV: {}", e),
            }
        }

        println!("\nDo you want to perform another query? [y]es or [n]o");
        let ans: String;
        scan!("{}", ans);
        mustContinue = ans.to_lowercase() == "y";
    }
    println!("\nThanks for using - By Pedro Masteguin");
}
