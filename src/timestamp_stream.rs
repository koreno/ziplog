use crate::timestamp_kinds::TimestampKind;
use chrono::{DateTime, Utc};

use std::io::{BufReader, BufRead, Lines};
use std::rc::Rc;
use std::io::Read;

pub type PossibleTimestampKinds = Rc<Vec<Rc<TimestampKind>>>;

pub struct TimestampedStream<F: Read> {
    reader: Lines<BufReader<F>>,
    prefix: String,
    filler: String,
    scratch_string: String,
    possible_kinds: PossibleTimestampKinds,
    kind: Option<Rc<TimestampKind>>,
}

impl<F> TimestampedStream<F>
    where F: Read
{
    pub fn new(file: F, prefix: String, possible_kinds: PossibleTimestampKinds) -> Self {
        let reader = BufReader::new(file).lines();
        let filler = std::iter::repeat(" ").take(prefix.len()).collect::<String>();

        TimestampedStream {
            reader, prefix, kind: None, filler, possible_kinds,
            scratch_string: String::new(),
        }
    }

    pub fn get_next(&mut self) -> Option<(Option<DateTime<Utc>>, String)> {
        let line = self.reader.next()?;
        let line = match line {
            Err(_) => return None,
            Ok(v) => v,
        };

        // TODO: work on uncolored line
        if self.kind.is_none() {
            // Try finding a successful timestamp format
            for kind in &*self.possible_kinds {
                if let Some(_) = kind.parse(&line, &mut self.scratch_string) {
                    // Found it
                    self.kind = Some(kind.clone());
                    break;
                }
            }
        }

        match &self.kind {
            Some(kind) => {
                let ts = kind.parse(&line, &mut self.scratch_string);
                Some((ts, format!("{}{}",
                    if ts.is_none() { &self.filler } else { &self.prefix },
                    line)))
            }
            None => {
                Some((None, format!("{}{}", &self.filler, line)))
            },
        }
    }
}

impl<F> Iterator for TimestampedStream<F>
    where F: Read
{
    type Item = (Option<DateTime<Utc>>, String);

    fn next(&mut self) -> Option<Self::Item> {
        self.get_next()
    }
}
