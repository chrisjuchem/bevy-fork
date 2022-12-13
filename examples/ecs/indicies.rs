use bevy::prelude::*;
use bevy_internal::input::InputPlugin;
use rand::{thread_rng, Rng};
use std::cmp::Ordering;

const PLAYERS_PER_TEAM: u32 = 4;
const TEAMS: u32 = 2048;
const SCOREBOARD_SIZE: usize = 10;

#[derive(Component, PartialEq, Eq)]
struct Player {
    name: String,
    alive: bool,
    team: u32,
    score: u32,
}

impl PartialOrd for Player {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(match self.score.cmp(&(other.score)) {
            Ordering::Equal => self.name.cmp(&(other.name)),
            ord @ _ => ord,
        })
    }
}

impl Ord for Player {
    fn cmp(&self, other: &Self) -> Ordering {
        self.partial_cmp(other).unwrap()
    }
}

#[derive(Resource, Default)]
struct Hill {
    king: Option<u32>,
}

fn main() {
    App::new()
        .add_plugins(MinimalPlugins)
        .add_plugin(InputPlugin)
        .add_startup_system(setup)
        .init_resource::<Hill>()
        .add_system(handle_input)
        .add_system(update_score_naive)
        // .add_system(print_scoreboard_naive)
        .run();
}

fn setup(mut commands: Commands) {
    for team in 0..TEAMS {
        for player in 0..PLAYERS_PER_TEAM {
            commands.spawn(Player {
                name: format!("{team}p{player}"),
                alive: true,
                team,
                score: 0,
            });
        }
    }
}

fn handle_input(
    keys: Res<Input<KeyCode>>,
    mut hill: ResMut<Hill>,
    mut players: Query<&mut Player>,
) {
    // A new team captures the hill
    if keys.just_pressed(KeyCode::Space) {
        hill.king = Some(thread_rng().gen_range(0..TEAMS));
        println!("Cap")
    }

    // Kill 1/3 players randomly
    if keys.just_pressed(KeyCode::K) {
        let mut rng = thread_rng();
        for mut player in &mut players {
            player.alive = rng.gen_bool(0.666);
        }
    }

    // Change Index update strategy
}

fn update_score_naive(mut players: Query<&mut Player>, hill: Res<Hill>) {
    if let Some(king) = hill.king {
        print!("Score");
        for mut player in &mut players {
            if player.alive && player.team == king {
                player.score += 1;
            }
        }
    }
}

fn print_scoreboard_naive(players: Query<&Player>) {
    let mut top: Vec<&Player> = vec![];
    let mut threshold: u32 = 0;
    for mut player in &players {
        // if top.len() < SCOREBOARD_SIZE || player.score > threshold {
        //     for i in 0..SCOREBOARD_SIZE {
        //         if player > top[i] {
        //             top.insert(i, player);
        //             break;
        //         }
        //     }
        //     top.truncate(SCOREBOARD_SIZE);
        //     threshold = top[top.len()-1].score
        // }

        if top.len() < SCOREBOARD_SIZE {
            top.push(player);
            continue;
        }

        for top_player in &mut top {
            // Keep swapping the "current" player into the list. The smallest score should fall off
            if player > top_player {
                std::mem::swap::<&Player>(&mut player, top_player);
            }
        }
    }

    top.sort();
    top.reverse();

    for p in top {
        println!("{} {} {}", p.name, p.team, p.score)
    }
    println!();
}

/*=====
enum IndexUpdateTiming {
    /// Best for frequent changes, infrequent use
    BeforeUse,
    /// Best for frequent use, infrequent changes
    AfterUpdate,
}

trait ComponentIndex<V, C> {
    const UPDATE_TIMING: IndexUpdateTiming;

    fn compute_value(component: &C) -> V;
}

trait RangeComponentIndex // TODO GAT?



struct Index<I> where I: ComponentIndex<V, C> {}
impl Index<I> where I: ComponentIndex<V, C> {
    fn get(value: V) -> HashSet<C>; //todo retrun arbitrary worldquery
    fn get_range()
}


// cannonical player index
impl ComponentIndex<u32, Player> for Player {}
enum SpecialPlayerIndex {}
impl ComponentIndex<u32, Player> for SpecialPlayerIndex {}

fn system(idx: Index<Player>, idx2: Index<SpecialPlayerIndex>) {}
fn const_index(cidx: Query<(Entity, ConstIndex<&Player, const u32 7777>)>) {}//??????



 */
