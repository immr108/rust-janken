use std::io;
use std::thread;
use std::time::Duration;
use rand::Rng;

enum Hand {
    Rock,
    Scissors,
    Paper,
}

fn main() {
    let mut win_count = 0;
    loop{
        
    
    println!("じゃんけんの手を入力してください");
    println!("入力例: グー、チョキ、パー");

    let mut input = String::new();
    io::stdin().read_line(&mut input).expect("入力エラー");
    let input = input.trim();

    let player = if input == "グー" {
        Hand::Rock
    } else if input == "チョキ" {
        Hand::Scissors
    } else if input == "パー" {
        Hand::Paper
    } else {
        println!("グー・チョキ・パーで入力してください");
        continue;
    };

    let num = rand::thread_rng().gen_range(0..3);
    let computer = match num {
        0 => Hand::Rock,
        1 => Hand::Scissors,
        _ => Hand::Paper,
    };

    let player_str = match player {
        Hand::Rock     => "グー",
        Hand::Scissors => "チョキ",
        Hand::Paper    => "パー",
    };
    let computer_str = match computer {
        Hand::Rock     => "グー",
        Hand::Scissors => "チョキ",
        Hand::Paper    => "パー",
    };

    println!("じゃんけん...");
    thread::sleep(Duration::from_secs(1));
    println!("ぽん！");
    thread::sleep(Duration::from_secs(1));

    println!("自分の手: {}", player_str);
    println!("相手の手: {}", computer_str);
    thread::sleep(Duration::from_secs(1));

    match (player, computer) {
        (Hand::Rock,     Hand::Scissors) |
        (Hand::Scissors, Hand::Paper)    |
        (Hand::Paper,    Hand::Rock)     => {
            println!("あなたの勝ちです🎉");
            win_count += 1;
        }
        (Hand::Rock,     Hand::Rock)     |
        (Hand::Scissors, Hand::Scissors) |
        (Hand::Paper,    Hand::Paper)    => println!("あいこです🤝"),

        _ => {
            println!("あなたの負けです😢");
            println!("記録: {}連勝",win_count);
            break;
    }
    }}}