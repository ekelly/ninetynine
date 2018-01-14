use cards::card::{Card, Suit};
use cards::deck::Deck;

/// When settings are implemented, they'll go here. For now, though, this is simply an empty struct
struct GameOptions {

}

/// A game consists of a list of scores for each 'Game', the current round, and the set of players
struct Game {

    /// A list of 'game' scores
    /// A player's total score is the sum of their scores for each individual game.
    scores: Vec<Vec<u16>>,

    /// The current round
    round: Round,

    /// How many players there are
    player_count: u8
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

    /// The current id of the dealer
    dealer: u8,

    /// The trump suit
    trump: Option<Suit>,

    /// Players have hands
    /// Bids are also tracked per player
    players: Vec<Player>,

    /// The ID of the player who's turn it is. This is only applicable in the play stage of the game
    current_player: u8,

    /// The current trick is the tail of the tricks vector
    tricks: Vec<Trick>,
}

struct Player {
    hand: Vec<Card>,
    bid: Option<Bid>,
    tricks_won: u8
}

struct Bid {
    bid: [Card; 3],
    bid_value: u8
}

struct Trick {

}

impl Round {

    /// Creates a new Round
    fn new(dealer: u8, player_count: u8) -> Self {
        if player_count != 3 && player_count != 4 {
            panic!("Can't handle player counts other than 3 or 4");
        }

        let mut deck = if player_count == 3 {
            Deck::new_shuffled_subset(Rank::Six, Rank::Ace);
        } else if player_count == 4 {
            Deck::new_shuffled();
        };
        let card_count_per_player = deck.size() / player_count;
        let mut players: Vec<Player> = Vec::with_capacity(player_count);
        for i in 0..player_count {
            players.append(Player {
                hand: deck.deal_n(card_count_per_player).unwrap(),
                bid: None,
                tricks_won: 0
            })
        }
        Round {
            dealer,
            trump: None,
            players,
            current_player: increment_player(dealer),
            tricks: vec![]
        }
    }

    pub fn submit_bid(&self, game: &Game, player_num: u8, cards: &[Card; 3]) -> Self {
        // TODO: This function really doesn't need mutation. Get rid of it.
        if player_num < 0 || player_num > game.player_count {
            panic!("Incorrect number of players!");
        }

        let mut new_bid: Vec<Card> = Vec::with_capacity(3);
        new_bid.clone_from_slice(cards);
        let player = game.round.players[player_num];
        let mut player_bid = player.bid;
        let mut player_hand = player.hand;

        if let Some(previous_bid) = player_bid {
            player_hand.append(previous_bid);
        };
        player_hand.retain(|&card| !new_bid.contains(card));
        player_bid.bid = new_bid.as_slice();
        player_bid.bid_value = 0; // TODO: Create function to calculate value

        bids[player_num] = cards;
        Round {
            dealer: self.dealer,
            trump: self.trump,
            current_player: self.current_player,
            players,
            tricks: vec![],
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
        // TODO: This entire function needs to be rewritten.
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
            players,
            tricks
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_round_new() {
    }
}

fn increment_player(player: u8) -> u8 {
    player + 1 % 3       // This is hardcoding the player count. We probably don't want to do that.
}