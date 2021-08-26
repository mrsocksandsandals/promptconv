/* ---------------------------------------------------------- */
/* promptconv - A simple tool to convert bash prompts to zsh. */
/* ---------------------------------------------------------- */

pub const VERSION: &str = "0.1.3";

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

pub fn convert_prompt(prompt: String) -> String {
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
                                            n_chars.push('F');
                                            n_chars.push('{');
                                            if ansi_high {
                                                n_chars.push('8');
                                            } else {
                                                n_chars.push(*c);
                                            }
                                        }
                                    }

                                    '3' => {
                                        if chars[i - 1] == '0' || chars[i - 2] == '0' {
                                            // part of escape
                                            i += 1;
                                            continue;
                                        } else {
                                            if ansi_high == true {
                                                n_chars.push('F');
                                                n_chars.push('{');
                                                n_chars.push('1');
                                                n_chars.push('1');
                                            } else if chars[i + 1] == 'm' {
                                                n_chars.push('F');
                                                n_chars.push('{');
                                                n_chars.push('3');
                                                println!("created brown!");
                                                i += 1;
                                                continue;
                                            }
                                        }
                                    }

                                    '9' => {
                                        // 9 can be somewhat bothersome, as it shifts away from the terminfo sequence
                                        ansi_high = true;
                                    }

                                    _ => {
                                        // work on high ansi colours, and also kill '['
                                        if ansi_high {
                                            match c {
                                                '1' => {
                                                    n_chars.push('F');
                                                    n_chars.push('{');
                                                    n_chars.push('9');
                                                }

                                                '2' => {
                                                    n_chars.push('F');
                                                    n_chars.push('{');
                                                    n_chars.push('1');
                                                    n_chars.push('0');
                                                }

                                                '3' => {
                                                    n_chars.push('F');
                                                    n_chars.push('{');
                                                    n_chars.push('1');
                                                    n_chars.push('1');
                                                }

                                                '4' => {
                                                    n_chars.push('F');
                                                    n_chars.push('{');
                                                    n_chars.push('1');
                                                    n_chars.push('2');
                                                }

                                                '5' => {
                                                    n_chars.push('F');
                                                    n_chars.push('{');
                                                    n_chars.push('1');
                                                    n_chars.push('3');
                                                }

                                                '6' => {
                                                    n_chars.push('F');
                                                    n_chars.push('{');
                                                    n_chars.push('1');
                                                    n_chars.push('4');
                                                }

                                                '7' => {
                                                    n_chars.push('F');
                                                    n_chars.push('{');
                                                    n_chars.push('1');
                                                    n_chars.push('5');
                                                }

                                                _ => {
                                                    ansi_high = false;
                                                    if c != &'[' {
                                                        n_chars.push(*c);
                                                    }
                                                }
                                            }
                                        } else if !ansi_high {
                                            if c == &'[' {
                                                i += 1;
                                                continue;
                                            } else {
                                                n_chars.push('F');
                                                n_chars.push('{');
                                                println!("pushing char {}", chars[i]);
                                                n_chars.push(*c);
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

#[cfg(test)]
mod tests {
    use crate::*;

    // Library unit tests
    #[test]
    fn convert_default_redhat() {
        let test_prompt = String::from("[\\u@\\h \\W]\\$ ");
        let new_prompt = convert_prompt(test_prompt);
        assert_eq!(new_prompt, String::from("[%n@%m %1~]%(#.#.$) "));
    }

    #[test]
    fn convert_colourful_redhat() {
        let test_prompt = String::from("\\033[92m[\\u@\\033[94m\\h \\W\\033[92m]\\$ \\033[0m");
        let new_prompt = convert_prompt(test_prompt);
        assert_eq!(
            new_prompt,
            String::from("%F{10}[%n@%F{12}%m %1~%F{10}]%(#.#.$) %f")
        );
    }

    #[test]
    fn convert_gentoo_root() {
        let test_prompt = String::from("\\033[91m\\h \\033[94m\\w \\$ \\033[0m");
        let new_prompt = convert_prompt(test_prompt);
        assert_eq!(new_prompt, String::from("%F{9}%m %F{12}%~ %(#.#.$) %f"));
    }

    #[test]
    fn convert_gentoo_user() {
        let test_prompt = String::from("\\033[92m\\u@\\h \\033[94m\\w \\$ \\033[0m");
        let new_prompt = convert_prompt(test_prompt);
        assert_eq!(new_prompt, String::from("%F{10}%n@%m %F{12}%~ %(#.#.$) %f"));
    }

    #[test]
    fn colors() {
        let test_prompt = String::from(
            "\\033[30m\\033[31m\\033[32m\\033[33m\\033[34m\\033[35m\\033[36m\\033[37m\\033[0m",
        );
        let new_prompt = convert_prompt(test_prompt);
        assert_eq!(
            new_prompt,
            String::from("%F{0}%F{1}%F{2}%F{3}%F{4}%F{5}%F{6}%F{7}%f")
        );
        let test_prompt = String::from(
            "\\033[90m\\033[91m\\033[92m\\033[93m\\033[94m\\033[95m\\033[96m\\033[97m\\033[0m",
        );
        let new_prompt = convert_prompt(test_prompt);
        assert_eq!(
            new_prompt,
            String::from("%F{8}%F{9}%F{10}%F{11}%F{12}%F{13}%F{14}%F{15}%f")
        );
    }
}
