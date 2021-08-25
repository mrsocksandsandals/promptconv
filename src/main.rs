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

// Import stuff
use promptconv::*;
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
