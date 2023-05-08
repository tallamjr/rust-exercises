#[derive(Eq, PartialEq, Debug)]
pub enum Command {
    Publish(String),
    Retrieve,
    Command, // introduced only temporarely
}

#[derive(Eq, PartialEq, Debug)]
pub enum Error {
    TrailingData,
    IncompleteMessage,
    EmptyMessage,
    UnknownCommand,
    UnknownError,
    UnexpectedPayload,
    MissingPayload,
}


pub fn parse(input: &str) -> Result<Command, Error> {

    match input.split_once('\n') {

        Some((_,data)) => {
            if data.len() != 0 {
                Err(Error::TrailingData)
            } else {
                Ok(Command::Command)}
            },
        None => Err(Error::IncompleteMessage),
    }
}

#[test]
fn test_trailing_data() {
    let line = "PUBLISH The message\n is wrong \n";
    let result: Result<Command, Error> = parse(line);
    let expected = Err(Error::TrailingData);
    assert_eq!(result, expected);
    }

    #[test]
fn test_missing_nl() {
    let line = "PUBLISH";
    let result: Result<Command, Error> = parse(line);
    let expected = Err(Error::IncompleteMessage);
    assert_eq!(result, expected);
}   

#[test]
fn test_correct_nl() {
    let line = "PUBLISH \n";
    let result: Result<Command, Error> = parse(line);
    let expected = Ok(Command::Command);
    assert_eq!(result, expected);
}  

#[cfg(test)]
mod tests {
    use super::*;

    // Tests placement of \n
    #[test]
    fn test_missing_nl() {
        let line = "RETRIEVE";
        let result: Result<Command, Error> = parse(line);
        let expected = Err(Error::IncompleteMessage);
        assert_eq!(result, expected);
    }
    #[test]
    fn test_trailing_data() {
        let line = "PUBLISH The message\n is wrong \n";
        let result: Result<Command, Error> = parse(line);
        let expected = Err(Error::TrailingData);
        assert_eq!(result, expected);
    }

//     // Tests for empty messages and unknown commands

//     #[test]
//     fn test_only_nl() {
//         let line = "\n";
//         let result: Result<Command, Error> = parse(line);
//         let expected = Err(Error::EmptyMessage);
//         assert_eq!(result, expected);
//     }

//     #[test]
//     fn test_unknown_command() {
//         let line = "SERVE \n";
//         let result: Result<Command, Error> = parse(line);
//         let expected = Err(Error::UnknownCommand);
//         assert_eq!(result, expected);
//     }

//     // Tests correct formatting of RETRIEVE command

//     #[test]
//     fn test_retrieve_w_whitespace() {
//         let line = "RETRIEVE \n";
//         let result: Result<Command, Error> = parse(line);
//         let expected = Err(Error::UnexpectedPayload);
//         assert_eq!(result, expected);
//     }

//     #[test]
//     fn test_retrieve_payload() {
//         let line = "RETRIEVE this has a payload\n";
//         let result: Result<Command, Error> = parse(line);
//         let expected = Err(Error::UnexpectedPayload);
//         assert_eq!(result, expected);
//     }
   
//     #[test]
//     fn test_retrieve() {
//         let line = "RETRIEVE\n";
//         let result: Result<Command, Error> = parse(line);
//         let expected = Ok(Command::Retrieve);
//         assert_eq!(result, expected);
//     }

//     // Tests correct formatting of PUBLISH command

//     #[test]
//     fn test_publish() {
//         let line = "PUBLISH TestMessage\n";
//         let result: Result<Command, Error> = parse(line);
//         let expected = Ok(Command::Publish("TestMessage".into()));
//         assert_eq!(result, expected);
//     }

//     #[test]
//     fn test_empty_publish() {
//         let line = "PUBLISH \n";
//         let result: Result<Command, Error> = parse(line);
//         let expected = Ok(Command::Publish("".into()));
//         assert_eq!(result, expected);
//     }

//     #[test]
//     fn test_missing_payload() {
//         let line = "PUBLISH\n";
//         let result: Result<Command, Error> = parse(line);
//         let expected = Err(Error::MissingPayload);
//         assert_eq!(result, expected);
//     }
}

