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

    /// Disables evaluation of the 16 ANSI escape colours
    #[structopt(long)]
    disable_ansi: bool,
}

fn main() {
    println!("promptconv v{}\nNOTE: This program will ONLY convert ANSI escape sequences in the prompt.\n", VERSION);

    let args = Args::from_args();
    let prompt = &args.prompt_string;
    // Print back the Bash prompt.
    println!("Bash prompt: \"{}\"", prompt);
    // Convert to ZSH.
    let zprompt = convert_prompt(prompt.to_string());
    println!("Zsh prompt:  \"{}\"", zprompt);
}

fn convert_prompt(prompt: String) -> String {
    let chars: Vec<char> = prompt.chars().collect();
    let mut n_chars: Vec<char> = vec![];
    let mut escaping: bool = false;
    let mut i: usize = 0; // used to find the next char(s)
    for c in &chars {
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
                            match &Args::from_args().disable_ansi {
                                true => {
                                    if chars[i + 1] == '0' {
                                        // Maybe?
                                        if chars[i + 2] == '3' && chars[i + 3] == '3' {
                                            // Yes! Now, evaluate ANSI escape and convert to a colour.
                                        }
                                    }
                                }

                                false => {
                                    // continue on to the next iteration
                                    continue;
                                }
                            }
                            n_chars.push('\\');
                            n_chars.push(*c);
                        }
                    }
                    escaping = false; // Done.
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
