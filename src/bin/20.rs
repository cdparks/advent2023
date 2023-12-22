#![feature(let_chains)]
advent_of_code::solution!(20);

use std::collections::{HashMap, HashSet, VecDeque};

pub fn part_one(input: &str) -> Option<usize> {
    Some(Config::parse(input)?.step_times(1000))
}

pub fn part_two(input: &str) -> Option<usize> {
    Config::parse(input)?.step_till("rx")
}

#[derive(Debug, Clone)]
struct Config<'a> {
    modules: HashMap<&'a str, Module<'a>>,
    inputs: HashMap<&'a str, HashSet<&'a str>>,
    outputs: HashMap<&'a str, HashSet<&'a str>>,
}

impl<'a> Config<'a> {
    fn parse(input: &'a str) -> Option<Self> {
        let mut inputs: HashMap<_, HashSet<_>> = HashMap::new();
        let mut outputs: HashMap<_, HashSet<_>> = HashMap::new();

        let modules = input
            .lines()
            .flat_map(|line| {
                let (src, dsts) = line.split_once(" -> ")?;
                let module = Module::parse(src)?;
                let name = module.name();
                for dst in dsts.split(", ") {
                    outputs.entry(name).or_default().insert(dst);
                    inputs.entry(dst).or_default().insert(name);
                }
                Some((name, module))
            })
            .collect();

        Some(Self {
            modules,
            inputs,
            outputs,
        })
    }

    fn step_times(&mut self, presses: usize) -> usize {
        self.reset();
        let mut stats = Stats::new();
        while stats.presses < presses {
            self.step(&mut stats);
        }
        stats.lo * stats.hi
    }

    fn step_till(&mut self, target: &'a str) -> Option<usize> {
        self.reset();
        for name in &self.inputs[target] {
            let Some(Module::Conjunction { name, inputs, .. }) = self.modules.get(name) else {
                continue;
            };
            let mut stats = Stats::new_with(
                Some(name),
                &inputs.keys().copied().collect::<Vec<&'a str>>(),
            );
            loop {
                self.step(&mut stats);
                if stats.watch.values().all(|presses| *presses > 0) {
                    return Some(stats.watch.values().product());
                }
            }
        }
        None
    }

    fn reset(&mut self) {
        for module in self.modules.values_mut() {
            module.reset(&self.inputs)
        }
    }

    fn step(&mut self, stats: &mut Stats<'a>) {
        stats.press();
        let mut queue = VecDeque::from([(Pulse::Lo, "broadcaster", "button")]);
        while let Some((pulse, name, src)) = queue.pop_front() {
            let Some(module) = self.modules.get_mut(name) else {
                continue;
            };

            let Some((pulse, name)) = module.latch(src, pulse, stats) else {
                continue;
            };

            for dst in self.outputs[name].iter() {
                queue.push_back((pulse, dst, name));
                stats.inc(pulse);
            }
        }
    }
}

#[derive(Debug, Clone)]
enum Module<'a> {
    Broadcaster {
        name: &'a str,
    },
    FlipFlop {
        name: &'a str,
        on: bool,
    },
    Conjunction {
        name: &'a str,
        inputs: HashMap<&'a str, Pulse>,
    },
}

impl<'a> Module<'a> {
    fn parse(input: &'a str) -> Option<Self> {
        match input {
            "broadcaster" => Some(Self::Broadcaster { name: input }),
            _ if input.starts_with('%') => Some(Self::FlipFlop {
                name: &input[1..],
                on: false,
            }),
            _ if input.starts_with('&') => Some(Self::Conjunction {
                name: &input[1..],
                inputs: HashMap::new(),
            }),
            _ => None,
        }
    }

    fn name(&self) -> &'a str {
        match self {
            Self::Broadcaster { name } => name,
            Self::FlipFlop { name, .. } => name,
            Self::Conjunction { name, .. } => name,
        }
    }

    fn reset(&mut self, all_inputs: &HashMap<&'a str, HashSet<&'a str>>) {
        match self {
            Self::Broadcaster { .. } => (),
            Self::FlipFlop { on, .. } => *on = false,
            Self::Conjunction { name, inputs } => {
                inputs.clear();
                if let Some(srcs) = all_inputs.get(name) {
                    for src in srcs.iter() {
                        inputs.insert(src, Pulse::Lo);
                    }
                }
            }
        }
    }

    fn latch(
        &mut self,
        src: &'a str,
        pulse: Pulse,
        stats: &mut Stats<'a>,
    ) -> Option<(Pulse, &'a str)> {
        match self {
            Module::Broadcaster { name } => Some((pulse, name)),
            Module::FlipFlop { name, on } => {
                let Pulse::Lo = pulse else {
                    return None;
                };

                *on = !*on;
                let pulse = if *on { Pulse::Hi } else { Pulse::Lo };
                Some((pulse, name))
            }
            Module::Conjunction { name, inputs } => {
                inputs.insert(src, pulse);
                let pulse = if inputs.values().all(|pulse| *pulse == Pulse::Hi) {
                    Pulse::Lo
                } else {
                    Pulse::Hi
                };

                stats.watch(name, inputs);
                Some((pulse, name))
            }
        }
    }
}

#[derive(Debug, PartialEq, Copy, Clone)]
enum Pulse {
    Lo,
    Hi,
}

struct Stats<'a> {
    presses: usize,
    lo: usize,
    hi: usize,
    sink: Option<&'a str>,
    watch: HashMap<&'a str, usize>,
}

impl<'a> Stats<'a> {
    fn new() -> Self {
        Self::new_with(None, &[])
    }

    fn new_with(sink: Option<&'a str>, watch: &[&'a str]) -> Self {
        Self {
            presses: 0,
            lo: 0,
            hi: 0,
            sink,
            watch: watch.iter().map(|name| (*name, 0)).collect(),
        }
    }

    fn inc(&mut self, pulse: Pulse) {
        match pulse {
            Pulse::Lo => self.lo += 1,
            Pulse::Hi => self.hi += 1,
        }
    }

    fn press(&mut self) {
        self.lo += 1;
        self.presses += 1;
    }

    fn watch(&mut self, name: &'a str, inputs: &HashMap<&'a str, Pulse>) {
        if self.sink == Some(name) {
            for (input, pulse) in inputs.iter() {
                if *pulse == Pulse::Hi && self.watch[input] == 0 {
                    self.watch.insert(input, self.presses);
                }
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let result = part_one(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(32000000));
    }
}
