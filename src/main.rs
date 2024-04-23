//! A simple tool that intercept and ignore unlink and unlinkat syscalls.
//! Instead of deleting the file, it will print the path of the file that was attempted to be deleted.

use clap::Parser;
use reverie::syscalls::ReadAddr;
use reverie::syscalls::Syscall;
use reverie::Error;
use reverie::Guest;
use reverie::Tool;
use reverie_util::CommonToolArguments;

#[derive(Debug, Default, Clone)]
struct IgnoreUnlink {}

#[reverie::tool]
impl Tool for IgnoreUnlink {
    type GlobalState = ();
    type ThreadState = ();

    async fn handle_syscall_event<T: Guest<Self>>(
        &self,
        guest: &mut T,
        syscall: Syscall,
    ) -> Result<i64, Error> {
        match syscall {
            Syscall::Unlinkat(d) => {
                let path = d
                    .path()
                    .expect("failed to get path")
                    .read(&guest.memory())
                    .expect("failed to read path");

                eprintln!("Unlinkat: tried to remove file {:?}", path);
                Ok(0)
            }
            Syscall::Unlink(d) => {
                let path = d
                    .path()
                    .expect("failed to get path")
                    .read(&guest.memory())
                    .expect("failed to read path");

                eprintln!("Unlink: tried to remove file {:?}", path);
                Ok(0)
            }
            _ => guest.tail_inject(syscall).await,
        }
    }
}

#[tokio::main]
async fn main() -> Result<(), Error> {
    let args = CommonToolArguments::from_args();
    let tracer = reverie_ptrace::TracerBuilder::<IgnoreUnlink>::new(args.into())
        .spawn()
        .await?;
    let (status, _global_state) = tracer.wait().await?;
    status.raise_or_exit()
}
