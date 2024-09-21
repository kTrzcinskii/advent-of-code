use std::{fmt::Display, fs, io::Read, path::Path};

use anyhow::Result;

pub trait Solution {
    fn calculate_solution_easy(&self, data: String) -> Result<impl Display>;
    fn calculate_solution_hard(&self, data: String) -> Result<impl Display>;
    /// Returns solution in the form that can be displayed in the terminal
    fn get_solution<P>(&self, path: P) -> Result<(Result<impl Display>, Result<impl Display>)>
    where
        P: AsRef<Path>,
    {
        let data = self.get_data(path)?;
        let easy = self.calculate_solution_easy(data.clone());
        let hard = self.calculate_solution_hard(data);
        Ok((easy, hard))
    }
    /// Loads data from file input
    fn get_data<P>(&self, path: P) -> Result<String>
    where
        P: AsRef<Path>,
    {
        let mut file = fs::File::open(path)?;
        let mut content = String::new();
        file.read_to_string(&mut content)?;
        Ok(content)
    }
}
