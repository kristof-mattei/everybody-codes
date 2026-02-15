use std::path::Path;
use std::{env, fs};

use super::quest::Quest;

fn read_file_base(filepath: impl AsRef<Path>) -> String {
    let f = fs::read_to_string(&filepath);

    f.unwrap_or_else(|error| {
        panic!(
            "Error reading file \"{}\": {:?}",
            filepath.as_ref().to_str().unwrap(),
            error
        )
    })
}

/// Helper function that reads a text file to a string.
///
/// # Panics
///
/// if the file does not exist or cannot be read.
#[must_use]
pub fn read_file(folder: &str, quest: &Quest) -> String {
    let cwd = env::current_dir().unwrap();
    let filepath = cwd.join("data").join(folder).join(format!("{}.txt", quest));

    read_file_base(filepath)
}

/// Helper function that reads a text file to string, appending a part suffix. E.g. like `01-2.txt`.
///
/// # Panics
///
/// if the file does not exist or cannot be read.
#[must_use]
pub fn read_file_part(folder: &str, quest: &Quest, part: u8) -> String {
    let cwd = env::current_dir().unwrap();
    let filepath = cwd
        .join("data")
        .join(folder)
        .join(format!("{}-{}.txt", quest, part));

    read_file_base(filepath)
}

#[macro_export]
macro_rules! solution {
    () => {
        $crate::solution!(PartSolution::None, PartSolution::None);
    };
    ($solution_1:expr) => {
        $crate::solution!($solution_1, PartSolution::None, PartSolution::None);
    };
    ($solution_1:expr, $solution_2:expr) => {
        $crate::solution!($solution_1, $solution_2, PartSolution::None);
    };
    ($solution_1:expr, $solution_2:expr,  $solution_3:expr) => {
        /// The current quest.
        static QUEST: std::sync::LazyLock<$crate::quest::Quest> = std::sync::LazyLock::new(|| {
            use std::path::Path;

            let path = Path::new(file!());
            let file_stem = path
                .file_stem()
                .expect("No stem found")
                .to_str()
                .expect("Invalid str");

            std::str::FromStr::from_str(file_stem).expect("Could not convert input to Quest")
        });

        #[expect(clippy::disallowed_macros, reason = "No pretty needed here")]
        fn main() {
            let s = Solution {};

            let inputs = concat!(env!("CARGO_MANIFEST_DIR"), "/data/inputs");

            {
                let input1 = $crate::solution::read_file_part(inputs, &QUEST, 1);
                let part_1_expected_solution: PartSolution = PartSolution::from($solution_1);
                assert_eq!(part_1_expected_solution, s.part_1(&input1));
            }

            {
                let input2 = $crate::solution::read_file_part(inputs, &QUEST, 2);
                let part_2_expected_solution: PartSolution = PartSolution::from($solution_2);
                assert_eq!(part_2_expected_solution, s.part_2(&input2));
            }

            {
                let input3 = $crate::solution::read_file_part(inputs, &QUEST, 3);
                let part_3_expected_solution: PartSolution = PartSolution::from($solution_3);
                assert_eq!(part_3_expected_solution, s.part_3(&input3));
            }
        }

        pub struct Solution {}
    };
}
