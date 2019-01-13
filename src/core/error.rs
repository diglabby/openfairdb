use pwhash;
use std::{error, io};

quick_error! {
    #[derive(Debug)]
    pub enum ParameterError {
        Title{
            description("The title is invalid")
        }
        Bbox{
            description("Bounding box is invalid")
        }
        License{
            description("Unsupported license")
        }
        Email{
            description("Invalid email address")
        }
        Phone{
            description("Invalid phone nr")
        }
        Url{
            description("Invalid URL")
        }
        Contact{
            description("Invalid contact")
        }
        RegistrationType{
            description("Invalid registraion type")
        }
        UserName{
            description("Invalid username")
        }
        UserExists{
            description("The user already exits")
        }
        Password{
            description("Invalid password")
        }
        EmptyComment{
            description("Empty comment")
        }
        RatingValue{
            description("Rating value out of range")
        }
        RatingContext(context: String){
            description("Invalid rating context")
        }
        Credentials {
            description("Invalid credentials")
        }
        EmailNotConfirmed {
            description("Email not confirmed")
        }
        Forbidden{
            description("This is not allowed")
        }
        Unauthorized{
            description("This is not allowed without auth")
        }
        EndDateBeforeStart{
            description("The end date is before the start")
        }
        OwnedTag{
            description("The tag is owned by an organization")
        }
        CreatorEmail{
            description("Missing the email of the creator")
        }
    }
}

quick_error! {
    #[derive(Debug)]
    //TODO: rename to GatewayError
    pub enum RepoError {
        NotFound{
            description("The requested object could not be found")
        }
        #[cfg(test)]
        AlreadyExists{
            description("The object already exists")
        }
        InvalidVersion{
            description("The version of the object is invalid")
        }
        Io(err: io::Error) {
            from()
            cause(err)
            description(err.description())
        }
        Other(err: Box<error::Error>){
            description(err.description())
        }
    }
}

quick_error! {
    #[derive(Debug)]
    pub enum Error {
        Parameter(err: ParameterError){
            from()
            cause(err)
            description(err.description())
        }
        Repo(err: RepoError){
            from()
            cause(err)
            description(err.description())
        }
        Pwhash(err: pwhash::error::Error){
            from()
            cause(err)
            description(err.description())
        }
        Internal(msg: String){
            from()
        }
    }
}
