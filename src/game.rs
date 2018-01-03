use cards::card::{Card, Suit};

/// When settings are implemented, they'll go here. For now, though, this is simply an empty struct
struct GameOptions {

}

/// A game consists of a list of scores for each 'Game', the current round, and the set of players
struct Game {
    scores: Vec<[u16; 3]>,
    round: Round,
    players: [Player; 3],
}

/*
 * Each round has:
 * - A dealer
 * - The current player
 * - An incomplete trick
 * - The number of tricks taken by each player
 * - Trump
 * - Players
 */
struct Round {
    dealer: u8,
    trump: Option<Suit>,
    current_player: u8,
    current_trick: [Option<Card>; 3],
    current_trick_count: [u8; 3],
    current_bid: [Option<[Card; 3]>; 3],
}

impl Round {

    pub fn new(dealer: u8) -> Self {
        Round {
            dealer,
            trump: None,
            current_player: increment_player(dealer),
            current_trick: [None; 3],
            current_trick_count: [0; 3],
            current_bid: [[None; 3]; 3],
        }
    }

    pub fn submit_bid(&self, game: &Game, player_num: u8, cards: [Card; 3]) -> Self {
        let mut bids: [[Card; 3]; 3] = self.current_bid;
        bids[player_num] = cards;
        Round {
            dealer: self.dealer,
            trump: self.trump,
            current_player: self.current_player,
            current_trick: self.current_trick,
            current_trick_count: self.current_trick_count,
            current_bid: bids,
        }
    }

    fn determine_trick_winner(&self, player: u8, card: Card) -> Option<u8> {
        let mut current_winner: (u8, Option<Card>) = (-1, None);
        for (option_card, index) in self.current_trick.iter().enumerate() {
            match option_card {
                Some(card) => {
                    if current_winner[0] < 0 {
                        current_winner = (index, option_card)
                    } else {

                    }
                },
                None => {
                    if player != index {
                        return None
                    }
                }
            }
        }
    }



    pub fn play_card(&self, card: Card) -> Self {
        let trick_winner: Option<u8> = self.determine_trick_winner(self.current_player, card);
        let current_player = match trick_winner {
            Some(next_player) => next_player,
            None => increment_player(self.current_player)
        };
        let current_trick_count = match trick_winner {
            Some(winner) => {
                let mut tricks: [u8; 3] = [0; 3];
                tricks.clone_from_slice(self.current_trick_count);
                tricks[winner] = tricks[winner] + 1;
                tricks
            },
            None => self.current_trick_count
        };
        let current_trick = match trick_winner {
            Some(winner) => [None; 3],
            None => {
                let mut trick: [Option<Card>; 3] = [None; 3];
                trick.clone_from_slice(self.current_trick);
                trick[self.current_player] = Some(card);
                trick
            },
        };
        Round {
            dealer: self.dealer,
            trump: self.trump,
            current_player,
            current_trick,
            current_trick_count,
            current_bid: bids,
        }
    }
}

fn increment_player(player: u8) -> u8 {
    player + 1 % 3
}