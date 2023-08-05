#[macro_export]
macro_rules! log_error {
    ($result:expr, $msg:expr) => {
        match $result {
            Ok(val) => Ok(val),
            Err(err) => {
                log::error!("{}: {}", $msg, err);
                Err(err)
            }
        }
    };
    ($result:expr, $msg:expr, $status:expr) => {
        match $result {
            Ok(val) => Ok(val),
            Err(err) => {
                log::error!("{}: {} - Status: {}", $msg, err, $status);
                Err(err).map_err(|e| RocketError(e.into(), $status))
            }
        }
    };
}
