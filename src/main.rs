use std::fmt;

#[derive(Debug)]
struct Week {
    attending: Vec<bool>,
}

impl fmt::Display for Week {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "[ ")?;
        for (i, attendance) in self.attending.iter().enumerate() {
            if *attendance {
                write!(f, "{} ", i)?;
            } else {
                write!(f, "x ")?;
            }
        }
        write!(f, "]")
    }
}

fn make_combinations_internal(
    weeks: &mut Vec<Week>,
    combination: &mut Vec<usize>,
    people: usize,
    offset: usize,
    k: usize,
) {
    if k == 0 {
        let mut attending = vec![true; people];
        for i in combination {
            attending[*i] = false;
        }
        weeks.push(Week { attending });
        return;
    }

    for i in offset..=(people - k) {
        combination.push(i);
        make_combinations_internal(weeks, combination, people, i + 1, k - 1);
        combination.pop();
    }
}

fn make_week_combinations(weeks: &mut Vec<Week>, people: usize, off: usize) {
    let mut combination = Vec::new();

    make_combinations_internal(weeks, &mut combination, people, 0, off);
}

fn person_off_in_only_one_week(a: &Week, b: &Week) -> bool {
    a.attending.iter().zip(b.attending.iter()).all(|(&a, &b)| a || b)
}

    // Rules
    // 1. No person off for 2 weeks in a row
    // 2. The people off in the last two weeks must play
    //   - for 7 people with 2 off, other rule may generalize better
    // 3. Use all combinations
fn sort_weeks(weeks: &mut [Week], offset: usize) -> Result<(), ()> {
    let (prev, next) = weeks.split_at(offset);

    if next.is_empty() {
        // All weeks have been sorted, test the cycle condition
        if person_off_in_only_one_week(&weeks[0], &weeks[weeks.len() - 1]) {
            return Ok(());
        } else {
            return Err(());
        }
    }

    //println!("Prev:");
    //for (i, week) in prev.iter().enumerate() {
    //    println!("Week {}: {}", i, week);
    //}

    // find all weeks that satisfy rule 1 and rule 2
    let filtered_weeks = if prev.len() > 1 {
        next.iter()
            .enumerate()
            .filter(|e| {
                //println!("Comparing week {} to week {}", offset + e.0, offset - 1);
                person_off_in_only_one_week(&prev[prev.len() - 2], e.1)
            })
            .filter(|e| {
                //println!("Comparing week {} to week {}", offset + e.0, offset);
                person_off_in_only_one_week(&prev[prev.len() - 1], e.1)
            })
            .map(|e| e.0)
            .collect::<Vec<usize>>()
    } else {
        next.iter()
            .enumerate()
            .filter(|e| {
                //println!("Comparing week {} to week {}", offset + e.0, offset);
                person_off_in_only_one_week(&prev[prev.len() - 1], e.1)
            })
            .map(|e| e.0)
            .collect::<Vec<usize>>()
    };

    //println!("{:?}", filtered_weeks.iter().map(|i| i + offset).collect::<Vec<_>>());

    for test_week in filtered_weeks {
        //println!("Swapping week {} for week {}", offset, offset + test_week);
        // swap current "next week" with first week in list
        weeks.swap(offset, offset + test_week);
        // sort remaining list
        if let Ok(_) = sort_weeks(weeks, offset + 1) {
            // if success, return ok
            return Ok(());
        }
        //println!("Swapping back week {} for week {}", offset, offset + test_week);
        // if failure, undo swap and continue iterating over list
        weeks.swap(offset, offset + test_week);
    }

    // All weeks tried, return error
    Err(())
}

fn main() {
    let num_people = 7;
    let num_off = 2;

    let mut weeks = Vec::new();
    make_week_combinations(&mut weeks, num_people, num_off);

    for (i, week) in weeks.iter().enumerate() {
        println!("Week {}: {}", i, week);
    }

    if let Err(_) = sort_weeks(&mut weeks, 1) {
        eprintln!("Failed to sort weeks following all the rules!")
    }

    for (i, week) in weeks.iter().enumerate() {
        println!("Week {}: {}", i, week);
    }
}
