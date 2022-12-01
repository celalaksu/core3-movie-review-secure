use borsh::{BorshDeserialize};
use solana_program::{program_error::ProgramError};

// inside instruction.rs
pub enum MovieInstruction {
    AddMovieReview {
        title: String,
        rating: u8,
        description: String
    },
    UpdateMovieReview {
        title: String,
        rating: u8,
        description: String,
    },
}

// inside instruction.rs
impl MovieInstruction {
    pub fn unpack(input: &[u8]) -> Result<Self, ProgramError> {
        
		let (&variant, rest) = input.split_first().ok_or(ProgramError::InvalidInstructionData)?;
        
		let payload = MovieReviewPayload::try_from_slice(rest).unwrap();
        
        Ok(match variant {
            0 => Self::AddMovieReview {
                title: payload.title,
                rating: payload.rating,
                description: payload.description },
            1 => Self::UpdateMovieReview { 
                title: payload.title, 
                rating: payload.rating, description: 
                payload.description },
            _ => return Err(ProgramError::InvalidInstructionData)
        })
    }
}

#[derive(BorshDeserialize)]
struct MovieReviewPayload {
    title: String,
    rating: u8,
    description: String
}