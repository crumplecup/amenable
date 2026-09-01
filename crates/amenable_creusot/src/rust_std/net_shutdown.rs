#[cfg(creusot)]
use creusot_std::macros::{check, ensures, extern_spec, logic, requires, trusted};
#[cfg(creusot)]
use std::net::Shutdown;
amenable_derive::harness! {
    creusot, VERIFY_SHUTDOWN_WRITE_PREVENTS_FURTHER_WRITES_SRC, {
        /// `.shutdown(Shutdown::Write)` closes the write half, so a later
        /// write on that stream fails.
        #[trusted]
        #[requires(true)]
        #[ensures(result)]
        fn verify_shutdown_write_prevents_further_writes() -> bool {
            use std::io::Write;
            use std::net::{TcpListener, TcpStream};

            let listener = TcpListener::bind("127.0.0.1:0").unwrap();
            let addr = listener.local_addr().unwrap();
            let mut client = TcpStream::connect(addr).unwrap();
            let _server_side = listener.accept().unwrap();

            client.shutdown(Shutdown::Write).unwrap();
            client.write(b"more data").is_err()
        }
    }
}
