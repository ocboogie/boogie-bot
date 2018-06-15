use prelude::*;
use std::collections::HashMap;

lazy_static! {
  static ref LETTER_TO_EMOJI: HashMap<char, Vec<&'static str>> = {
    let mut m = HashMap::new();
    m.insert('a', vec![emoji!(":regional_indicator_a:"), emoji!(":a:")]);
    m.insert('b', vec![emoji!(":regional_indicator_b:"), emoji!(":b:")]);
    m.insert(
      'c',
      vec![emoji!(":regional_indicator_c:"), emoji!(":copyright:")],
    );
    m.insert('d', vec![emoji!(":regional_indicator_d:")]);
    m.insert(
      'e',
      vec![emoji!(":regional_indicator_e:"), emoji!(":three:")],
    );
    m.insert('f', vec![emoji!(":regional_indicator_f:")]);
    m.insert('g', vec![emoji!(":regional_indicator_g:")]);
    m.insert('h', vec![emoji!(":regional_indicator_h:")]);
    m.insert(
      'i',
      vec![
        emoji!(":regional_indicator_i:"),
        emoji!(":information_source:"),
        emoji!(":spoon:"),
      ],
    );
    m.insert('j', vec![emoji!(":regional_indicator_j:")]);
    m.insert('k', vec![emoji!(":regional_indicator_k:")]);
    m.insert('l', vec![emoji!(":regional_indicator_l:")]);
    m.insert(
      'm',
      vec![
        emoji!(":regional_indicator_m:"),
        emoji!(":m:"),
        emoji!(":part_alternation_mark:"),
      ],
    );
    m.insert('n', vec![emoji!(":regional_indicator_n:")]);
    m.insert(
      'o',
      vec![
        emoji!(":regional_indicator_o:"),
        emoji!(":o:"),
        emoji!(":o2:"),
        emoji!(":zero:"),
      ],
    );
    m.insert(
      'p',
      vec![emoji!(":regional_indicator_p:"), emoji!(":parking:")],
    );
    m.insert('q', vec![emoji!(":regional_indicator_q:")]);
    m.insert(
      'r',
      vec![emoji!(":regional_indicator_r:"), emoji!(":registered:")],
    );
    m.insert(
      's',
      vec![
        emoji!(":regional_indicator_s:"),
        emoji!(":heavy_dollar_sign:"),
      ],
    );
    m.insert(
      't',
      vec![emoji!(":regional_indicator_t:"), emoji!(":cross:")],
    );
    m.insert('u', vec![emoji!(":regional_indicator_u:")]);
    m.insert(
      'v',
      vec![emoji!(":regional_indicator_v:"), emoji!(":aries:")],
    );
    m.insert('w', vec![emoji!(":regional_indicator_w:")]);
    m.insert(
      'x',
      vec![
        emoji!(":regional_indicator_x:"),
        emoji!(":x:"),
        emoji!(":heavy_multiplication_x:"),
        emoji!(":negative_squared_cross_mark:"),
      ],
    );
    m.insert('y', vec![emoji!(":regional_indicator_y:")]);
    m.insert('z', vec![emoji!(":regional_indicator_z:")]);

    m.insert('0', vec![emoji!(":zero:")]);
    m.insert(
      '1',
      vec![emoji!(":one:"), emoji!(":spoon:"), emoji!(":first_place:")],
    );
    m.insert('2', vec![emoji!(":two:"), emoji!(":second_place:")]);
    m.insert('3', vec![emoji!(":three:"), emoji!(":third_place:")]);
    m.insert('4', vec![emoji!(":four:")]);
    m.insert('5', vec![emoji!(":five:")]);
    m.insert('6', vec![emoji!(":six:")]);
    m.insert('7', vec![emoji!(":seven:")]);
    m.insert('8', vec![emoji!(":eight:"), emoji!(":8ball:")]);
    m.insert('9', vec![emoji!(":nine:")]);
    m.insert(
      ' ',
      vec![
        emoji!(":white_large_square:"),
        emoji!(":black_large_square:"),
        emoji!(":stop_button:"),
        emoji!(":white_square_button:"),
        emoji!(":black_square_button:"),
      ],
    );
    m.insert('?', vec![emoji!(":question:"), emoji!(":grey_question:")]);
    m.insert(
      '!',
      vec![
        emoji!(":exclamation:"),
        emoji!(":grey_exclamation:"),
        emoji!(":bangbang:"),
      ],
    );
    m.insert('#', vec![emoji!(":hash:")]);
    m.insert('*', vec![emoji!(":asterisk:")]);
    m.insert('+', vec![emoji!(":heavy_plus_sign:"), emoji!(":battery:")]);
    m.insert('-', vec![emoji!(":heavy_minus_sign:")]);
    m.insert(
      '$',
      vec![
        emoji!(":heavy_dollar_sign:"),
        emoji!(":moneybag:"),
        emoji!(":dollar:"),
      ],
    );
    m.insert(
      '\'',
      vec![
        emoji!(":black_small_square:"),
        emoji!(":white_small_square:"),
      ],
    );
    m
  };
}

struct SpellCommand;

impl Command for SpellCommand {
  fn execute(&self, _ctx: &mut Context, msg: &Message, mut args: Args) -> CommandResult {
    let target_message_id = args.single::<u64>().expect("Could not find message");
    let target_message = msg
      .channel_id
      .message(target_message_id)
      .expect("Could not find message");

    let mut used_emoji: Vec<&'static str> = Vec::new();

    for letter in args.full().chars() {
      if let Some(emojis) = LETTER_TO_EMOJI.get(&letter) {
        for ref_emoji in emojis {
          if used_emoji.contains(ref_emoji) {
            continue;
          }

          let emoji = *ref_emoji;
          // let target_message_clone = target_message.clone();
          // thread::spawn(move || {
          // let _ =
          target_message.react(emoji).unwrap();
          // });
          info!("Added reaction");
          used_emoji.push(emoji);
          break;
        }
      }
    }
    Ok(())
  }
}

pub fn register_command(sf: StandardFramework) -> StandardFramework {
  sf.command("spell", |c| c.cmd(SpellCommand))
}
