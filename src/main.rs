#[derive(Debug)]
struct ErrorA;

#[derive(Debug)]
struct ErrorB;

#[derive(Debug)]
enum AppError {
    ErrorA,
    ErrorB,
}

impl From<AppError> for anyhow::Error {
    fn from(value: AppError) -> Self {
        let err = match value {
            AppError::ErrorA => anyhow::anyhow!("fuck: error_a"),
            AppError::ErrorB => anyhow::anyhow!("fuck: error_b"),
        };
        err
    }
}


impl std::error::Error for ErrorA {}
impl std::fmt::Display for ErrorA {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "ErrorA occured")
    }
}

impl std::error::Error for ErrorB {}
impl std::fmt::Display for ErrorB {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "ErrorB occured")
    }
}

fn return_a_a() -> Result<(), anyhow::Error> {
    Err(AppError::ErrorA.into())
}
fn return_b_b() -> Result<(), anyhow::Error> {
    Err(AppError::ErrorB.into())
}

// we don't even need to add the error type if we use
// the anyhow's Result enum 
fn with_anyhow() -> anyhow::Result<()> {
    let _ = return_b_b()?;
    let _ = return_a_a()?;
    Ok(())
}

fn main(){
    let res =  with_anyhow();
    println!("{:?}", res);
}
