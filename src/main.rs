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

fn main() {
    let mut win_count = 0;
    loop {
        println!("じゃんけんの手を選んでください");
        println!("1: グー  2: チョキ  3: パー  0: ゲームを終了");

        let mut input = String::new();
        io::stdin().read_line(&mut input).expect("入力エラー");
        let input = input.trim();

        let num: i32 = match input.parse() {
            Ok(n) => n,
            Err(_) => {
                println!("指定された数字を入力してください。");
                continue;
            }
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

        let num = rand::thread_rng().gen_range(0..3);
        let computer = match num {
            0 => Hand::Rock,
            1 => Hand::Scissors,
            _ => Hand::Paper,
        };

        let player_str = to_str(player);
        let computer_str = to_str(computer);

        println!("じゃんけん...");
        thread::sleep(Duration::from_secs(1));
        println!("ぽん！");
        thread::sleep(Duration::from_secs(1));

        println!("自分の手: {}", player_str);
        println!("相手の手: {}", computer_str);
        thread::sleep(Duration::from_secs(1));

        match (player, computer) {
            (Hand::Rock, Hand::Scissors)
            | (Hand::Scissors, Hand::Paper)
            | (Hand::Paper, Hand::Rock) => {
                println!("あなたの勝ちです🎉");
                win_count += 1;
            }
            (Hand::Rock, Hand::Rock)
            | (Hand::Scissors, Hand::Scissors)
            | (Hand::Paper, Hand::Paper) => println!("あいこです🤝"),

            _ => {
                println!("あなたの負けです😢");
                println!("記録: {}連勝", win_count);
                break;
            }
        }
    }
}

fn to_str(hand: Hand) -> &'static str {
    match hand {
        Hand::Rock => "グー",
        Hand::Scissors => "チョキ",
        Hand::Paper => "パー",
    }
}
