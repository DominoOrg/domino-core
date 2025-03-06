use super::{error::DominoError, types::Puzzle};

pub fn get_n(puzzle: &Puzzle) -> Result<i32, DominoError> {
    if puzzle.len() == 0 {
        return Err(DominoError::InvalidLength);
    }
    let mut tmp: f32 = (-3.0 + (1.0 as f32 + 8.0 as f32 * puzzle.len() as f32).sqrt()) / 2.0;
    if tmp - tmp.floor() == 0.0 {
        return Ok(tmp as i32);
    }
    tmp = -1.0 + (2.0 as f32 * puzzle.len() as f32).sqrt();
    if tmp - tmp.floor() == 0.0 {
        return Ok(tmp as i32);
    }
    tmp = (-1.0 + (1.0 as f32 + 4.0 * puzzle.len() as f32).sqrt()) / 2.0;
    if tmp - tmp.floor() == 0.0 {
        return Ok(tmp as i32);
    }
    tmp = (puzzle.len() as f32).sqrt();
    if tmp - tmp.floor() == 0.0 {
        return Ok(tmp as i32);
    }
    Err(DominoError::InvalidLength)
}
