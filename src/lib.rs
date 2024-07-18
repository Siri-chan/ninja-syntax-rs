//! Rust port of ninja-build/ninja/misc/ninja_syntax.py
//
// Note that this is emphatically not a required piece of Ninja, just a convenient
// utility for a meta-build system written in Rust.
//
// The Original Project was written in Python and is Copyright (C) Google, 2011.
// Licensed under the Apache Software License 2.0
// This project is a complete rewrite, but did take heavy reference from the
// source material, and so as per the license, the original license is included
// in the source tree as `orig_COPYING`.

use regex::Regex;
use std::collections::HashMap;

/*
import re
import textwrap
from io import TextIOWrapper
from typing import Dict, List, Match, Optional, Tuple, Union
*/
use textwrap::{Options, WordSplitter};

pub fn escape_path(word: &String) -> String {
    word.replace("$ ", "$$ ")
        .replace(" ", "$ ")
        .replace(":", "$:")
}
pub struct RuleDescriptor {
    name: String,
    command: String,
    description: Option<String>,
    depfile: Option<String>,
    generator: bool,
    pool: Option<String>,
    restat: bool,
    rspfile: Option<String>,
    rspfile_content: Option<String>,
    deps: Vec<String>,
}
impl Default for RuleDescriptor {
    fn default() -> Self {
        Self {
            name: String::from("New Rule"),
            command: String::from("echo"),
            description: None,
            depfile: None,
            generator: false,
            pool: None,
            restat: false,
            rspfile: None,
            rspfile_content: None,
            deps: Vec::new(),
        }
    }
}
pub fn escape(string: String) -> Result<String, String> {
    if string.contains('\n') {
        return Err(String::from("Ninja syntax does not allow newlines"));
    }
    let string = string.replace("$", "$$");
    Ok(string)
}

pub fn expand(
    string: &mut String,
    vars: HashMap<String, String>,
    local_vars: HashMap<String, String>,
) -> String {
    /* Expand a string containing $vars as Ninja would.

    Note: doesn't handle the full Ninja variable syntax, but it's enough
    to make configure.py's use of it work.
    */
    fn exp(
        m: regex::Captures,
        vars: &HashMap<String, String>,
        local_vars: &HashMap<String, String>,
    ) -> String {
        let var = m.get(1).unwrap().as_str();
        if var == "$" {
            return "$".to_string();
        }
        return local_vars
            .get(&var.to_owned())
            .unwrap_or(vars.get(&var.to_owned()).unwrap_or(&String::new()))
            .to_owned();
    }
    let re = Regex::new(r#"\$(\$|\w*)"#).unwrap();
    let string_clone = string.clone();
    let matches: Vec<_> = re.captures_iter(&string_clone).collect();
    for matc/*h*/ in matches {
        let full_match = matc.get(0).unwrap();
        string.replace_range(full_match.start()..full_match.end(), &exp(matc, &vars, &local_vars))
    }
    return string.clone();
}

pub struct Writer {
    pub width: u16,
    pub output: Box<dyn std::io::Write>, //todo figure out types
}
impl Default for Writer {
    fn default() -> Self {
        Self {
            width: 78,
            output: Box::new(std::io::stdout()),
        }
    }
}
impl Writer {
    pub fn newline(&mut self) {
        write!(&mut self.output, "\n").unwrap();
    }
    pub fn comment(&mut self, text: String) {
        for line in textwrap::wrap(
            &text,
            Options::new((self.width - 2) as usize)
                .break_words(false)
                .word_splitter(WordSplitter::NoHyphenation),
        ) {
            write!(&mut self.output, "#{}\n", line).unwrap()
        }
    }
    pub fn variable(&mut self, key: String, value: Option<VariableValue>, indent: Option<usize>) {
        if value.is_none() {
            return;
        }
        // unwrap safely
        let value = value.unwrap();
        match value {
            VariableValue::ListStr(list) => self.line(
                &list
                    .into_iter()
                    .filter(|s| !s.is_empty())
                    .fold(format!("{} =", key), |acc, s| format!("{acc} {s}")),
                indent,
            ),
            VariableValue::Int(v) => self.line(&format!("{} = {}", key, v), indent),
            VariableValue::Bool(v) => self.line(&format!("{} = {}", key, v), indent),
            VariableValue::Float(v) => self.line(&format!("{} = {}", key, v), indent),
            VariableValue::Str(v) => self.line(&v, indent),
        };
    }
    pub fn pool(&mut self, name: String, depth: isize) {
        self.line(&format!("pool {}", name), None);
        self.variable(
            "depth".to_string(),
            Some(VariableValue::Int(depth)),
            Some(1),
        )
    }

    pub fn rule(&mut self, rule_descriptor: RuleDescriptor) {
        self.line(&format!("rule {}", rule_descriptor.name), None);
        self.variable(
            "command".to_string(),
            Some(VariableValue::Str(rule_descriptor.command)),
            Some(1),
        );
        if let Some(descr) = rule_descriptor.description {
            self.variable(
                "description".to_string(),
                Some(VariableValue::Str(descr)),
                Some(1),
            )
        }
        if let Some(depfile) = rule_descriptor.depfile {
            self.variable(
                "depfile".to_string(),
                Some(VariableValue::Str(depfile)),
                Some(1),
            )
        }
        if rule_descriptor.generator {
            self.variable(
                "generator".to_string(),
                Some(VariableValue::Int(1)),
                Some(1),
            )
        }
        if let Some(pool) = rule_descriptor.pool {
            self.variable("pool".to_string(), Some(VariableValue::Str(pool)), Some(1))
        }
        if rule_descriptor.restat {
            self.variable("restat".to_string(), Some(VariableValue::Int(1)), Some(1))
        }
        if let Some(rspfile) = rule_descriptor.rspfile {
            self.variable(
                "rspfile".to_string(),
                Some(VariableValue::Str(rspfile)),
                Some(1),
            )
        }
        if let Some(rspfile_content) = rule_descriptor.rspfile_content {
            self.variable(
                "rspfile_content".to_string(),
                Some(VariableValue::Str(rspfile_content)),
                Some(1),
            )
        }
        if !rule_descriptor.deps.is_empty() {
            self.variable(
                "deps".to_string(),
                Some(VariableValue::ListStr(rule_descriptor.deps)),
                Some(1),
            )
        }
    }
    pub fn build(
        &mut self,
        outputs: Vec<String>,
        rule: String,
        inputs: Vec<String>,
        implicit: &mut Vec<String>,
        order_only: &mut Vec<String>,
        variables: Option<HashMap<String, Vec<String>>>,
        implicit_outputs: Vec<String>,
        pool: Option<String>,
        dyndep: Option<String>,
    ) -> Vec<String> {
        let mut out_outputs: Vec<String> = outputs
            .clone()
            .into_iter()
            .map(|x| escape_path(&x))
            .collect();
        let mut all_inputs: Vec<String> = inputs.into_iter().map(|x| escape_path(&x)).collect();

        if !implicit.is_empty() {
            let mut implicit = implicit.into_iter().map(|x| escape_path(&x)).collect();
            all_inputs.push("|".to_string());
            all_inputs.append(&mut implicit);
        }
        if !order_only.is_empty() {
            let mut order_only = order_only.into_iter().map(|x| escape_path(x)).collect();
            all_inputs.push("||".to_string());
            all_inputs.append(&mut order_only);
        }
        if !implicit_outputs.is_empty() {
            let mut implicit_outputs = implicit_outputs
                .into_iter()
                .map(|x| escape_path(&x))
                .collect();
            out_outputs.push("|".to_string());
            out_outputs.append(&mut implicit_outputs);
        }

        self.line(
            &format!(
                "build {}: {} {}",
                out_outputs
                    .into_iter()
                    .reduce(|acc, x| format!("{} {}", acc, x))
                    .unwrap(),
                rule,
                all_inputs
                    .into_iter()
                    .reduce(|acc, x| format!("{} {}", acc, x))
                    .unwrap()
            ),
            None,
        );

        if let Some(pool) = pool {
            self.line(&format!("  pool = {}", pool), None);
        }
        if let Some(dyndep) = dyndep {
            self.line(&format!("  dyndep = {}", dyndep), None);
        }

        if let Some(variables) = variables {
            let iter = variables.into_iter();

            for (key, val) in iter {
                if val.is_empty() {
                    self.variable(key, None, Some(1));
                } else if val.len() == 1 {
                    self.variable(key, Some(VariableValue::Str(val[0].clone())), Some(1));
                } else {
                    self.variable(key, Some(VariableValue::ListStr(val)), Some(1));
                }
            }
        }

        outputs
    }
    pub fn include(&mut self, path: String) {
        self.line(&format!("include {}", path), None);
    }

    pub fn subninja(&mut self, path: String) {
        self.line(&format!("subninja {}", path), None);
    }

    pub fn default(&mut self, paths: Vec<String>) {
        self.line(
            &format!(
                "default {}",
                paths
                    .into_iter()
                    .reduce(|acc, x| format!("{} {}", acc, x))
                    .unwrap()
            ),
            None,
        );
    }

    pub fn close(mut self) {
        let _ = self.output.flush().unwrap();
    }

    /// Write 'text' word-wrapped at self.width characters.
    fn line(&mut self, text: &String, indent: Option<usize>) {
        let indent = indent.unwrap_or(0);
        let mut leading_space = "  ".repeat(indent);
        let mut string;
        let mut text = text;
        while leading_space.len() + text.len() > self.width as usize {
            // The text is too wide; wrap if possible.
            // Find the rightmost space that would obey our width constraint and
            // that's not an escaped space.
            let available_space = (self.width as usize) - leading_space.len() - " $".len();
            let mut space = available_space;
            loop {
                let res = text.clone().split_at(space as usize).0.rfind(' ');
                if res.is_none() {
                    space = 0;
                    break;
                }
                space = res.unwrap();
                if Self::count_dollars_before_index(text, space) % 2 == 0 {
                    break;
                }
            }
            if space == 0 {
                // No such space; just use the first unescaped space we can find.
                space = available_space - 1;
                loop {
                    space = text.clone().split_at(space).1.find(' ').unwrap();
                    if Self::count_dollars_before_index(text, space) % 2 == 0 {
                        break;
                    }
                }
            }
            // Give up.
            write!(&mut self.output, "{}{} $\n", leading_space, &text[0..space]).unwrap();
            string = text.split_at(space + 1).1.to_string();
            text = &string;

            // Subsequent lines are continuations, so indent them.
            leading_space = "  ".repeat(indent + 2)
        }

        write!(&mut self.output, "{}{}\n", leading_space, text).unwrap();
    }
    /// Returns the number of `$` directly before s[i]
    fn count_dollars_before_index(s: &String, i: usize) -> usize {
        let mut q = s.clone().split_at(i).0.chars().rev().collect::<String>();
        let mut i = 0;
        while q.find('$') == Some(0) {
            q.pop();
            i += 1;
        }
        i
    }
}

pub enum VariableValue {
    Bool(bool),
    Int(isize),
    Float(f64),
    Str(String),
    ListStr(Vec<String>),
}
