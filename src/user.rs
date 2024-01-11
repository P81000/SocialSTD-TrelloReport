use crate::trello;

#[allow(non_snake_case)]
pub struct User {
    opBoard: trello::Board,
    opList: Vec<trello::List>,
}

#[allow(non_snake_case)]
impl User {
    pub fn new() -> Self {
        User {
            opBoard: Default::default(),
            opList: Default::default(),
        }
    }

    pub fn setBoard(&mut self, board: trello::Board) {
        self.opBoard = board;
    }

    pub fn setList(&mut self, list: trello::List) {
        self.opList.push(list);
    }

    pub fn getBoard(&self) -> trello::Board {
        self.opBoard.clone()
    }

    pub fn getList(&self) -> Vec<trello::List> {
        self.opList.clone()
    }
}

