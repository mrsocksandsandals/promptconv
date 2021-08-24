/* ---------------------------------------------------------- */
/* promptconv - A simple tool to convert bash prompts to zsh. */
/* ---------------------------------------------------------- */

/* MIT License
 *
 * Copyright (c) 2021 Brett I. <notronaldmcdonalds>
 *
 * Permission is hereby granted, free of charge, to any person obtaining a copy
 * of this software and associated documentation files (the "Software"), to deal
 * in the Software without restriction, including without limitation the rights
 * to use, copy, modify, merge, publish, distribute, sublicense, and/or sell
 * copies of the Software, and to permit persons to whom the Software is
 * furnished to do so, subject to the following conditions:
 *
 * The above copyright notice and this permission notice shall be included in all
 * copies or substantial portions of the Software.
 *
 * THE SOFTWARE IS PROVIDED "AS IS", WITHOUT WARRANTY OF ANY KIND, EXPRESS OR
 * IMPLIED, INCLUDING BUT NOT LIMITED TO THE WARRANTIES OF MERCHANTABILITY,
 * FITNESS FOR A PARTICULAR PURPOSE AND NONINFRINGEMENT. IN NO EVENT SHALL THE
 * AUTHORS OR COPYRIGHT HOLDERS BE LIABLE FOR ANY CLAIM, DAMAGES OR OTHER
 * LIABILITY, WHETHER IN AN ACTION OF CONTRACT, TORT OR OTHERWISE, ARISING FROM,
 * OUT OF OR IN CONNECTION WITH THE SOFTWARE OR THE USE OR OTHER DEALINGS IN THE
 * SOFTWARE.
*/

// Define constants
const VERSION: &str = "0.1.1";

// Import stuff
use structopt::StructOpt;

// Arguments structure
#[derive(Debug, StructOpt)]
#[structopt(name = "promptconv", about = "A simple Bash > Zsh prompt converter.")]
struct Args {
    /// Bash prompt to convert
    #[structopt(name = "prompt")]
    prompt_string: String,

    /// Print less text, only printing the zsh prompt when done
    #[structopt(short, long)]
    quiet: bool,
}

fn main() {

    let args = Args::from_args();
    let prompt = &args.prompt_string;
    if !args.quiet {
        println!(
            "promptconv v{}\nNOTE: This program will ONLY convert foreground ANSI colours.\n",
            VERSION
        );
    }
    // Print back the Bash prompt.
    if ! args.quiet {
        println!("Bash prompt: \"{}\"", prompt);
    }
    // Convert to ZSH.
    let zprompt = convert_prompt(prompt.to_string());
    if args.quiet {
        println!("\"{}\"", zprompt)
    } else {
        println!("Zsh prompt:  \"{}\"", zprompt);
    }
}

fn convert_prompt(prompt: String) -> String {
    let chars: Vec<char> = prompt.chars().collect();
    let mut n_chars: Vec<char> = vec![];
    let mut escaping: bool = false;
    let mut ansi: bool = false;
    let mut ansi_high: bool = false; // evaluating high ansi foreground (9x)
    let mut i: usize = 0; // used to find the next char(s)
    'assemble_prompt: for c in &chars {
        match c {
            // Check if translating an escape.
            '\\' => {
                escaping = true;
            }

            _ => {
                if escaping == true {
                    // We are translating an escape, check the character.
                    match c {
                        'u' => {
                            // Username
                            n_chars.push('%');
                            n_chars.push('n');
                        }

                        'h' => {
                            // Hostname
                            n_chars.push('%');
                            n_chars.push('m');
                        }

                        'H' => {
                            // Full hostname
                            n_chars.push('%');
                            n_chars.push('M');
                        }

                        'j' => {
                            // Jobs
                            n_chars.push('%');
                            n_chars.push('j');
                        }

                        'l' => {
                            // Basename of current line
                            n_chars.push('%');
                            n_chars.push('y');
                        }

                        't' => {
                            // 24-hour clock with seconds
                            n_chars.push('%');
                            n_chars.push('*');
                        }

                        '@' => {
                            // 12-hour AM/PM
                            n_chars.push('%');
                            n_chars.push('@');
                        }

                        'w' => {
                            // Current working directory
                            n_chars.push('%');
                            n_chars.push('~');
                        }

                        'W' => {
                            // Truncated cwd
                            n_chars.push('%');
                            n_chars.push('1');
                            n_chars.push('~');
                        }

                        '!' => {
                            // History event number
                            n_chars.push('%');
                            n_chars.push('!');
                        }

                        '$' => {
                            // Permission sign
                            n_chars.push('%');
                            n_chars.push('(');
                            n_chars.push('#');
                            n_chars.push('.');
                            n_chars.push('#');
                            n_chars.push('.');
                            n_chars.push('$');
                            n_chars.push(')');
                        }

                        _ => {
                            // Check if we are looking at an ANSI escape, aftering ensuring it is actually enabled.
                            if ansi == true {
                                // we are working on an ANSI escape.
                                match c {
                                    'm' => {
                                        // end of sequence
                                        if !(chars[i - 1] == '[' && chars[i] == '0') {
                                            n_chars.push('}');
                                        }
                                        ansi = false;
                                    }

                                    '0' => {
                                        if chars[i - 1] == '\\' {
                                            // part of escape
                                            i += 1;
                                            continue;
                                        } else {
                                            // check if the only char in escape. if so, then reset.
                                            if chars[i - 1] == '[' && chars[i + 1] == 'm' {
                                                n_chars.push('f');
                                                for _i in 1..3 {
                                                    continue 'assemble_prompt;
                                                }
                                            }
                                            n_chars.push(*c);
                                        }
                                    }

                                    '3' => {
                                        if chars[i - 1] == '0' || chars[i - 2] == '0' {
                                            // part of escape
                                            i += 1;
                                            continue;
                                        } else {
                                            if ansi_high == true {
                                                n_chars.push('1');
                                                n_chars.push('1');
                                            } else {
                                                n_chars.push(*c);
                                            }
                                        }
                                    }

                                    '9' => {
                                        // 9 can be somewhat bothersome, as it shifts away from the terminfo sequence
                                        ansi_high = true;
                                    }

                                    _ => {
                                        // work on high ansi colours, and also kill '['
                                        if ansi_high == true {
                                            match c {
                                                '1' => {
                                                    n_chars.push('F');
                                                    n_chars.push('{');
                                                    n_chars.push('9');
                                                }

                                                '2' => {
                                                    n_chars.push('1');
                                                    n_chars.push('0');
                                                }

                                                '3' => {
                                                    n_chars.push('1');
                                                    n_chars.push('1');
                                                }

                                                '4' => {
                                                    n_chars.push('1');
                                                    n_chars.push('2');
                                                }

                                                '5' => {
                                                    n_chars.push('1');
                                                    n_chars.push('3');
                                                }

                                                '6' => {
                                                    n_chars.push('1');
                                                    n_chars.push('4');
                                                }

                                                '7' => {
                                                    n_chars.push('1');
                                                    n_chars.push('5');
                                                }

                                                _ => {
                                                    ansi_high = false;
                                                    if c != &'[' {
                                                        n_chars.push(*c);
                                                    } else {
                                                        // is '[', check if the escape is 0m or not (91m also has trouble check that here)
                                                        if !(chars[i + 1] == '0' && chars[i + 2] == 'm') {
                                                            n_chars.push('F');
                                                            n_chars.push('{');
                                                        }
                                                    }
                                                }
                                            }
                                        }
                                    }
                                }
                            }
                            if chars[i] == '0' {
                                // Maybe?
                                    if chars[i + 1] == '3' && chars[i + 1] == '3' {
                                        // Yes! Now, evaluate ANSI escape and convert to a colour.
                                        n_chars.push('%');
                                        ansi = true;
                                    }
                            }
                        }
                    }
                    if !ansi {
                        // make sure it doesn't end us early
                        escaping = false; // Done.
                    }
                } else {
                    // Push the character.
                    n_chars.push(*c);
                }
            }
        }
        i += 1;
    }
    let n_prompt = n_chars.into_iter().collect();
    n_prompt
}
