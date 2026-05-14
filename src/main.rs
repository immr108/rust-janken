use rand::Rng;
use std::io;
use std::thread;
use std::time::Duration;

#[derive(Copy, Clone)]
enum Hand {
    Rock,
    Scissors,
    Paper,
}

impl Hand {
    fn to_str(self) -> &'static str {
        match self {
            Hand::Rock => "グー",
            Hand::Scissors => "チョキ",
            Hand::Paper => "パー",
        }
    }
}

#[derive(Copy, Clone)]
enum Outcome {
    Win,
    Draw,
    Lose,
}

fn main() {
    let mut win_count = 0;
    loop {
        println!("じゃんけんの手を選んでください");
        println!("1: グー  2: チョキ  3: パー  0: ゲームを終了");

        let num = match get_input() {
            Some(n) => n,
            None => continue,
        };

        let player = match num {
            1 => Hand::Rock,
            2 => Hand::Scissors,
            3 => Hand::Paper,
            0 => {
                println!("ゲームを終了します。");
                println!("記録: {}連勝", win_count);
                break;
            }
            _ => {
                println!("0〜3の数字を入力してください");
                continue;
            }
        };

        let computer = get_computer_hand();

        let player_str = player.to_str();
        let computer_str = computer.to_str();

        println!("じゃんけん...");
        thread::sleep(Duration::from_secs(1));
        println!("ぽん！");
        thread::sleep(Duration::from_secs(1));

        println!("自分の手: {}", player_str);
        println!("相手の手: {}", computer_str);
        thread::sleep(Duration::from_secs(1));

        match judge(player, computer) {
            Outcome::Win => {
                println!("あなたの勝ちです");
                win_count += 1;
            }
            Outcome::Draw => {
                println!("あいこです");
            }
            Outcome::Lose => {
                println!("あなたの負けです");
                println!("記録: {}連勝", win_count);
                break;
            }
        }
    }
}

fn get_computer_hand() -> Hand {
    let num = rand::thread_rng().gen_range(0..3);
    match num {
        0 => Hand::Rock,
        1 => Hand::Scissors,
        _ => Hand::Paper,
    }
}

fn judge(player: Hand, computer: Hand) -> Outcome {
    match (player, computer) {
        (Hand::Rock, Hand::Scissors)
        | (Hand::Scissors, Hand::Paper)
        | (Hand::Paper, Hand::Rock) => Outcome::Win,
        (Hand::Rock, Hand::Rock)
        | (Hand::Scissors, Hand::Scissors)
        | (Hand::Paper, Hand::Paper) => Outcome::Draw,

        _ => Outcome::Lose,
    }
}

fn get_input() -> Option<i32> {
    let mut input = String::new();
    io::stdin().read_line(&mut input).expect("入力エラー");
    let input = input.trim();
    match input.parse::<i32>() {
        Ok(n) => Some(n),
        Err(_) => {
            println!("指定された数字を入力してください");
            None
        }
    }
}
