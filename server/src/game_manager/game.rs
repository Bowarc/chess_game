
pub struct Game{
    id: shared::id::Id,
    player1: Option<super::Player>,
    player2: Option<super::Player>,

    state: super::State,
}

impl Game{
    pub fn new() -> Self{
        Self{
            id:shared::id::Id::new(),
            player1: None,
            player2: None,
            state: super::State::default(),
        }
    }

    pub fn id(&self) -> shared::id::Id{
        self.id
    }

    pub fn connect_player(&mut self, new_player: super::Player) -> Result<(), shared::error::server::GameError>{
        if self.player1.is_none(){
            self.player1 = Some(new_player);
        }else if self.player2.is_none(){
            self.player2 = Some(new_player);
        }else{
            return Err(shared::error::server::GameError::SessionIsFull)
        }

        Ok(())

    }

    pub fn is_active(&self) -> bool{
        self.player1.is_some() || self.player2.is_some()
    }

    pub fn update(&mut self){

    }
}

