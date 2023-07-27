use std::fmt;

#[derive(Debug, Clone)]
pub struct SummationError {
    msg: String,
}

impl std::error::Error for SummationError {}
impl fmt::Display for SummationError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.msg)
    }
}
impl From<std::io::Error> for SummationError {
    fn from(e: std::io::Error) -> SummationError {
        SummationError {
            msg: format!("io::Error: {}", e),
        }
    }
}
impl From<std::num::ParseIntError> for SummationError {
    fn from(e: std::num::ParseIntError) -> SummationError {
        SummationError {
            msg: format!("ParseIntError: {}", e),
        }
    }
}

// TODO: takes a &std::path::Path and returns Result<i64, SummationError>. 
// In this implementation, you must pattern match each Result and handle it (by returning a Err(SummationError)). 
// In all error cases, we want the function to return an Err(e) where e is a SummationError. If we can successfully sum the value, we return Ok(sum).
pub fn sum_file_1(path: &std::path::Path) -> Result<i64, SummationError> {
    let file = std::fs::read_to_string(path);
    match file {
        Ok(file) => {
            let mut sum = 0;
            let lines: Vec<&str> = file.trim().split('\n').collect();
            for line in lines {
                let num = line.parse::<i64>();
                match num {
                    Ok(num) => sum += num,
                    Err(e) => return Err(SummationError::from(e)),
                }
            }
            Ok(sum)
        }
        Err(e) => Err(SummationError::from(e)),
    }
}

// TODO: sum_file_2 that has the same behaviour, but uses the ? operator to handle any Result values you get.
pub fn sum_file_2 (pa: &std::path::Path )-> Result<i64, SummationError>{
    let file = std::fs::read_to_string(pa)?;  
    let mut sum = 0;
    let lines: Vec<&str> = file.trim().split('\n').collect();
    for line in lines {
        let num = line.parse::<i64>()?;
        sum += num;
    }
    
    Ok(sum)
}
